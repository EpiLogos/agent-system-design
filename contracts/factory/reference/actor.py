"""Executable Agent/Agency/AgentSession/Execution identity reference."""
from dataclasses import dataclass, replace
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

class ActorContractError(ValueError): pass

@dataclass(frozen=True)
class Agent:
    ref: object
    name: str
    canonical_position: str | None = None

@dataclass(frozen=True)
class AgencyProfile:
    ref: object
    name: str
    identity_form: tuple = ()

@dataclass(frozen=True)
class Agency:
    ref: object
    agent_ref: object
    profile_ref: object
    function: str

@dataclass(frozen=True)
class AgentSession:
    ref: object
    agent_ref: object
    agency_ref: object
    harness: str
    model: str
    provider: str
    continuity_key: str
    session_space: str | None = None

@dataclass(frozen=True)
class Execution:
    ref: object
    agent_ref: object
    agency_ref: object
    session_ref: object
    action: str
    model: str
    harness: str
    provider: str

EPI_PROFILE = (("0/1","Epi-Logos"),("#0","Anuttara"),("#1","Paramasiva"),("#2","Parāśakti"),("#3","Mahāmāyā"),("#4","Nara"),("#5","Epii"))

class ActorRuntime:
    def __init__(self):
        self.identities=IdentityStore(); self.agents={}; self.profiles={}; self.agencies={}; self.sessions={}; self.executions={}

    @staticmethod
    def _ref(value, kind):
        ref=Ref.parse(value) if isinstance(value,str) else value
        if ref.kind != kind: raise ActorContractError(f"expected {kind} Ref")
        return ref

    def register_agent(self, ref, *, name, canonical_position=None):
        ref=self._ref(ref,"agent")
        if not name.strip(): raise ActorContractError("Agent name required")
        item=Agent(ref,name.strip(),canonical_position)
        self.identities.create(ref,{"name":item.name,"canonicalPosition":canonical_position}); self.agents[ref]=item
        return item

    def register_profile(self, ref, *, name, identity_form=()):
        ref=self._ref(ref,"agency-profile")
        item=AgencyProfile(ref,name.strip(),tuple(identity_form))
        self.identities.create(ref,{"name":item.name,"identityForm":list(item.identity_form)}); self.profiles[ref]=item
        return item

    def situate(self, ref, *, agent_ref, profile_ref, function):
        ref=self._ref(ref,"agency"); agent_ref=self._ref(agent_ref,"agent"); profile_ref=self._ref(profile_ref,"agency-profile")
        if agent_ref not in self.agents or profile_ref not in self.profiles: raise ActorContractError("registered Agent and profile required")
        item=Agency(ref,agent_ref,profile_ref,function.strip())
        self.identities.create(ref,{"agentRef":str(agent_ref),"profileRef":str(profile_ref),"function":item.function}); self.agencies[ref]=item
        return item

    def open_session(self, ref, *, agency_ref, harness, model, provider, continuity_key, session_space=None):
        ref=self._ref(ref,"agent-session"); agency_ref=self._ref(agency_ref,"agency"); agency=self.agencies[agency_ref]
        item=AgentSession(ref,agency.agent_ref,agency.ref,harness,model,provider,continuity_key,session_space)
        self.identities.create(ref,{"agentRef":str(agency.agent_ref),"agencyRef":str(agency.ref),"continuityKey":continuity_key}); self.sessions[ref]=item
        return item

    def replace_session_runtime(self, session_ref, *, harness, model, provider, session_space=None):
        session_ref=self._ref(session_ref,"agent-session"); current=self.sessions[session_ref]
        updated=replace(current,harness=harness,model=model,provider=provider,session_space=session_space); self.sessions[session_ref]=updated
        return updated

    def execute(self, ref, *, session_ref, action):
        ref=self._ref(ref,"execution"); session_ref=self._ref(session_ref,"agent-session"); session=self.sessions[session_ref]
        item=Execution(ref,session.agent_ref,session.agency_ref,session.ref,action,session.model,session.harness,session.provider)
        self.identities.create(ref,{"agentRef":str(session.agent_ref),"sessionRef":str(session.ref),"action":action}); self.executions[ref]=item
        return item

    def install_epi_profile(self):
        profile=self.register_profile("factory:agency-profile:epi-logos",name="Epi-Logos canonical profile",identity_form=EPI_PROFILE)
        agents=[]
        for position,name in EPI_PROFILE:
            suffix="orchestrator" if position=="0/1" else position.replace("#","p")
            agents.append(self.register_agent(f"factory:agent:epi-logos.{suffix}",name=name,canonical_position=position))
        return profile,tuple(agents)
