import importlib.util,sys,unittest
from pathlib import Path
R=Path(__file__).resolve().parents[2]; s=importlib.util.spec_from_file_location("verify_contract",R/"contracts/factory/reference/verification.py"); v=importlib.util.module_from_spec(s); sys.modules[s.name]=v; s.loader.exec_module(v)
class VerificationTests(unittest.TestCase):
 def low(self,state="rev-2"):
  req=v.VerificationRequirement("factory:verification-requirement:r1","low",("unit",)); disp=v.HumanDisposition("central:control:verification",False); return v.resolve_plan("factory:verification-plan:p1",req,"factory:candidate:c1",state,disp)
 def test_run_termination_is_not_closure(self):
  with self.assertRaises(v.VerificationContractError): v.close("factory:closure:c1",plan=self.low(),opening_condition="change",checks=(),evidence=(),run_terminated=True)
 def test_stale_and_mismatched_evidence_cannot_close_new_state(self):
  p=self.low(); c=v.Check("factory:check:k1","unit",True,p.subject_ref,"rev-1"); e=v.bind_evidence("factory:evidence:e1",c,True)
  with self.assertRaises(v.VerificationContractError): v.close("factory:closure:c1",plan=p,opening_condition="change",checks=(c,),evidence=(e,))
  c2=v.Check("factory:check:k2","unit",True,p.subject_ref,p.subject_state); e2=v.bind_evidence("factory:evidence:e2",c2,False)
  with self.assertRaises(v.VerificationContractError): v.close("factory:closure:c2",plan=p,opening_condition="change",checks=(c2,),evidence=(e2,))
 def test_low_and_high_risk_resolve_from_same_field(self):
  low=self.low(); high=v.resolve_plan("factory:verification-plan:p2",v.VerificationRequirement("factory:verification-requirement:r2","high",("unit",)),"factory:candidate:c1","rev-2",v.HumanDisposition("central:control:verification",False)); self.assertFalse(low.assessment_required); self.assertFalse(low.review_required); self.assertTrue(high.assessment_required); self.assertTrue(high.review_required)
 def test_assurance_impact_requires_independent_scrutiny(self):
  p=v.resolve_plan("factory:verification-plan:p3",v.VerificationRequirement("factory:verification-requirement:r3","low",("unit",)),"factory:candidate:c1","rev-2",v.HumanDisposition("central:control:verification",False),impact="closure"); self.assertTrue(p.independent); self.assertTrue(p.assessment_required)
 def test_positive_closure_then_gate(self):
  p=self.low(); c=v.Check("factory:check:k1","unit",True,p.subject_ref,p.subject_state); e=v.bind_evidence("factory:evidence:e1",c,True); closure=v.close("factory:closure:c1",plan=p,opening_condition="change",checks=(c,),evidence=(e,)); self.assertEqual(p.subject_state,closure.subject_state); self.assertFalse(v.gate("factory:gate-decision:g0",transition="promote").allowed); g=v.gate("factory:gate-decision:g1",transition="promote",closure=closure); self.assertTrue(g.allowed); self.assertEqual(closure.ref,g.closure_ref)
if __name__=="__main__": unittest.main()
