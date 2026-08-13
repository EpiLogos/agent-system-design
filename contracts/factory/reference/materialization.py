from dataclasses import dataclass
class MaterializationError(ValueError): pass
@dataclass(frozen=True)
class ExecutionDemand: ref:str; required:tuple; preferred:tuple=(); optional:tuple=()
@dataclass(frozen=True)
class CandidateMaterializationDemand(ExecutionDemand): candidate_ref:str=""
@dataclass(frozen=True)
class WorkcellOffer: workcell_ref:str; affordances:tuple
@dataclass(frozen=True)
class Binding: logical_ref:str; concrete_ref:str; provider:str
@dataclass(frozen=True)
class MaterializedExecutionWorld: ref:str; demand_ref:str; candidate_ref:str|None; workcell_ref:str; bindings:tuple

def materialize(ref,*,demand,offer,bindings):
 if hasattr(demand,"provider"): raise MaterializationError("ExecutionDemand must be provider-neutral")
 missing=set(demand.required)-set(offer.affordances)
 if missing: raise MaterializationError("required affordance unavailable")
 for binding in bindings:
  if binding.logical_ref.startswith("factory:binding:"): raise MaterializationError("Binding cannot replace canonical semantic identity")
 candidate_ref=demand.candidate_ref if isinstance(demand,CandidateMaterializationDemand) else None
 return MaterializedExecutionWorld(ref,demand.ref,candidate_ref,offer.workcell_ref,tuple(bindings))
