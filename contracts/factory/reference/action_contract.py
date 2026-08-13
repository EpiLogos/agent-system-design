from dataclasses import dataclass

class ContractError(ValueError): pass
@dataclass(frozen=True)
class Capability: ref:str; name:str
@dataclass(frozen=True)
class Action: ref:str; project_ref:str; name:str; handler_ref:str
@dataclass(frozen=True)
class Invocation: ref:str; action_ref:str; projection:str; caller_execution_ref:str; handler_ref:str; output:object

class Catalog:
    def __init__(self): self.capabilities={}; self.actions={}; self.handlers={}; self.invocations={}
    def add_capability(self,item):
        if not item.ref.startswith("factory:capability:"): raise ContractError("Capability Ref required")
        self.capabilities[item.ref]=item; return item
    def add_action(self,item,handler):
        if not item.ref.startswith("factory:action:"): raise ContractError("Action Ref required")
        if item.ref in self.capabilities or not callable(handler): raise ContractError("Action and Capability remain distinct")
        self.actions[item.ref]=item; self.handlers[item.ref]=handler; return item
    def action_set(self,refs):
        refs=tuple(refs)
        if any(ref not in self.actions for ref in refs): raise ContractError("unknown Action")
        return refs
    def capability_set(self,refs):
        refs=tuple(refs)
        if any(ref not in self.capabilities for ref in refs): raise ContractError("unknown Capability")
        return refs
    def invoke(self,ref,*,action_ref,projection,caller_execution_ref,payload):
        if action_ref not in self.actions: raise ContractError("unknown Action")
        action=self.actions[action_ref]; item=Invocation(ref,action_ref,projection,caller_execution_ref,action.handler_ref,self.handlers[action_ref](payload)); self.invocations[ref]=item; return item
