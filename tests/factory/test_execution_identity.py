import importlib.util
import sys
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location("factory_actor_contract",ROOT/"contracts/factory/reference/actor.py")
actor=importlib.util.module_from_spec(spec); sys.modules[spec.name]=actor; spec.loader.exec_module(actor)

class ExecutionIdentityTests(unittest.TestCase):
    def make_generic(self):
        runtime=actor.ActorRuntime()
        agent=runtime.register_agent("factory:agent:generic-builder",name="Generic Builder")
        profile=runtime.register_profile("factory:agency-profile:code",name="Code")
        agency=runtime.situate("factory:agency:generic-build",agent_ref=agent.ref,profile_ref=profile.ref,function="development")
        session=runtime.open_session("factory:agent-session:s-1",agency_ref=agency.ref,harness="pi",model="model-a",provider="provider-a",continuity_key="conversation-1",session_space="cmux-a")
        return runtime,agent,profile,agency,session

    def test_runtime_changes_never_rewrite_agent_ref(self):
        runtime,agent,_,agency,session=self.make_generic()
        updated=runtime.replace_session_runtime(session.ref,harness="h2",model="m2",provider="p2",session_space="tmux-b")
        self.assertEqual(agent.ref,updated.agent_ref)
        self.assertEqual(agent.ref,runtime.agencies[agency.ref].agent_ref)

    def test_execution_is_concrete_provenance_snapshot(self):
        runtime,agent,_,agency,session=self.make_generic()
        execution=runtime.execute("factory:execution:e-1",session_ref=session.ref,action="implement")
        runtime.replace_session_runtime(session.ref,harness="h2",model="m2",provider="p2")
        self.assertEqual((agent.ref,agency.ref,session.ref),(execution.agent_ref,execution.agency_ref,execution.session_ref))
        self.assertEqual(("model-a","pi","provider-a"),(execution.model,execution.harness,execution.provider))

    def test_epi_and_generic_share_runtime_and_preserve_constellation(self):
        runtime,generic,_,_,_=self.make_generic()
        profile,agents=runtime.install_epi_profile()
        self.assertIsInstance(generic,actor.Agent)
        self.assertTrue(all(isinstance(item,actor.Agent) for item in agents))
        self.assertEqual(actor.EPI_PROFILE,tuple((item.canonical_position,item.name) for item in agents))
        self.assertEqual("Epi-Logos canonical profile",profile.name)

    def test_agent_agency_profile_session_execution_are_distinct(self):
        runtime,agent,profile,agency,session=self.make_generic()
        execution=runtime.execute("factory:execution:e-1",session_ref=session.ref,action="act")
        refs={str(agent.ref),str(profile.ref),str(agency.ref),str(session.ref),str(execution.ref)}
        self.assertEqual(5,len(refs))

if __name__=="__main__": unittest.main()
