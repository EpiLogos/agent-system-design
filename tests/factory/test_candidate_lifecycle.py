import importlib.util,sys,unittest
from pathlib import Path
R=Path(__file__).resolve().parents[2]; s=importlib.util.spec_from_file_location("reality",R/"contracts/factory/reference/reality.py"); reality=importlib.util.module_from_spec(s); sys.modules[s.name]=reality; s.loader.exec_module(reality)
class CandidateTests(unittest.TestCase):
 def test_candidate_survives_provider_substitution(self):
  store=reality.CandidateStore(); a=store.add_artifact(reality.Artifact("factory:artifact:a1","development","blob:1")); c=store.create_candidate(reality.Candidate("factory:candidate:c1","factory:project:p1",(a.ref,))); c=store.materialize(c.ref,provider="wc-a",environment_ref="env-a"); c=store.materialize(c.ref,provider="wc-b",environment_ref="env-b"); self.assertEqual("factory:candidate:c1",c.ref); self.assertEqual(("wc-a","wc-b"),tuple(x.provider for x in c.materializations))
 def test_recursion_routes_only_authorised_effects(self):
  store=reality.CandidateStore(); r=store.recursion(reality.RecursionArtifact("factory:recursion-artifact:r1","factory:run:r1","factory:decision:d1",("promote-design",))); self.assertEqual("promote-design",store.route_effect(r.ref,"promote-design"))
  with self.assertRaises(reality.CandidateContractError): store.route_effect(r.ref,"rewrite-intent")
if __name__=="__main__": unittest.main()
