#!/usr/bin/env python3
"""Validate and render the Factory constitutional authority manifest using stdlib only."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "contracts/factory/authority-manifest.json"
EXPECTED_DETERMINATIONS = {
    "CONSTITUTIONAL DETERMINATION", "CURRENT DESIGN", "OBSERVED", "VERIFIED",
    "SOURCE-INSPECTION BLOCKED", "OPEN DECISION", "GENUINE HUMAN AUTHORSHIP",
    "OPEN SOCKET", "RESEARCH CLAIM", "SUPERSEDED",
}
ALLOWED_DOCUMENT_STATUSES = {
    "constitutional", "current design", "module specification", "refined by",
    "superseded", "experimental / research claim", "reference/source material",
}


def load_manifest(path: Path = DEFAULT_MANIFEST) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def validate_manifest(data: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if data.get("schemaVersion") != "factory.authority-manifest/v1":
        errors.append("unsupported schemaVersion")
    statuses = data.get("determinationStatuses", [])
    if len(statuses) != len(set(statuses)):
        errors.append("duplicate determination status")
    if set(statuses) != EXPECTED_DETERMINATIONS:
        errors.append("determination status set differs from root programme")

    sources = data.get("sources", [])
    ids = [item.get("id") for item in sources]
    paths = [item.get("path") for item in sources]
    if len(ids) != len(set(ids)):
        errors.append("duplicate source id")
    if len(paths) != len(set(paths)):
        errors.append("duplicate source path")
    if not sources:
        errors.append("no authority sources")

    for item in sources:
        if item.get("status") not in ALLOWED_DOCUMENT_STATUSES:
            errors.append(f"invalid document status for {item.get('id')}")
        if item.get("determinationStatus") not in EXPECTED_DETERMINATIONS:
            errors.append(f"invalid determination status for {item.get('id')}")
        provenance = item.get("provenance") or {}
        if not provenance.get("ref") or not provenance.get("blobSha"):
            errors.append(f"missing provenance for {item.get('id')}")
        if item.get("governs") and item.get("status") in {
            "experimental / research claim", "reference/source material", "superseded"
        }:
            errors.append(f"silent authority promotion for {item.get('id')}")

    index = next((item for item in sources if item.get("id") == "constitutional-index"), None)
    if not index or not index.get("governs") or index.get("status") != "constitutional":
        errors.append("constitutional index must govern as constitutional")
    if index and any(item.get("precedence", -1) >= index.get("precedence", 0) for item in sources if item is not index):
        errors.append("constitutional index must have unique highest precedence")

    for ref in data.get("retrievedReferences", []):
        if ref.get("governs"):
            errors.append(f"retrieved reference may not silently govern: {ref.get('id')}")

    for rule in data.get("promotionRules", []):
        if rule.get("allowedSilently") is not False:
            errors.append(f"promotion must require evidence/authority: {rule}")
    return errors


def render_markdown(data: dict[str, Any]) -> str:
    lines = [
        "# Factory authority manifest (generated)", "",
        "> Generated from `contracts/factory/authority-manifest.json`; do not edit by hand.", "",
        f"Pinned baseline: `{data['repository']['baselineCommit']}` (`{data['repository']['baselineRef']}`)", "",
        "| Precedence | Source | Status | Determination | Governs | Scope |", "|---:|---|---|---|---|---|",
    ]
    for item in sorted(data["sources"], key=lambda x: x["precedence"], reverse=True):
        lines.append(
            f"| {item['precedence']} | `{item['path']}` | {item['status']} | "
            f"{item['determinationStatus']} | {'yes' if item['governs'] else 'no'} | {item['scope']} |"
        )
    lines += ["", "## Retrieved but non-governing references", ""]
    for item in data.get("retrievedReferences", []):
        lines.append(f"- `{item['path']}` — {item['status']}: {item['reason']}")
    lines += ["", "## Promotion rule", "", "No source becomes more authoritative because files agree, generated prose looks formal, or a provider exposes it.", ""]
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--render", type=Path)
    args = parser.parse_args()
    data = load_manifest(args.manifest)
    errors = validate_manifest(data)
    if errors:
        for error in errors:
            print(f"ERROR: {error}")
        return 1
    if args.render:
        args.render.parent.mkdir(parents=True, exist_ok=True)
        args.render.write_text(render_markdown(data), encoding="utf-8")
    print(f"authority manifest valid: {len(data['sources'])} sources")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
