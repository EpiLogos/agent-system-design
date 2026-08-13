#!/usr/bin/env python3
"""Run deterministic checks for the Software Factory repository."""
from __future__ import annotations

import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load_authority_module():
    path = ROOT / "scripts/validate_factory_authority.py"
    spec = importlib.util.spec_from_file_location("factory_authority", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> int:
    authority = load_authority_module()
    data = authority.load_manifest(ROOT / "contracts/factory/authority-manifest.json")
    failures = authority.validate_manifest(data)

    try:
        json.loads((ROOT / "contracts/factory/authority-manifest.schema.json").read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        failures.append(f"authority schema does not parse: {error}")

    generated = ROOT / "docs/generated/FACTORY-AUTHORITY.md"
    try:
        if generated.read_text(encoding="utf-8") != authority.render_markdown(data):
            failures.append("generated authority projection is stale")
    except OSError as error:
        failures.append(f"generated authority projection unavailable: {error}")

    if failures:
        for failure in failures:
            print(f"CHECK FAIL: {failure}", file=sys.stderr)
        return 1
    print("CHECK PASS: authority contract")

    suite = unittest.defaultTestLoader.discover(str(ROOT / "tests/factory"), pattern="test_*.py")
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    if not result.wasSuccessful():
        return 1
    print("CHECK PASS: Factory tests")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
