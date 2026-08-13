import copy
import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/validate_factory_authority.py"
spec = importlib.util.spec_from_file_location("authority", SCRIPT)
authority = importlib.util.module_from_spec(spec)
spec.loader.exec_module(authority)

IDENTITY = ROOT / "contracts/factory/reference/identity.py"
identity_spec = importlib.util.spec_from_file_location("factory_identity", IDENTITY)
identity = importlib.util.module_from_spec(identity_spec)
sys.modules[identity_spec.name] = identity
identity_spec.loader.exec_module(identity)


class AuthorityManifestTests(unittest.TestCase):
    def setUp(self):
        self.data = authority.load_manifest(ROOT / "contracts/factory/authority-manifest.json")

    def test_manifest_is_valid(self):
        self.assertEqual([], authority.validate_manifest(self.data))

    def test_provenance_round_trip_is_lossless(self):
        encoded = json.dumps(self.data, sort_keys=True)
        self.assertEqual(self.data, json.loads(encoded))

    def test_rejects_research_or_reference_promotion(self):
        mutated = copy.deepcopy(self.data)
        promoted = copy.deepcopy(mutated["retrievedReferences"][0])
        promoted.update({"precedence": 99, "scope": "bad", "governs": True})
        mutated["sources"].append(promoted)
        errors = authority.validate_manifest(mutated)
        self.assertTrue(any("silent authority promotion" in error for error in errors))

    def test_rejects_duplicate_source_identity(self):
        mutated = copy.deepcopy(self.data)
        mutated["sources"].append(copy.deepcopy(mutated["sources"][0]))
        errors = authority.validate_manifest(mutated)
        self.assertIn("duplicate source id", errors)
        self.assertIn("duplicate source path", errors)

    def test_index_has_unique_highest_precedence(self):
        mutated = copy.deepcopy(self.data)
        mutated["sources"][1]["precedence"] = 100
        self.assertIn("constitutional index must have unique highest precedence", authority.validate_manifest(mutated))

    def test_determination_statuses_are_exact(self):
        mutated = copy.deepcopy(self.data)
        mutated["determinationStatuses"].append("CONSENSUS")
        self.assertIn("determination status set differs from root programme", authority.validate_manifest(mutated))

    def test_generated_markdown_is_derived_from_manifest(self):
        rendered = authority.render_markdown(self.data)
        self.assertIn("QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md", rendered)
        self.assertIn("RUN-CLOSURE-VERIFICATION-ALIGNMENT.md", rendered)
        self.assertIn("Generated from `contracts/factory/authority-manifest.json`", rendered)


class IdentityContractTests(unittest.TestCase):
    def test_ref_round_trip_is_canonical(self):
        ref = identity.Ref.make("project", "epi-logos")
        self.assertEqual("factory:project:epi-logos", str(ref))
        self.assertEqual(ref, identity.Ref.parse(str(ref)))

    def test_identity_survives_provider_projection_change(self):
        fixtures = json.loads((ROOT / "contracts/factory/fixtures/ref-cases.json").read_text())
        store = identity.IdentityStore()
        created = store.create(fixtures["canonicalRef"], {"name": "Epi-Logos"})
        for projection in fixtures["projectionChanges"]:
            self.assertEqual(created.ref, store.resolve_projection(projection).ref)

    def test_projection_loss_does_not_infer_identity(self):
        store = identity.IdentityStore()
        store.create("factory:project:epi-logos", {})
        with self.assertRaises(identity.ProjectionIdentityLoss):
            store.resolve_projection({"provider": "github", "externalId": "repo-100"})

    def test_stale_write_is_rejected(self):
        store = identity.IdentityStore()
        store.create("factory:run:run-1", {"state": "opened"})
        store.update("factory:run:run-1", expected_revision=1, payload={"state": "active"})
        with self.assertRaises(identity.StaleRevision):
            store.update("factory:run:run-1", expected_revision=1, payload={"state": "stale"})
        self.assertEqual({"state": "active"}, store.get("factory:run:run-1").payload)

    def test_alias_change_advances_revision_without_changing_ref(self):
        store = identity.IdentityStore()
        created = store.create("factory:project:epi-logos", {}, aliases=["project:legacy"])
        updated = store.add_alias(created.ref, "project:current", expected_revision=1)
        self.assertEqual(created.ref, updated.ref)
        self.assertEqual(2, updated.revision)
        self.assertEqual(created.ref, store.get("project:legacy").ref)
        self.assertEqual(created.ref, store.get("project:current").ref)

    def test_alias_cannot_be_reassigned(self):
        store = identity.IdentityStore()
        store.create("factory:project:one", {}, aliases=["project:shared"])
        with self.assertRaises(identity.IdentityConflict):
            store.create("factory:project:two", {}, aliases=["project:shared"])

    def test_tombstone_preserves_identity_and_forbids_reuse(self):
        store = identity.IdentityStore()
        created = store.create("factory:candidate:c-1", {}, aliases=["candidate:legacy"])
        dead = store.tombstone(created.ref, expected_revision=1, reason="superseded")
        self.assertEqual(created.ref, dead.ref)
        self.assertTrue(dead.tombstoned)
        self.assertEqual(created.ref, store.get("candidate:legacy").ref)
        with self.assertRaises(identity.TombstonedIdentity):
            store.update(created.ref, expected_revision=2, payload={})
        with self.assertRaises(identity.IdentityConflict):
            store.create(created.ref, {})

    def test_serialization_round_trip_preserves_retirement(self):
        store = identity.IdentityStore()
        store.create("factory:project:one", {"path": "old"}, aliases=["project:one"])
        store.update("project:one", expected_revision=1, payload={"path": "new"})
        store.tombstone("project:one", expected_revision=2, reason="retired")
        restored = identity.IdentityStore.from_json(store.to_json())
        self.assertEqual(store.to_dict(), restored.to_dict())
        with self.assertRaises(identity.IdentityConflict):
            restored.create("factory:project:one", {})

    def test_external_path_and_provider_fields_do_not_change_ref(self):
        store = identity.IdentityStore()
        record = store.create("factory:artifact:a-1", {})
        first = store.resolve_projection({"canonicalRef": str(record.ref), "provider": "github", "path": "old"})
        second = store.resolve_projection({"canonicalRef": str(record.ref), "provider": "local", "path": "new"})
        self.assertEqual(first.ref, second.ref)
        self.assertEqual(record.revision, second.revision)


if __name__ == "__main__":
    unittest.main()
