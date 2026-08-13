import copy
import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/validate_factory_authority.py"
spec = importlib.util.spec_from_file_location("authority", SCRIPT)
authority = importlib.util.module_from_spec(spec)
spec.loader.exec_module(authority)


class AuthorityManifestTests(unittest.TestCase):
    def setUp(self):
        self.data = authority.load_manifest(ROOT / "contracts/factory/authority-manifest.json")

    def test_manifest_is_valid(self):
        self.assertEqual([], authority.validate_manifest(self.data))

    def test_provenance_round_trip_is_lossless(self):
        encoded = json.dumps(self.data, sort_keys=True)
        self.assertEqual(self.data, json.loads(encoded))

    def test_rejects_research_or_reference_promotion(self):
        mutated = copy.deepcopy(self.data)
        promoted = copy.deepcopy(mutated["retrievedReferences"][0])
        promoted.update({"precedence": 99, "scope": "bad", "governs": True})
        mutated["sources"].append(promoted)
        errors = authority.validate_manifest(mutated)
        self.assertTrue(any("silent authority promotion" in error for error in errors))

    def test_rejects_duplicate_source_identity(self):
        mutated = copy.deepcopy(self.data)
        mutated["sources"].append(copy.deepcopy(mutated["sources"][0]))
        errors = authority.validate_manifest(mutated)
        self.assertIn("duplicate source id", errors)
        self.assertIn("duplicate source path", errors)

    def test_index_has_unique_highest_precedence(self):
        mutated = copy.deepcopy(self.data)
        mutated["sources"][1]["precedence"] = 100
        self.assertIn("constitutional index must have unique highest precedence", authority.validate_manifest(mutated))

    def test_determination_statuses_are_exact(self):
        mutated = copy.deepcopy(self.data)
        mutated["determinationStatuses"].append("CONSENSUS")
        self.assertIn("determination status set differs from root programme", authority.validate_manifest(mutated))

    def test_generated_markdown_is_derived_from_manifest(self):
        rendered = authority.render_markdown(self.data)
        self.assertIn("QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md", rendered)
        self.assertIn("RUN-CLOSURE-VERIFICATION-ALIGNMENT.md", rendered)
        self.assertIn("Generated from `contracts/factory/authority-manifest.json`", rendered)


if __name__ == "__main__":
    unittest.main()
