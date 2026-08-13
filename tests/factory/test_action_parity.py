import importlib.util,sys,unittest
from pathlib import Path
R=Path(__file__).resolve().parents[2]; s=importlib.util.spec_from_file_location("a",R/"contracts/factory/reference/action_contract.py"); a=importlib.util.module_from_spec(s); sys.modules[s.name]=a; s.loader.exec_module(a)
class ActionParity(unittest.TestCase):
 def test_one_action_many_projections(self):
  c=a.Catalog(); cap=c.add_capability(a.Capability("factory:capability:browser","Browser")); calls=[]
  def h(v): calls.append(v); return v
  action=c.add_action(a.Action("factory:action:update","factory:project:p1","Update","handler:update"),h); x=c.invoke("factory:invocation:i1",action_ref=action.ref,projection="cli",caller_execution_ref="factory:execution:e1",payload=1); y=c.invoke("factory:invocation:i2",action_ref=action.ref,projection="mcp",caller_execution_ref="factory:execution:e2",payload=2)
  self.assertEqual((x.action_ref,x.handler_ref),(y.action_ref,y.handler_ref)); self.assertEqual([1,2],calls); self.assertNotEqual(cap.ref,action.ref); self.assertEqual((cap.ref,),c.capability_set((cap.ref,))); self.assertEqual((action.ref,),c.action_set((action.ref,)))
