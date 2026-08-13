import importlib.util,sys,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location("ctx",ROOT/"contracts/factory/reference/context.py")
ctx=importlib.util.module_from_spec(spec); sys.modules[spec.name]=ctx; spec.loader.exec_module(ctx)
class ContextContractTests(unittest.TestCase):
    def make(self):
        s=ctx.ContextStore(); r=s.add_resource("factory:context-resource:canon",source_ref="factory:artifact:canon",provider="index-a",freshness="rev-1",authoritative=True); c=s.create_context("factory:context:c1",project_ref="factory:project:factory",operative_world=("factory:agency:a1",),information_horizon=(r.ref,),focus=("factory:run:r1",)); return s,r,c
    def test_available_retrieved_loaded_are_distinct(self):
        s,r,_=self.make(); self.assertEqual("available",r.state.value)
        with self.assertRaises(ctx.ContextContractError): s.transition_resource(r.ref,"loaded")
        r=s.transition_resource(r.ref,"retrieved"); self.assertEqual("retrieved",r.state.value); self.assertEqual("loaded",s.transition_resource(r.ref,"loaded").state.value)
    def test_resolver_is_beneath_context(self):
        s,r,c=self.make(); resolution=s.resolve("factory:context-resolution:cr1",context_ref=c.ref,resolver="AIKit",generation="g1",resource_refs=(r.ref,)); self.assertEqual(c.ref,resolution.context_ref); self.assertNotEqual(c.ref,resolution.ref)
if __name__=="__main__": unittest.main()
