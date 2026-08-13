from dataclasses import dataclass
import importlib.util
from pathlib import Path
import sys
_NAME="factory_identity_contract"
if _NAME not in sys.modules:
    s=importlib.util.spec_from_file_location(_NAME,Path(__file__).with_name("identity.py")); m=importlib.util.module_from_spec(s); sys.modules[_NAME]=m; s.loader.exec_module(m)
identity=sys.modules[_NAME]; Ref=identity.Ref; IdentityStore=identity.IdentityStore
class InvocationContractError(ValueError): pass
@dataclass(frozen=True)
class Capability: ref:object; name:str; provider:str
@dataclass(frozen=True)
class Action: ref:object; project_ref:object; name:str; handler_ref:str
@dataclass(frozen=True)
class CapabilitySet: ref:object; capability_refs:tuple
@dataclass(frozen=True)
class ActionSet: ref:object; action_refs:tuple
@dataclass(frozen=True)
class Invocation: ref:object; action_ref:object; projection:str; caller_execution_ref:object; handler_ref:str; input:object; output:object
class InvocationRuntime:
    def __init__(self): self.identities=IdentityStore(); self.capabilities={}; self.actions={}; self.handlers={}; self.invocations={}
    @staticmethod
    def _ref(value,kind):
        ref=Ref.parse(value) if isinstance(value,str) else value
        if ref.kind!=kind: raise InvocationContractError(f"expected {kind} Ref")
        return ref
    def register_capability(self,ref,*,name,provider):
        ref=self._ref(ref,"capability"); item=Capability(ref,name,provider); self.identities.create(ref,{"name":name,"provider":provider}); self.capabilities[ref]=item; return item
    def register_action(self,ref,*,project_ref,name,handler_ref,handler):
        ref=self._ref(ref,"action"); project_ref=self._ref(project_ref,"project")
        if not callable(handler): raise InvocationContractError("handler required")
        item=Action(ref,project_ref,name,handler_ref); self.identities.create(ref,{"projectRef":str(project_ref),"handlerRef":handler_ref}); self.actions[ref]=item; self.handlers[ref]=handler; return item
