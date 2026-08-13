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
class ClosureDetermination: ref:str; subject_ref:str; subject_state:str; evidence_refs:tuple
@dataclass(frozen=True)
class GateDecision: ref:str; allowed:bool; transition:str; closure_ref:str|None

def resolve_plan(ref,requirement,subject_ref,subject_state,disposition,impact="none"):
 impact=AssuranceImpact(impact); high=requirement.risk=="high"; independent=impact is not AssuranceImpact.NONE
 return VerificationPlan(ref,subject_ref,subject_state,requirement.checks,high or independent,high or disposition.require_review,independent,"AIKit",disposition.source_ref)
