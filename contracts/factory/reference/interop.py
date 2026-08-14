"""Language-neutral CR-001 semantic conformance reference.

This module validates the shared JSON fixture corpus without becoming a client
runtime package. Node, TypeScript and Rust consumers implement the same rules
from the versioned schema and fixtures.
"""
from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any, Mapping

CANONICAL_REF = re.compile(r"^factory:[a-z][a-z0-9-]*:[A-Za-z0-9][A-Za-z0-9._-]*$")
KIND_REF = lambda kind: re.compile(rf"^factory:{re.escape(kind)}:[A-Za-z0-9][A-Za-z0-9._-]*$")
QL_COORDINATE_KEYS = ("qlFormRef", "qlAddress", "lensRef", "sublensRef")


class InteropError(ValueError):
    pass


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise InteropError(message)


def _subject_tuple(value: Mapping[str, Any]) -> tuple[str, str, int]:
    return (value.get("subjectRef"), value.get("stateRef"), value.get("revision"))


def _validate_ref(value: Any, kind: str | None = None) -> None:
    pattern = CANONICAL_REF if kind is None else KIND_REF(kind)
    _require(isinstance(value, str) and pattern.fullmatch(value) is not None, f"invalid {kind or 'canonical'} Ref: {value!r}")


def schema_owner_failures(schema: Mapping[str, Any]) -> list[str]:
    """Every declared Factory instance field in the schema has exactly one semantic owner."""
    failures: list[str] = []

    def walk(node: Any, path: str) -> None:
        if not isinstance(node, dict):
            return
        properties = node.get("properties")
        if isinstance(properties, dict):
            for name, child in properties.items():
                owner = child.get("x-semantic-owner") if isinstance(child, dict) else None
                if not isinstance(owner, str) or not owner.strip():
                    failures.append(f"{path}/{name} has no semantic owner")
                walk(child, f"{path}/{name}")
        defs = node.get("$defs")
        if isinstance(defs, dict):
            for name, child in defs.items():
                walk(child, f"{path}/$defs/{name}")
        items = node.get("items")
        if isinstance(items, dict):
            walk(items, f"{path}/items")

    walk(schema, "#")
    return failures


def _validate_ql_composition(
    ql: Mapping[str, Any], schema: Mapping[str, Any], ql_schema: Mapping[str, Any]
) -> None:
    parent = schema.get("properties", {}).get("qlComposition", {})
    _require(parent.get("$ref") == ql_schema.get("$id"), "parent QL schema reference drift")
    _require(parent.get("x-semantic-owner") == "Standalone QL/MEF module", "QL semantic owner drift")
    _require("qlComposition" not in schema.get("$defs", {}), "Factory must not duplicate QL composition definition")

    properties = ql_schema.get("properties")
    _require(isinstance(properties, dict), "QL schema properties missing")
    _require(set(ql).issubset(properties), "QL composition contains fields outside the standalone schema")
    _require(isinstance(ql.get("targetRef"), str) and bool(ql["targetRef"]), "QL targetRef is required")
    _require(any(key in ql for key in QL_COORDINATE_KEYS), "QL composition requires at least one coordinate")

    for key in QL_COORDINATE_KEYS:
        if key not in ql:
            continue
        pattern = properties.get(key, {}).get("pattern")
        _require(isinstance(pattern, str), f"QL schema pattern missing for {key}")
        value = ql[key]
        _require(isinstance(value, str) and re.fullmatch(pattern, value) is not None, f"invalid {key}: {value!r}")

    _validate_ref(ql["targetRef"])


def validate_contract(
    contract: Mapping[str, Any], schema: Mapping[str, Any], ql_schema: Mapping[str, Any]
) -> None:
    _require(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "interop schema must use JSON Schema 2020-12")
    owner_failures = schema_owner_failures(schema)
    _require(not owner_failures, "; ".join(owner_failures))
    required = schema.get("required", [])
    _require(set(contract) == set(required), "contract fields must exactly match the versioned schema surface")
    _require(contract.get("contractVersion") == "factory.interop/v1", "unsupported interop version")

    identity = contract["identityEnvelope"]
    _validate_ref(identity["ref"])
    _require(isinstance(identity["revision"], int) and identity["revision"] >= 1, "identity revision must be positive")
    current_state = identity["subjectState"]
    _validate_ref(current_state["subjectRef"])
    _require(current_state["subjectRef"] == identity["ref"], "identity subject-state must describe the same subject")
    _require(current_state["revision"] == identity["revision"], "identity revision and subject-state revision drift")

    action = contract["actionDescriptor"]
    capability = contract["capabilityDescriptor"]
    _validate_ref(action["actionRef"], "action")
    _validate_ref(capability["capabilityRef"], "capability")
    _validate_ref(action["ownerProjectRef"], "project")
    _require(action["actionRef"] != capability["capabilityRef"], "Action and Capability identity collapsed")

    project_binding = contract["projectBinding"]
    _validate_ref(project_binding["projectBindingRef"], "project-binding")
    _validate_ref(project_binding["projectRef"], "project")
    _require(project_binding["projectRef"] == action["ownerProjectRef"], "project ownership drift across Action and ProjectBinding")
    _require(not str(project_binding["sourceRef"]).startswith("factory:project:"), "source reference cannot replace Factory Project identity")

    run = contract["runEnvelope"]
    _validate_ref(run["runRef"], "run")
    _validate_ref(run["projectRef"], "project")
    _validate_ref(run["runMapRef"], "run-map")
    _require(run["projectRef"] == project_binding["projectRef"], "Run ProjectRef drift")

    execution = contract["executionEnvelope"]
    _validate_ref(execution["executionRef"], "execution")
    _validate_ref(execution["runRef"], "run")
    _validate_ref(execution["agentRef"], "agent")
    _validate_ref(execution["agencyRef"], "agency")
    _validate_ref(execution["agentSessionRef"], "agent-session")
    _require(execution["runRef"] == run["runRef"], "Execution RunRef drift")
    _require(not execution["agentRef"].startswith("factory:agent-session:"), "Agent cannot be an AgentSession")
    _require(not execution["agentRef"].startswith("model:"), "Agent cannot be a model")

    evidence = contract["evidenceEnvelope"]
    assessment = contract["assessmentEnvelope"]
    closure = contract["closureEnvelope"]
    gate = contract["gateDecisionEnvelope"]
    _validate_ref(evidence["evidenceRef"], "evidence")
    _validate_ref(assessment["assessmentRef"], "assessment")
    _validate_ref(closure["closureRef"], "closure")
    _validate_ref(gate["gateDecisionRef"], "gate-decision")
    _validate_ref(evidence["producerExecutionRef"], "execution")
    _validate_ref(assessment["producerExecutionRef"], "execution")
    for label, value in (("evidence", evidence), ("assessment", assessment), ("closure", closure), ("gate", gate)):
        _require(_subject_tuple(value["subjectState"]) == _subject_tuple(current_state), f"{label} subject-state is stale or mismatched")
    _require(evidence["evidenceRef"] in assessment["evidenceRefs"], "Assessment lost EvidenceRef")
    _require(evidence["evidenceRef"] in closure["evidenceRefs"], "Closure lost EvidenceRef")
    _require(assessment["assessmentRef"] in closure["assessmentRefs"], "Closure lost AssessmentRef")
    _require(closure["closureRef"] == gate["closureRef"], "Gate/Closure relation drift")
    _require(assessment["producerExecutionRef"] != evidence["producerExecutionRef"], "independent Assessment reused producer execution")
    _require(evidence["producerExecutionRef"] in assessment["independentFromExecutionRefs"], "Assessment independence lineage missing")

    demand = contract["executionDemand"]
    _validate_ref(demand["demandRef"], "execution-demand")
    _validate_ref(demand["projectRef"], "project")
    _validate_ref(demand["runRef"], "run")
    if demand["candidateRef"] is not None:
        _validate_ref(demand["candidateRef"], "candidate")
    _require(not any("provider" in key.lower() for key in demand), "ExecutionDemand leaked provider selection")
    _require(demand["projectRef"] == project_binding["projectRef"] and demand["runRef"] == run["runRef"], "ExecutionDemand semantic identity drift")
    _require(demand["candidateRef"] == identity["ref"], "ExecutionDemand CandidateRef drift")

    offer = contract["workcellOffer"]
    missing = set(demand["requiredAffordances"]) - set(offer["affordances"])
    _require(not missing, f"WorkcellOffer misses required affordances: {sorted(missing)}")

    binding = contract["binding"]
    _require(isinstance(binding["bindingId"], str) and binding["bindingId"].startswith("binding:"), "Binding must use Workcell binding identity")
    _require(not binding["bindingId"].startswith("factory:"), "Binding cannot masquerade as canonical Ref")
    _validate_ref(binding["executionRef"], "execution")
    _require(binding["executionRef"] == execution["executionRef"], "Binding execution relation drift")
    _require(binding["workcellRef"] == offer["workcellRef"], "Binding WorkcellRef drift")

    generation = contract["generationProvenance"]
    _validate_ref(generation["generationRef"], "generation")
    _validate_ref(generation["contextResolutionRef"], "context-resolution")
    _validate_ref(generation["projectRef"], "project")
    _require(generation["projectRef"] == project_binding["projectRef"], "Generation ProjectRef drift")
    _require(action["actionRef"] in generation["actionRefs"], "Generation lost ActionRef")
    _require(capability["capabilityRef"] in generation["capabilityRefs"], "Generation lost CapabilityRef")
    _require(execution["generationRef"] == generation["generationRef"], "Execution GenerationRef drift")

    projections = contract["projectionProvenance"]
    _require(isinstance(projections, list) and len(projections) >= 2, "at least two projections are required for provider-change fixture")
    providers = set()
    external_ids = set()
    for projection in projections:
        _validate_ref(projection["projectionRef"], "projection")
        _validate_ref(projection["canonicalRef"])
        _require(projection["canonicalRef"] == project_binding["projectRef"], "projection lost canonical identity")
        _require(projection["projectedRevision"] >= 1, "projection revision invalid")
        _require(projection["generationRef"] == generation["generationRef"], "projection GenerationRef drift")
        providers.add(projection["providerRef"])
        external_ids.add(projection["externalId"])
    _require(len(providers) >= 2 and len(external_ids) >= 2, "provider/projection-change fixture must actually change provider and external identity")

    _validate_ql_composition(contract["qlComposition"], schema, ql_schema)


def reject_anti_fixture(item: Mapping[str, Any]) -> None:
    fixture_id = item.get("id")
    value = item.get("value", {})
    if fixture_id == "action-as-capability-identity-collapse":
        _validate_ref(value.get("actionRef"), "action")
        _validate_ref(value.get("capabilityRef"), "capability")
        _require(value["actionRef"] != value["capabilityRef"], "Action and Capability identity collapsed")
        return
    if fixture_id == "binding-as-ref":
        _require(isinstance(value.get("bindingId"), str) and value["bindingId"].startswith("binding:"), "Binding cannot be a canonical Ref")
        return
    if fixture_id in {"model-as-agent", "session-as-agent"}:
        _validate_ref(value.get("agentRef"), "agent")
        return
    if fixture_id == "provider-as-project":
        _validate_ref(value.get("projectRef"), "project")
        return
    if fixture_id == "stale-subject-state-evidence":
        _require(_subject_tuple(value["currentSubjectState"]) == _subject_tuple(value["evidenceSubjectState"]), "stale or mismatched Evidence cannot be current")
        return
    raise InteropError(f"unknown anti-fixture: {fixture_id}")


def validate_fixture_document(
    document: Mapping[str, Any], schema: Mapping[str, Any], ql_schema: Mapping[str, Any]
) -> None:
    _require(document.get("fixtureVersion") == "factory.interop-fixtures/v1", "unsupported fixture corpus version")
    validate_contract(document["contract"], schema, ql_schema)
    anti = document.get("antiFixtures")
    _require(isinstance(anti, list) and anti, "anti-fixture corpus is required")
    required_ids = {
        "action-as-capability-identity-collapse",
        "binding-as-ref",
        "model-as-agent",
        "session-as-agent",
        "provider-as-project",
        "stale-subject-state-evidence",
    }
    _require({item.get("id") for item in anti} == required_ids, "anti-fixture set is incomplete or contains unknown cases")
    for item in anti:
        _require(item.get("mustReject") is True, f"anti-fixture {item.get('id')} must be marked mustReject")
        try:
            reject_anti_fixture(item)
        except InteropError:
            continue
        raise InteropError(f"anti-fixture unexpectedly accepted: {item['id']}")


def round_trip(
    document: Mapping[str, Any], schema: Mapping[str, Any], ql_schema: Mapping[str, Any]
) -> dict[str, Any]:
    encoded = json.dumps(document, sort_keys=True, separators=(",", ":"))
    decoded = json.loads(encoded)
    validate_fixture_document(decoded, schema, ql_schema)
    return decoded


def load_default(root: Path) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    schema = json.loads((root / "contracts/factory/interop.schema.json").read_text(encoding="utf-8"))
    ql_schema = json.loads((root / "contracts/factory/ql-mef-composition.schema.json").read_text(encoding="utf-8"))
    document = json.loads((root / "contracts/factory/fixtures/interop-v1.json").read_text(encoding="utf-8"))
    return schema, ql_schema, document
