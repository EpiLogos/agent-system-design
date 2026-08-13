from dataclasses import dataclass
class CandidateContractError(ValueError): pass
@dataclass(frozen=True)
class Artifact: ref:str; artifact_type:str; content_ref:str
@dataclass(frozen=True)
class Materialization: provider:str; environment_ref:str; branch_ref:str|None=None
@dataclass(frozen=True)
class Candidate: ref:str; project_ref:str; artifact_refs:tuple; materializations:tuple=()
@dataclass(frozen=True)
class RecursionArtifact: ref:str; run_ref:str; authorization_decision_ref:str; authorised_effects:tuple
class CandidateStore:
 def __init__(self): self.artifacts={}; self.candidates={}; self.recursions={}
 def add_artifact(self,item): self.artifacts[item.ref]=item; return item
 def create_candidate(self,item):
  if not item.ref.startswith("factory:candidate:") or any(r not in self.artifacts for r in item.artifact_refs): raise CandidateContractError("invalid Candidate")
  self.candidates[item.ref]=item; return item
 def materialize(self,candidate_ref,*,provider,environment_ref,branch_ref=None):
  c=self.candidates[candidate_ref]; updated=Candidate(c.ref,c.project_ref,c.artifact_refs,c.materializations+(Materialization(provider,environment_ref,branch_ref),)); self.candidates[c.ref]=updated; return updated
 def recursion(self,item):
  if not item.authorization_decision_ref.startswith("factory:decision:") or not item.authorised_effects: raise CandidateContractError("explicit Decision and retained effects required")
  self.recursions[item.ref]=item; return item
 def route_effect(self,recursion_ref,effect):
  item=self.recursions[recursion_ref]
  if effect not in item.authorised_effects: raise CandidateContractError("effect was not authorised")
  return effect
