from dataclasses import dataclass
from enum import Enum
class VerificationContractError(ValueError): pass
class AssuranceImpact(str,Enum): NONE="none"; EVIDENCE="evidence"; CLOSURE="closure"; GATE="gate"
@dataclass(frozen=True)
class VerificationRequirement: ref:str; risk:str; checks:tuple
@dataclass(frozen=True)
class HumanDisposition: source_ref:str; require_review:bool=False
@dataclass(frozen=True)
class VerificationPlan: ref:str; subject_ref:str; subject_state:str; checks:tuple; assessment_required:bool; review_required:bool; independent:bool; resolver:str; disposition_source_ref:str
@dataclass(frozen=True)
class Check: ref:str; check_id:str; passed:bool; subject_ref:str; subject_state:str
@dataclass(frozen=True)
class EvidenceBinding: evidence_ref:str; check_ref:str; subject_ref:str; subject_state:str; current:bool
@dataclass(frozen=True)
class ClosureDetermination: ref:str; subject_ref:str; subject_state:str; evidence_refs:tuple; assessment_refs:tuple; review_refs:tuple
@dataclass(frozen=True)
class GateDecision: ref:str; allowed:bool; transition:str; closure_ref:str|None

def resolve_plan(ref,requirement,subject_ref,subject_state,disposition,impact="none"):
 impact=AssuranceImpact(impact); high=requirement.risk=="high"; independent=impact is not AssuranceImpact.NONE
 return VerificationPlan(ref,subject_ref,subject_state,requirement.checks,high or independent,high or disposition.require_review,independent,"AIKit",disposition.source_ref)

def bind_evidence(evidence_ref,check,current=True):
 return EvidenceBinding(evidence_ref,check.ref,check.subject_ref,check.subject_state,bool(current))

def close(ref,*,plan,opening_condition,checks,evidence,assessment_refs=(),review_refs=(),run_terminated=False):
 by_ref={c.ref:c for c in checks}; bindings={e.check_ref:e for e in evidence}
 for check_id in plan.checks:
  matches=[c for c in checks if c.check_id==check_id]
  if not matches or not matches[-1].passed: raise VerificationContractError("required Check did not pass")
  c=matches[-1]; e=bindings.get(c.ref)
  if e is None or not e.current or e.subject_ref!=plan.subject_ref or e.subject_state!=plan.subject_state: raise VerificationContractError("evidence is stale or mismatched")
 if plan.assessment_required and not assessment_refs: raise VerificationContractError("Assessment required")
 if plan.review_required and not review_refs: raise VerificationContractError("human Review required")
 if plan.independent and any(x==checks[-1].ref for x in assessment_refs): raise VerificationContractError("independent scrutiny required")
 return ClosureDetermination(ref,plan.subject_ref,plan.subject_state,tuple(e.evidence_ref for e in evidence),tuple(assessment_refs),tuple(review_refs))

def gate(ref,*,transition,closure=None,requires_closure=True):
 if requires_closure and closure is None: return GateDecision(ref,False,transition,None)
 return GateDecision(ref,True,transition,closure.ref if closure else None)
