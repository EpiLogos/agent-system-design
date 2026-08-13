import importlib.util,sys,unittest
from pathlib import Path
R=Path(__file__).resolve().parents[2]; s=importlib.util.spec_from_file_location("mat",R/"contracts/factory/reference/materialization.py"); m=importlib.util.module_from_spec(s); sys.modules[s.name]=m; s.loader.exec_module(m)
class MaterializationTests(unittest.TestCase):
 def test_provider_substitution_preserves_semantic_demand(self):
  d=m.CandidateMaterializationDemand("factory:execution-demand:d1",("shell",),candidate_ref="factory:candidate:c1"); a=m.materialize("factory:materialized-world:w1",demand=d,offer=m.WorkcellOffer("workcell:a",("shell",)),bindings=()); b=m.materialize("factory:materialized-world:w2",demand=d,offer=m.WorkcellOffer("workcell:b",("shell","snapshot")),bindings=()); self.assertEqual(a.demand_ref,b.demand_ref); self.assertEqual(a.candidate_ref,b.candidate_ref); self.assertNotEqual(a.workcell_ref,b.workcell_ref)
 def test_required_affordance_and_binding_identity_are_enforced(self):
  d=m.ExecutionDemand("factory:execution-demand:d2",("shell","internet"))
  with self.assertRaises(m.MaterializationError): m.materialize("factory:materialized-world:w",demand=d,offer=m.WorkcellOffer("workcell:a",("shell",)),bindings=())
  with self.assertRaises(m.MaterializationError): m.materialize("factory:materialized-world:w",demand=m.ExecutionDemand("factory:execution-demand:d3",("shell",)),offer=m.WorkcellOffer("workcell:a",("shell",)),bindings=(m.Binding("factory:binding:wrong","x","p"),))
if __name__=="__main__": unittest.main()
