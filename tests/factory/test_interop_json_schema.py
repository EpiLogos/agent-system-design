import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[2]


class InteropJsonSchemaTests(unittest.TestCase):
    def test_canonical_fixture_validates_against_draft_2020_12_schema(self):
        schema = json.loads((ROOT / "contracts/factory/interop.schema.json").read_text(encoding="utf-8"))
        fixture = json.loads((ROOT / "contracts/factory/fixtures/interop-v1.json").read_text(encoding="utf-8"))
        Draft202012Validator.check_schema(schema)
        errors = sorted(Draft202012Validator(schema).iter_errors(fixture["contract"]), key=lambda error: list(error.path))
        self.assertEqual([], [error.message for error in errors])


if __name__ == "__main__":
    unittest.main()
