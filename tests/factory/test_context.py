import importlib.util,sys,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location("ctx_provider",ROOT/"contracts/factory/reference/context.py")
ctx=importlib.util.module_from_spec(spec); sys.modules[spec.name]=ctx; spec.loader.exec_module(ctx)
class ContextProviderTests(unittest.TestCase):
    def test_provider_change_does_not_change_source_authority(self):
        s=ctx.ContextStore(); r=s.add_resource("factory:context-resource:r1",source_ref="factory:artifact:canon",provider="index-a",freshness="rev-1",authoritative=True); c=s.create_context("factory:context:c1",project_ref="factory:project:p1",operative_world=(),information_horizon=(r.ref,),focus=()); before=s.disclosure(c.ref)[0]; s.resources[r.ref]=ctx.replace(s.resources[r.ref],provider="index-b"); after=s.disclosure(c.ref)[0]; self.assertEqual(before["sourceRef"],after["sourceRef"]); self.assertTrue(after["authoritative"]); self.assertNotEqual(before["provider"],after["provider"])
    def test_project_map_is_navigation_not_target_identity(self):
        s=ctx.ContextStore(); m=s.project_map("factory:project-map:m1",project_ref="factory:project:p1",entries=(("factory:project-map-entry:e1","factory:artifact:canon","design","gitnexus"),)); self.assertEqual("factory:artifact:canon",str(m.entries[0].target_ref)); self.assertNotEqual(m.ref,m.entries[0].target_ref)
if __name__=="__main__": unittest.main()
