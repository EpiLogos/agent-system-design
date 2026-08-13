import importlib.util
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PATH = ROOT / "contracts/factory/reference/epistemic.py"
spec = importlib.util.spec_from_file_location("factory_epistemic_contract", PATH)
epistemic = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = epistemic
spec.loader.exec_module(epistemic)


class EpistemicContractTests(unittest.TestCase):
    def test_intended_observed_verified_are_distinct_relations(self):
        store = epistemic.EpistemicStore()
        subject = "factory:candidate:c-1"
        expected = {}
        for index, relation in enumerate(("intended", "observed", "verified"), start=1):
            claim = store.add_claim(
                f"factory:claim:c-{index}",
                subject_ref=subject,
                relation=relation,
                statement=f"statement {index}",
            )
            expected[relation] = store.standing(claim.ref)["relation"]
        self.assertEqual({"intended": "intended", "observed": "observed", "verified": "verified"}, expected)

    def test_confidence_does_not_promote_observed_claim(self):
        store = epistemic.EpistemicStore()
        claim = store.add_claim(
            "factory:claim:observed-1",
            subject_ref="factory:candidate:c-1",
            relation="observed",
            statement="candidate returned success",
            asserted_confidence=0.99,
        )
        evidence = store.add_evidence(
            "factory:evidence:e-1",
            claim_ref=claim.ref,
            purpose="observation",
            provenance={"source": "runtime", "producer": "factory:execution:e-1"},
            polarity="supports",
            content_ref="artifact:runtime-output",
        )
        store.add_assessment(
            "factory:assessment:a-1",
            claim_ref=claim.ref,
            evidence_refs=(evidence.ref,),
            conclusion="supports",
            rationale="output is internally consistent",
            confidence=1.0,
            producer_execution_ref="factory:execution:e-2",
        )
        self.assertEqual("observed", store.standing(claim.ref)["relation"])

    def test_contradictory_evidence_remains_representable(self):
        store = epistemic.EpistemicStore()
        claim = store.add_claim(
            "factory:claim:compatibility",
            subject_ref="factory:candidate:c-1",
            relation="verified",
            statement="candidate is compatible",
        )
        support = store.add_evidence(
            "factory:evidence:support",
            claim_ref=claim.ref,
            purpose="verification",
            provenance={"source": "suite-a", "producer": "factory:execution:e-a"},
            polarity="supports",
            content_ref="check:a",
        )
        challenge = store.add_evidence(
            "factory:evidence:challenge",
            claim_ref=claim.ref,
            purpose="verification",
            provenance={"source": "suite-b", "producer": "factory:execution:e-b"},
            polarity="challenges",
            content_ref="check:b",
        )
        standing = store.standing(claim.ref)
        self.assertEqual((support.ref,), standing["supports"])
        self.assertEqual((challenge.ref,), standing["challenges"])
        self.assertEqual("verified", standing["relation"])

    def test_verified_relation_without_evidence_has_no_manufactured_support(self):
        store = epistemic.EpistemicStore()
        claim = store.add_claim(
            "factory:claim:unproven-verification",
            subject_ref="factory:candidate:c-1",
            relation="verified",
            statement="verification was claimed",
            asserted_confidence=1.0,
        )
        standing = store.standing(claim.ref)
        self.assertEqual((), standing["supports"])
        self.assertEqual((), standing["assessments"])

    def test_evidence_requires_purpose_and_provenance(self):
        store = epistemic.EpistemicStore()
        claim = store.add_claim(
            "factory:claim:c-1",
            subject_ref="factory:artifact:a-1",
            relation="observed",
            statement="artifact exists",
        )
        with self.assertRaises(epistemic.EpistemicError):
            store.add_evidence(
                "factory:evidence:e-1",
                claim_ref=claim.ref,
                purpose="",
                provenance={"source": "check", "producer": "factory:execution:e-1"},
                polarity="supports",
                content_ref="check:1",
            )
        with self.assertRaises(epistemic.EpistemicError):
            store.add_evidence(
                "factory:evidence:e-2",
                claim_ref=claim.ref,
                purpose="existence",
                provenance={"source": "check"},
                polarity="supports",
                content_ref="check:2",
            )

    def test_assessment_requires_evidence_and_can_require_independent_execution(self):
        store = epistemic.EpistemicStore()
        claim = store.add_claim(
            "factory:claim:c-1",
            subject_ref="factory:artifact:a-1",
            relation="observed",
            statement="artifact exists",
        )
        evidence = store.add_evidence(
            "factory:evidence:e-1",
            claim_ref=claim.ref,
            purpose="existence",
            provenance={"source": "check", "producer": "factory:execution:builder"},
            polarity="supports",
            content_ref="check:1",
        )
        with self.assertRaises(epistemic.EpistemicError):
            store.add_assessment(
                "factory:assessment:a-empty",
                claim_ref=claim.ref,
                evidence_refs=(),
                conclusion="supports",
                rationale="no basis",
                confidence=0.9,
                producer_execution_ref="factory:execution:reviewer",
            )
        with self.assertRaises(epistemic.EpistemicError):
            store.add_assessment(
                "factory:assessment:a-same",
                claim_ref=claim.ref,
                evidence_refs=(evidence.ref,),
                conclusion="supports",
                rationale="looks right",
                confidence=0.9,
                producer_execution_ref="factory:execution:builder",
                independent_from="factory:execution:builder",
            )


if __name__ == "__main__":
    unittest.main()
