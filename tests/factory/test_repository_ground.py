import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class RepositoryGroundTests(unittest.TestCase):
    def setUp(self):
        self.ground = json.loads((ROOT / "contracts/factory/source-ground.json").read_text(encoding="utf-8"))

    def test_ground_is_pinned_and_observed(self):
        self.assertEqual("factory.source-ground/v1", self.ground["schemaVersion"])
        self.assertEqual("EpiLogos/agent-system-design", self.ground["repository"])
        self.assertEqual("7690069846eb6fc89f6aa78dcf7aab886ac7c737", self.ground["baseline"])
        self.assertEqual("OBSERVED", self.ground["determination"])

    def test_safe_root_insertion_points_are_explicit(self):
        self.assertEqual(
            ["contracts/factory/", "factory/", "tests/factory/", "scripts/factory_verify.py"],
            self.ground["safeInsertionPoints"],
        )

    def test_source_islands_are_not_factory_insertion_points(self):
        self.assertIn("ql-agent-experiments/", self.ground["nonInsertionPoints"])
        self.assertIn("super-simple-software-factory/", self.ground["nonInsertionPoints"])
        self.assertIn("inkwell-agent-sandboxes-and-software-factory/", self.ground["nonInsertionPoints"])

    def test_readme_identifies_repository_as_product(self):
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        self.assertIn("Software Factory product repository", readme)
        self.assertIn("GitHub Issues, pull requests and Actions are projections/providers", readme)


if __name__ == "__main__":
    unittest.main()
