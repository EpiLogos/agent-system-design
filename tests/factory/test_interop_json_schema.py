import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[2]


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


class InteropJsonSchemaTests(unittest.TestCase):
    def setUp(self):
        self.schema = load_json(ROOT / "contracts/factory/interop.schema.json")
        self.ql_schema = load_json(ROOT / "contracts/factory/ql-mef-composition.schema.json")
        self.fixture = load_json(ROOT / "contracts/factory/fixtures/interop-v1.json")
        self.ql_valid = load_json(ROOT / "contracts/factory/fixtures/ql-mef-composition-valid.json")
        self.ql_legacy = load_json(ROOT / "contracts/factory/fixtures/ql-mef-composition-legacy-invalid.json")
        self.registry = Registry().with_resource(
            self.ql_schema["$id"], Resource.from_contents(self.ql_schema)
        )

    def test_canonical_fixture_validates_against_composed_draft_2020_12_schema(self):
        Draft202012Validator.check_schema(self.schema)
        Draft202012Validator.check_schema(self.ql_schema)
        errors = sorted(
            Draft202012Validator(self.schema, registry=self.registry).iter_errors(
                self.fixture["contract"]
            ),
            key=lambda error: list(error.path),
        )
        self.assertEqual([], [error.message for error in errors])

    def test_parent_contract_references_ql_mef_owned_composition_schema(self):
        self.assertEqual(
            self.ql_schema["$id"], self.schema["properties"]["qlComposition"]["$ref"]
        )
        self.assertEqual(
            "Standalone QL/MEF module",
            self.schema["properties"]["qlComposition"]["x-semantic-owner"],
        )
        self.assertNotIn("qlComposition", self.schema["$defs"])

    def test_canonical_ql_mef_fixture_validates_and_legacy_shape_is_rejected(self):
        ql_validator = Draft202012Validator(self.ql_schema)
        self.assertEqual([], list(ql_validator.iter_errors(self.ql_valid)))
        self.assertNotEqual([], list(ql_validator.iter_errors(self.ql_legacy)))


if __name__ == "__main__":
    unittest.main()
