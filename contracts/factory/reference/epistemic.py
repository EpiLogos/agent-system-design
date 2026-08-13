"""Executable Claim, Evidence and Assessment reference contract."""
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
import importlib.util
from pathlib import Path
import sys

_NAME = "factory_identity_contract"
if _NAME not in sys.modules:
    spec = importlib.util.spec_from_file_location(_NAME, Path(__file__).with_name("identity.py"))
    module = importlib.util.module_from_spec(spec)
    sys.modules[_NAME] = module
    spec.loader.exec_module(module)
identity = sys.modules[_NAME]
Ref = identity.Ref
IdentityStore = identity.IdentityStore


class EpistemicError(ValueError):
    pass


class ClaimRelation(str, Enum):
    INTENDED = "intended"
    OBSERVED = "observed"
    VERIFIED = "verified"


class EvidencePolarity(str, Enum):
    SUPPORTS = "supports"
    CHALLENGES = "challenges"
    CONTEXTUALIZES = "contextualizes"


class AssessmentConclusion(str, Enum):
    SUPPORTS = "supports"
    CHALLENGES = "challenges"
    INDETERMINATE = "indeterminate"


@dataclass(frozen=True)
class Claim:
    ref: object
    subject_ref: object
    relation: ClaimRelation
    statement: str
    asserted_confidence: float | None = None


@dataclass(frozen=True)
class Evidence:
    ref: object
    claim_ref: object
    purpose: str
    provenance: dict
    polarity: EvidencePolarity
    content_ref: str


@dataclass(frozen=True)
class Assessment:
    ref: object
    claim_ref: object
    evidence_refs: tuple
    conclusion: AssessmentConclusion
    rationale: str
    confidence: float
    producer_execution_ref: object


class EpistemicStore:
    def __init__(self):
        self.identities = IdentityStore()
        self.claims = {}
        self.evidence = {}
        self.assessments = {}

    @staticmethod
    def _ref(value, kind):
        ref = Ref.parse(value) if isinstance(value, str) else value
        if ref.kind != kind:
            raise EpistemicError(f"expected {kind} Ref")
        return ref

    @staticmethod
    def _confidence(value):
        if value is None:
            return None
        if isinstance(value, bool) or not isinstance(value, (int, float)) or not 0 <= float(value) <= 1:
            raise EpistemicError("confidence must be between 0 and 1")
        return float(value)

    def add_claim(self, ref, *, subject_ref, relation, statement, asserted_confidence=None):
        ref = self._ref(ref, "claim")
        subject_ref = Ref.parse(subject_ref) if isinstance(subject_ref, str) else subject_ref
        try:
            relation = ClaimRelation(relation)
        except ValueError as error:
            raise EpistemicError("invalid Claim relation") from error
        if not isinstance(statement, str) or not statement.strip():
            raise EpistemicError("Claim statement is required")
        confidence = self._confidence(asserted_confidence)
        claim = Claim(ref, subject_ref, relation, statement.strip(), confidence)
        self.identities.create(ref, {"subjectRef": str(subject_ref), "relation": relation.value})
        self.claims[ref] = claim
        return claim

    def add_evidence(self, ref, *, claim_ref, purpose, provenance, polarity, content_ref):
        ref = self._ref(ref, "evidence")
        claim_ref = self._ref(claim_ref, "claim")
        if claim_ref not in self.claims:
            raise EpistemicError("Evidence must name an existing Claim")
        if not isinstance(purpose, str) or not purpose.strip():
            raise EpistemicError("Evidence purpose is required")
        if not isinstance(provenance, dict) or not provenance.get("source") or not provenance.get("producer"):
            raise EpistemicError("Evidence provenance requires source and producer")
        if not isinstance(content_ref, str) or not content_ref.strip():
            raise EpistemicError("Evidence content ref is required")
        try:
            polarity = EvidencePolarity(polarity)
        except ValueError as error:
            raise EpistemicError("invalid Evidence polarity") from error
        evidence = Evidence(ref, claim_ref, purpose.strip(), dict(provenance), polarity, content_ref.strip())
        self.identities.create(ref, {"claimRef": str(claim_ref), "purpose": evidence.purpose, "provenance": evidence.provenance, "polarity": polarity.value})
        self.evidence[ref] = evidence
        return evidence

    def add_assessment(self, ref, *, claim_ref, evidence_refs, conclusion, rationale, confidence, producer_execution_ref, independent_from=None):
        ref = self._ref(ref, "assessment")
        claim_ref = self._ref(claim_ref, "claim")
        producer_execution_ref = self._ref(producer_execution_ref, "execution")
        if claim_ref not in self.claims:
            raise EpistemicError("Assessment must name an existing Claim")
        refs = tuple(self._ref(value, "evidence") for value in evidence_refs)
        if not refs or any(value not in self.evidence or self.evidence[value].claim_ref != claim_ref for value in refs):
            raise EpistemicError("Assessment must interpret Evidence for the Claim")
        if not isinstance(rationale, str) or not rationale.strip():
            raise EpistemicError("Assessment rationale is required")
        try:
            conclusion = AssessmentConclusion(conclusion)
        except ValueError as error:
            raise EpistemicError("invalid Assessment conclusion") from error
        confidence = self._confidence(confidence)
        if independent_from is not None and producer_execution_ref == self._ref(independent_from, "execution"):
            raise EpistemicError("independent Assessment cannot reuse producer Execution")
        assessment = Assessment(ref, claim_ref, refs, conclusion, rationale.strip(), confidence, producer_execution_ref)
        self.identities.create(ref, {"claimRef": str(claim_ref), "evidenceRefs": [str(value) for value in refs], "conclusion": conclusion.value, "confidence": confidence, "producerExecutionRef": str(producer_execution_ref)})
        self.assessments[ref] = assessment
        return assessment

    def standing(self, claim_ref):
        claim_ref = self._ref(claim_ref, "claim")
        claim = self.claims[claim_ref]
        items = [item for item in self.evidence.values() if item.claim_ref == claim_ref]
        return {
            "relation": claim.relation.value,
            "supports": tuple(item.ref for item in items if item.polarity is EvidencePolarity.SUPPORTS),
            "challenges": tuple(item.ref for item in items if item.polarity is EvidencePolarity.CHALLENGES),
            "contextualizes": tuple(item.ref for item in items if item.polarity is EvidencePolarity.CONTEXTUALIZES),
            "assessments": tuple(item.ref for item in self.assessments.values() if item.claim_ref == claim_ref),
        }
