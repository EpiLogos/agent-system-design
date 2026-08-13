import importlib.util
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PATH = ROOT / "contracts/factory/reference/decision.py"
spec = importlib.util.spec_from_file_location("factory_decision_contract", PATH)
decision = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = decision
spec.loader.exec_module(decision)


class DecisionContractTests(unittest.TestCase):
    def test_recognition_is_specialized_decision(self):
        store = decision.DecisionStore()
        recognition = store.open_recognition(
            "factory:decision:recognise-c1",
            candidate_ref="factory:candidate:c-1",
        )
        self.assertIsInstance(recognition, decision.Decision)
        self.assertEqual(decision.DecisionKind.RECOGNITION, recognition.kind)
        self.assertTrue(recognition.requires_human_authorship)

    def test_human_request_names_decision_and_authorship_reason(self):
        store = decision.DecisionStore()
        recognition = store.open_recognition(
            "factory:decision:recognise-c1",
            candidate_ref="factory:candidate:c-1",
        )
        request = store.request_human(
            "factory:human-request:hr-1",
            decision_ref=recognition.ref,
            channel="inbox",
        )
        self.assertEqual(recognition.ref, request.decision_ref)
        self.assertEqual(recognition.authorship_rationale, request.why_human)

    def test_reversible_engineering_cannot_require_approval_by_default(self):
        store = decision.DecisionStore()
        routine = store.open_decision(
            "factory:decision:implementation-choice",
            subject_ref="factory:artifact:a-1",
            question="Which internal helper name should be used?",
            options=("a", "b"),
        )
        with self.assertRaises(decision.DecisionError):
            store.request_human(
                "factory:human-request:hr-routine",
                decision_ref=routine.ref,
                channel="inbox",
            )

    def test_authorship_decision_requires_explicit_rationale(self):
        store = decision.DecisionStore()
        with self.assertRaises(decision.DecisionError):
            store.open_decision(
                "factory:decision:authorial",
                subject_ref="factory:project:factory",
                question="Which future belongs to the project?",
                options=("a", "b"),
                requires_human_authorship=True,
            )

    def test_decision_survives_request_transport_change(self):
        store = decision.DecisionStore()
        recognition = store.open_recognition(
            "factory:decision:recognise-c1",
            candidate_ref="factory:candidate:c-1",
        )
        request = store.request_human(
            "factory:human-request:hr-1",
            decision_ref=recognition.ref,
            channel="telegram",
        )
        projection = store.request_projection(request.ref)
        projection["channel"] = "cmux"
        recovered = store.recover_decision_from_request_projection(projection)
        self.assertEqual(recognition.ref, recovered.ref)

    def test_resolution_mutates_decision_not_request(self):
        store = decision.DecisionStore()
        recognition = store.open_recognition(
            "factory:decision:recognise-c1",
            candidate_ref="factory:candidate:c-1",
        )
        request = store.request_human(
            "factory:human-request:hr-1",
            decision_ref=recognition.ref,
            channel="inbox",
        )
        resolved = store.resolve(recognition.ref, "recognise")
        self.assertEqual(decision.DecisionState.RESOLVED, resolved.state)
        self.assertEqual("recognise", resolved.resolution)
        self.assertEqual(recognition.ref, store.requests[request.ref].decision_ref)


if __name__ == "__main__":
    unittest.main()
