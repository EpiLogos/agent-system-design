from dataclasses import dataclass,replace
from enum import Enum
import importlib.util
from pathlib import Path
import sys
_NAME="factory_identity_contract"
if _NAME not in sys.modules:
    s=importlib.util.spec_from_file_location(_NAME,Path(__file__).with_name("identity.py")); m=importlib.util.module_from_spec(s); sys.modules[_NAME]=m; s.loader.exec_module(m)
identity=sys.modules[_NAME]; Ref=identity.Ref; IdentityStore=identity.IdentityStore
class ContextContractError(ValueError): pass
class LoadState(str,Enum): AVAILABLE="available"; RETRIEVED="retrieved"; LOADED="loaded"
@dataclass(frozen=True)
class ContextResource: ref:object; source_ref:object; provider:str; state:LoadState; freshness:str; authoritative:bool=False
@dataclass(frozen=True)
class ProjectMapEntry: ref:object; target_ref:object; facet:str; provider:str|None=None
@dataclass(frozen=True)
class ProjectMap: ref:object; project_ref:object; entries:tuple
@dataclass(frozen=True)
class Context: ref:object; project_ref:object; operative_world:tuple; information_horizon:tuple; focus:tuple
@dataclass(frozen=True)
class ContextResolution: ref:object; context_ref:object; resolver:str; generation:str; resource_refs:tuple
class ContextStore:
    def __init__(self): self.identities=IdentityStore(); self.resources={}; self.maps={}; self.contexts={}; self.resolutions={}
    @staticmethod
    def _ref(value,kind=None):
        ref=Ref.parse(value) if isinstance(value,str) else value
        if kind and ref.kind!=kind: raise ContextContractError(f"expected {kind} Ref")
        return ref
    def add_resource(self,ref,*,source_ref,provider,freshness,authoritative=False):
        ref=self._ref(ref,"context-resource"); source_ref=self._ref(source_ref)
        item=ContextResource(ref,source_ref,provider,LoadState.AVAILABLE,freshness,bool(authoritative)); self.identities.create(ref,{"sourceRef":str(source_ref),"provider":provider}); self.resources[ref]=item; return item
    def transition_resource(self,ref,to_state):
        ref=self._ref(ref,"context-resource"); current=self.resources[ref]; target=LoadState(to_state); allowed={LoadState.AVAILABLE:LoadState.RETRIEVED,LoadState.RETRIEVED:LoadState.LOADED}
        if allowed.get(current.state)!=target: raise ContextContractError("invalid ContextLoad transition")
        item=replace(current,state=target); self.resources[ref]=item; return item
    def project_map(self,ref,*,project_ref,entries):
        ref=self._ref(ref,"project-map"); project_ref=self._ref(project_ref,"project"); normalized=tuple(ProjectMapEntry(self._ref(a,"project-map-entry"),self._ref(b),c,d) for a,b,c,d in entries); item=ProjectMap(ref,project_ref,normalized); self.identities.create(ref,{"projectRef":str(project_ref)}); self.maps[ref]=item; return item
    def create_context(self,ref,*,project_ref,operative_world,information_horizon,focus):
        ref=self._ref(ref,"context"); project_ref=self._ref(project_ref,"project"); horizon=tuple(self._ref(v,"context-resource") for v in information_horizon)
        if any(v not in self.resources for v in horizon): raise ContextContractError("unknown horizon resource")
        item=Context(ref,project_ref,tuple(operative_world),horizon,tuple(focus)); self.identities.create(ref,{"projectRef":str(project_ref)}); self.contexts[ref]=item; return item
    def resolve(self,ref,*,context_ref,resolver,generation,resource_refs):
        ref=self._ref(ref,"context-resolution"); context_ref=self._ref(context_ref,"context"); resources=tuple(self._ref(v,"context-resource") for v in resource_refs)
        if context_ref not in self.contexts or not set(resources).issubset(set(self.contexts[context_ref].information_horizon)): raise ContextContractError("invalid resolution")
        item=ContextResolution(ref,context_ref,resolver,generation,resources); self.identities.create(ref,{"contextRef":str(context_ref),"resolver":resolver}); self.resolutions[ref]=item; return item
    def disclosure(self,context_ref):
        context_ref=self._ref(context_ref,"context"); c=self.contexts[context_ref]
        return tuple({"ref":str(r),"sourceRef":str(self.resources[r].source_ref),"provider":self.resources[r].provider,"state":self.resources[r].state.value,"freshness":self.resources[r].freshness,"authoritative":self.resources[r].authoritative} for r in c.information_horizon)
