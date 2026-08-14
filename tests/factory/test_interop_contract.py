import importlib.util
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "factory_interop_contract",
    ROOT / "contracts/factory/reference/interop.py",
)
interop = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = interop
SPEC.loader.exec_module(interop)


class InteropContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema, cls.document = interop.load_default(ROOT)

    def test_schema_is_versioned_and_every_instance_field_has_one_owner(self):
        self.assertEqual("https://json-schema.org/draft/2020-12/schema", self.schema["$schema"])
        self.assertEqual([], interop.schema_owner_failures(self.schema))

    def test_complete_positive_corpus_validates(self):
        interop.validate_fixture_document(self.document, self.schema)

    def test_round_trip_preserves_complete_contract(self):
        result = interop.round_trip(self.document, self.schema)
        self.assertEqual(self.document, result)

    def test_contract_surface_matches_live_cr001(self):
        expected = {
            "contractVersion", "identityEnvelope", "actionDescriptor", "capabilityDescriptor",
            "projectBinding", "runEnvelope", "executionEnvelope", "evidenceEnvelope",
            "assessmentEnvelope", "closureEnvelope", "gateDecisionEnvelope", "executionDemand",
            "workcellOffer", "binding", "generationProvenance", "projectionProvenance",
            "qlComposition",
        }
        self.assertEqual(expected, set(self.document["contract"]))
        self.assertEqual(expected, set(self.schema["required"]))

    def test_execution_demand_is_provider_neutral_but_binding_is_material(self):
        demand = self.document["contract"]["executionDemand"]
        self.assertFalse(any("provider" in key.lower() for key in demand))
        binding = self.document["contract"]["binding"]
        self.assertTrue(binding["bindingId"].startswith("binding:"))
        self.assertIn("providerRef", binding)

    def test_project_and_projection_providers_never_become_project_identity(self):
        contract = self.document["contract"]
        project_ref = contract["projectBinding"]["projectRef"]
        self.assertTrue(project_ref.startswith("factory:project:"))
        self.assertTrue(all(p["canonicalRef"] == project_ref for p in contract["projectionProvenance"]))
        self.assertGreater(len({p["providerRef"] for p in contract["projectionProvenance"]}), 1)

    def test_action_capability_agent_session_binding_and_provider_negative_cases_all_reject(self):
        for item in self.document["antiFixtures"]:
            with self.subTest(item=item["id"]):
                with self.assertRaises(interop.InteropError):
                    interop.reject_anti_fixture(item)

    def test_stale_subject_state_cannot_warrant_current_closure(self):
        contract = self.document["contract"]
        current = contract["identityEnvelope"]["subjectState"]
        for key in ("evidenceEnvelope", "assessmentEnvelope", "closureEnvelope", "gateDecisionEnvelope"):
            self.assertEqual(interop._subject_tuple(current), interop._subject_tuple(contract[key]["subjectState"]))
        stale = next(item for item in self.document["antiFixtures"] if item["id"] == "stale-subject-state-evidence")
        with self.assertRaises(interop.InteropError):
            interop.reject_anti_fixture(stale)

    def test_ql_fields_are_composed_without_factory_redefinition(self):
        ql = self.document["contract"]["qlComposition"]
        self.assertEqual("factory:claim:c-1", ql["targetRef"])
        ql_schema = self.schema["$defs"]["qlComposition"]["properties"]
        for key in ("qlFormRef", "qlAddress", "lensRef", "qlTarget"):
            self.assertEqual("Standalone QL/MEF module", ql_schema[key]["x-semantic-owner"])


if __name__ == "__main__":
    unittest.main()
