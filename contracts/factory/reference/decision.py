"""Executable reference for Decision, Recognition and HumanRequest semantics."""
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


class DecisionError(ValueError):
    pass


class DecisionKind(str, Enum):
    CHOICE = "choice"
    RECOGNITION = "recognition"


class DecisionState(str, Enum):
    OPEN = "open"
    RESOLVED = "resolved"
    DEFERRED = "deferred"


@dataclass(frozen=True)
class Decision:
    ref: object
    subject_ref: object
    kind: DecisionKind
    question: str
    options: tuple[str, ...]
    requires_human_authorship: bool
    authorship_rationale: str | None
    state: DecisionState = DecisionState.OPEN
    resolution: str | None = None


@dataclass(frozen=True)
class HumanRequest:
    ref: object
    decision_ref: object
    why_human: str
    channel: str


class DecisionStore:
    def __init__(self):
        self.identities = IdentityStore()
        self.decisions = {}
        self.requests = {}

    @staticmethod
    def _ref(value, kind):
        ref = Ref.parse(value) if isinstance(value, str) else value
        if ref.kind != kind:
            raise DecisionError(f"expected {kind} Ref")
        return ref

    def open_decision(
        self,
        ref,
        *,
        subject_ref,
        question,
        options,
        kind="choice",
        requires_human_authorship=False,
        authorship_rationale=None,
    ):
        ref = self._ref(ref, "decision")
        subject_ref = Ref.parse(subject_ref) if isinstance(subject_ref, str) else subject_ref
        try:
            kind = DecisionKind(kind)
        except ValueError as error:
            raise DecisionError("invalid Decision kind") from error
        if not isinstance(question, str) or not question.strip():
            raise DecisionError("Decision question is required")
        options = tuple(str(item).strip() for item in options if str(item).strip())
        if not options:
            raise DecisionError("Decision options are required")
        if requires_human_authorship and (not isinstance(authorship_rationale, str) or not authorship_rationale.strip()):
            raise DecisionError("human authorship requires an explicit rationale")
        decision = Decision(
            ref,
            subject_ref,
            kind,
            question.strip(),
            options,
            bool(requires_human_authorship),
            authorship_rationale.strip() if isinstance(authorship_rationale, str) else None,
        )
        self.identities.create(ref, {"kind": kind.value, "subjectRef": str(subject_ref), "question": decision.question})
        self.decisions[ref] = decision
        return decision

    def open_recognition(self, ref, *, candidate_ref, options=("recognise", "return", "discuss", "defer"), why_human="Recognition determines whether the encountered result belongs to the intended Project."):
        return self.open_decision(
            ref,
            subject_ref=candidate_ref,
            question="Should this encountered Candidate be recognised into the Project's durable reality?",
            options=options,
            kind="recognition",
            requires_human_authorship=True,
            authorship_rationale=why_human,
        )

    def request_human(self, ref, *, decision_ref, channel):
        ref = self._ref(ref, "human-request")
        decision_ref = self._ref(decision_ref, "decision")
        decision = self.decisions.get(decision_ref)
        if decision is None:
            raise DecisionError("HumanRequest must name an existing Decision")
        if not decision.requires_human_authorship:
            raise DecisionError("reversible/non-authorial Decision cannot require human approval by default")
        if decision.state is not DecisionState.OPEN:
            raise DecisionError("HumanRequest cannot target a non-open Decision")
        if not isinstance(channel, str) or not channel.strip():
            raise DecisionError("HumanRequest channel is required")
        request = HumanRequest(ref, decision_ref, decision.authorship_rationale, channel.strip())
        self.identities.create(ref, {"decisionRef": str(decision_ref), "whyHuman": request.why_human, "channel": request.channel})
        self.requests[ref] = request
        return request

    def resolve(self, decision_ref, resolution):
        decision_ref = self._ref(decision_ref, "decision")
        decision = self.decisions.get(decision_ref)
        if decision is None:
            raise DecisionError("unknown Decision")
        if decision.state is not DecisionState.OPEN:
            raise DecisionError("Decision is not open")
        if resolution not in decision.options:
            raise DecisionError("resolution must be one of the Decision options")
        current_revision = self.identities.get(decision_ref).revision
        resolved = Decision(
            decision.ref,
            decision.subject_ref,
            decision.kind,
            decision.question,
            decision.options,
            decision.requires_human_authorship,
            decision.authorship_rationale,
            DecisionState.RESOLVED,
            resolution,
        )
        self.identities.update(decision_ref, expected_revision=current_revision, payload={"kind": decision.kind.value, "state": "resolved", "resolution": resolution})
        self.decisions[decision_ref] = resolved
        return resolved

    def request_projection(self, request_ref):
        request_ref = self._ref(request_ref, "human-request")
        request = self.requests[request_ref]
        return {"canonicalDecisionRef": str(request.decision_ref), "channel": request.channel, "whyHuman": request.why_human}

    def recover_decision_from_request_projection(self, projection):
        value = projection.get("canonicalDecisionRef")
        if not isinstance(value, str):
            raise DecisionError("request projection cannot recover Decision without canonicalDecisionRef")
        ref = self._ref(value, "decision")
        if ref not in self.decisions:
            raise DecisionError("unknown Decision")
        return self.decisions[ref]
