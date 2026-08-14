#!/usr/bin/env python3
import json
from pathlib import Path

from jsonschema import Draft202012Validator
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
schema = json.loads((ROOT / "contracts/factory/interop.schema.json").read_text(encoding="utf-8"))
ql_schema = json.loads((ROOT / "contracts/factory/ql-mef-composition.schema.json").read_text(encoding="utf-8"))
fixture = json.loads((ROOT / "contracts/factory/fixtures/interop-v1.json").read_text(encoding="utf-8"))

Draft202012Validator.check_schema(schema)
Draft202012Validator.check_schema(ql_schema)
registry = Registry().with_resource(ql_schema["$id"], Resource.from_contents(ql_schema))
Draft202012Validator(schema, registry=registry).validate(fixture["contract"])
print("JSON Schema interop PASS")
