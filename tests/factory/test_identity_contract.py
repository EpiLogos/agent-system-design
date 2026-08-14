import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "factory_identity_contract_direct",
    ROOT / "contracts/factory/reference/identity.py",
)
identity = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = identity
SPEC.loader.exec_module(identity)
FIXTURES = json.loads((ROOT / "contracts/factory/fixtures/ref-cases.json").read_text(encoding="utf-8"))
SCHEMA = json.loads((ROOT / "contracts/factory/ref.schema.json").read_text(encoding="utf-8"))


class IdentityContractTests(unittest.TestCase):
    def make_store(self):
        store = identity.IdentityStore()
        record = store.create(FIXTURES["canonicalRef"], {"name": "Epi-Logos"}, FIXTURES["aliases"])
        return store, record

    def test_ref_round_trip_and_schema_are_canonical(self):
        ref = identity.Ref.parse(FIXTURES["canonicalRef"])
        self.assertEqual(FIXTURES["canonicalRef"], str(ref))
        self.assertEqual("https://json-schema.org/draft/2020-12/schema", SCHEMA["$schema"])
        self.assertEqual("factory.identity-store/v1", SCHEMA["properties"]["schemaVersion"]["const"])
        self.assertEqual(1, SCHEMA["$defs"]["identityRecord"]["properties"]["revision"]["minimum"])

    def test_provider_path_and_projection_change_do_not_change_identity(self):
        store, record = self.make_store()
        for projection in FIXTURES["projectionChanges"]:
            self.assertEqual(record.ref, store.resolve_projection(projection).ref)
        moved = {"provider": "other", "path": "/different/place", "canonicalRef": str(record.ref)}
        self.assertEqual(record.ref, store.resolve_projection(moved).ref)

    def test_stale_write_is_rejected_without_mutation(self):
        store, record = self.make_store()
        current = store.update(record.ref, expected_revision=1, payload={"name": "Epi-Logos", "v": 2})
        before = store.to_json()
        with self.assertRaises(identity.StaleRevision):
            store.update(current.ref, expected_revision=1, payload={"name": "stale"})
        self.assertEqual(before, store.to_json())

    def test_alias_revision_and_alias_ownership_are_preserved(self):
        store, record = self.make_store()
        updated = store.add_alias(record.ref, "project:current-epi", expected_revision=1)
        self.assertEqual(2, updated.revision)
        self.assertEqual(updated.ref, store.resolve_ref("project:current-epi"))
        other = store.create("factory:project:other", {})
        with self.assertRaises(identity.IdentityConflict):
            store.add_alias(other.ref, "project:current-epi", expected_revision=1)

    def test_tombstone_is_terminal_and_ref_is_never_reused(self):
        store, record = self.make_store()
        tombstoned = store.tombstone(record.ref, expected_revision=1, reason="retired")
        self.assertTrue(tombstoned.tombstoned)
        self.assertEqual(2, tombstoned.revision)
        self.assertEqual(record.ref, store.get(record.ref).ref)
        with self.assertRaises(identity.TombstonedIdentity):
            store.update(record.ref, expected_revision=2, payload={})
        with self.assertRaises(identity.IdentityConflict):
            store.create(record.ref, {"replacement": True})

    def test_serialization_round_trip_preserves_revision_aliases_and_retirement(self):
        store, record = self.make_store()
        record = store.add_alias(record.ref, "project:current-epi", expected_revision=1)
        store.tombstone(record.ref, expected_revision=2, reason="retired")
        encoded = store.to_json()
        restored = identity.IdentityStore.from_json(encoded)
        self.assertEqual(encoded, restored.to_json())
        self.assertEqual(record.ref, restored.resolve_ref("project:current-epi"))

    def test_projection_loss_is_rejected_instead_of_inferred(self):
        store, _ = self.make_store()
        projection = dict(FIXTURES["projectionLoss"])
        projection.pop("canonicalRefMissing", None)
        with self.assertRaises(identity.ProjectionIdentityLoss):
            store.resolve_projection(projection)
        with self.assertRaises(identity.ProjectionIdentityLoss):
            store.resolve_projection({"provider": "github", "externalId": "repo-100", "canonicalRef": "factory:project:unknown"})

    def test_invalid_ref_and_serialized_retirement_mismatch_fail(self):
        with self.assertRaises(identity.InvalidRef):
            identity.Ref.parse("github:EpiLogos/agent-system-design")
        store, record = self.make_store()
        raw = store.to_dict()
        raw["retiredRefs"] = [str(record.ref)]
        with self.assertRaises(identity.IdentityError):
            identity.IdentityStore.from_dict(raw)


if __name__ == "__main__":
    unittest.main()
