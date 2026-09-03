#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OPERATOR = ROOT / "skills/factory-operation/SKILL.md"
DEVELOPER = ROOT / "skills/factory-development/SKILL.md"
PRODUCT = ROOT / "skills/fixtures/product-improvement.json"
SKILL = ROOT / "skills/fixtures/skill-revision.json"


def require_text(path: Path, needles: list[str]) -> str:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise SystemExit(f"{path}: missing Agent Skill frontmatter")
    for needle in needles:
        if needle not in text:
            raise SystemExit(f"{path}: missing {needle!r}")
    return text


require_text(
    OPERATOR,
    [
        "factory:operator",
        "FactoryBuildViewProvider::snapshot",
        "FactoryActionExecutor::execute",
        "RunMutationAuthority",
        "Capability granted != Action authorised",
        "factory.build-view/v1",
        "plausible Claim != evidenced Closure",
        "partial evidence != whole satisfaction",
        "Whole-relative verification / plausibility barrier",
        "Claim -> operative Whole/opening condition -> VerificationRequirement/VerificationPlan -> Evidence/Assessment -> Closure/Gate",
    ],
)
require_text(
    DEVELOPER,
    [
        "factory:developer",
        "native-owner review / Recognition",
        "edited harness projection",
        "automatic backlog",
        "Proof over plausibility",
        "whole-relative verification ledger",
        "Discharge verification obligations",
        "A passing subset remains a passing subset.",
    ],
)

product = json.loads(PRODUCT.read_text(encoding="utf-8"))
assert product["schema"] == "factory.skill-workflow/v1"
assert product["kind"] == "product-improvement"
assert product["ownerSkillRef"] == "workcell:operator"
assert product["directForeignPrivateMutation"] is False
assert product["factoryRunImpliesRepositoryAuthority"] is False
assert product["automaticPromotion"] is False
assert product["stages"][-2:] == ["owner-recognition", "attributable-return"]

skill = json.loads(SKILL.read_text(encoding="utf-8"))
assert skill["schema"] == "factory.skill-workflow/v1"
assert skill["kind"] == "skill-revision"
assert skill["source"]["revision"]
assert skill["projectedCopyAuthoritative"] is False
assert skill["successfulUsePromotesAutomatically"] is False
assert skill["rollbackPointRequired"] is True
assert skill["stages"][-2:] == ["native-owner-recognition", "explicit-promotion-or-rejection"]

print("Factory native Skills and self-improvement fixtures: OK")
