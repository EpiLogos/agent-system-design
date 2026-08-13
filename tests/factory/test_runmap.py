import importlib.util
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PATH = ROOT / "contracts/factory/reference/runmap.py"
spec = importlib.util.spec_from_file_location("factory_runmap_contract", PATH)
runmap = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = runmap
spec.loader.exec_module(runmap)


class RunMapContractTests(unittest.TestCase):
    def make_store(self):
        store = runmap.ProjectRunStore()
        project = store.create_project("factory:project:factory", "Software Factory", ("repo:old",))
        run = store.open_run(
            project.ref,
            "factory:run:r-1",
            "factory:run-map:r-1",
            opening_condition="missing",
            destination="executable",
        )
        return store, project, run

    def test_project_and_run_identity_survive_operational_change(self):
        store, project, run = self.make_store()
        store.update_project_sources(project.ref, expected_revision=1, source_refs=("repo:new-a", "repo:new-b"))
        self.assertEqual(project.ref, store.identities.get(project.ref).ref)
        for session in ("session-a", "session-b"):
            resolved, resolved_map = store.resolve_projection(
                {
                    "canonicalRunRef": str(run.ref),
                    "canonicalRunMapRef": str(run.canonical_run_map_ref),
                    "session": session,
                }
            )
            self.assertEqual(run.ref, resolved.ref)
            self.assertEqual(run.canonical_run_map_ref, resolved_map.ref)

    def test_duplicate_map_and_projection_loss_are_rejected(self):
        store, _, run = self.make_store()
        with self.assertRaises(runmap.DuplicateCanonicalRunMap):
            store.attach_second_canonical_map(run.ref, "factory:run-map:other")
        with self.assertRaises(runmap.RunProjectionLoss):
            store.resolve_projection({"provider": "github", "externalId": "42"})

    def test_revision_conflict_is_rejected(self):
        store, _, run = self.make_store()
        authority = store.mutation_authority(run.ref)
        updated = store.add_node(authority, expected_revision=1, node_id="design", kind="artifact")
        self.assertEqual(2, updated.revision)
        with self.assertRaises(runmap.RunMapConflict):
            store.add_node(authority, expected_revision=1, node_id="build", kind="artifact")

    def test_invalid_topology_and_wrong_authority_are_rejected(self):
        store, _, run = self.make_store()
        authority = store.mutation_authority(run.ref)
        with self.assertRaises(runmap.InvalidTopologyMutation):
            store.add_edge(authority, expected_revision=1, source="opening", target="missing", relation="depends")
        wrong = runmap.RunMapAuthority(run.ref, runmap.Ref.parse("factory:run-map:not-canonical"))
        with self.assertRaises(runmap.RunMapConflict):
            store.add_node(wrong, expected_revision=1, node_id="x", kind="artifact")


if __name__ == "__main__":
    unittest.main()
