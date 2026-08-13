"""Executable reference for Project, durable Run and canonical RunMap semantics."""
from __future__ import annotations

from dataclasses import dataclass
import importlib.util
from pathlib import Path
import sys
from typing import Any, Mapping

_IDENTITY_NAME = "factory_identity_contract"
if _IDENTITY_NAME in sys.modules:
    identity = sys.modules[_IDENTITY_NAME]
else:
    _spec = importlib.util.spec_from_file_location(_IDENTITY_NAME, Path(__file__).with_name("identity.py"))
    identity = importlib.util.module_from_spec(_spec)
    sys.modules[_IDENTITY_NAME] = identity
    _spec.loader.exec_module(identity)

Ref = identity.Ref
IdentityStore = identity.IdentityStore


class RunContractError(ValueError):
    pass


class DuplicateCanonicalRunMap(RunContractError):
    pass


class InvalidTopologyMutation(RunContractError):
    pass


class RunMapConflict(RunContractError):
    pass


class RunProjectionLoss(RunContractError):
    pass


@dataclass(frozen=True)
class ProjectState:
    ref: Any
    title: str


@dataclass(frozen=True)
class RunState:
    ref: Any
    project_ref: Any
    opening_condition: str
    destination: str
    canonical_run_map_ref: Any


@dataclass(frozen=True)
class RunMapNode:
    id: str
    kind: str
    semantic_ref: Any | None = None


@dataclass(frozen=True)
class RunMapAuthority:
    run_ref: Any
    run_map_ref: Any


@dataclass(frozen=True)
class RunMapState:
    ref: Any
    run_ref: Any
    revision: int
    nodes: tuple[RunMapNode, ...]
    edges: tuple[tuple[str, str, str], ...]


class ProjectRunStore:
    """Deterministic semantic model; provider graph/session/host never own Run identity."""

    def __init__(self) -> None:
        self.identities = IdentityStore()
        self._projects: dict[Any, ProjectState] = {}
        self._runs: dict[Any, RunState] = {}
        self._maps_by_run: dict[Any, RunMapState] = {}

    @staticmethod
    def _require_kind(ref: Any, kind: str) -> Any:
        ref = Ref.parse(ref) if isinstance(ref, str) else ref
        if ref.kind != kind:
            raise RunContractError(f"expected {kind} Ref, got {ref}")
        return ref

    def create_project(self, ref: Any, title: str, source_refs: tuple[str, ...] = ()) -> ProjectState:
        ref = self._require_kind(ref, "project")
        if not isinstance(title, str) or not title.strip():
            raise RunContractError("Project title is required")
        self.identities.create(ref, {"title": title.strip(), "sourceRefs": list(source_refs)})
        state = ProjectState(ref, title.strip())
        self._projects[ref] = state
        return state

    def update_project_sources(self, ref: Any, *, expected_revision: int, source_refs: tuple[str, ...]) -> ProjectState:
        ref = self._require_kind(ref, "project")
        current = self.identities.get(ref)
        self.identities.update(
            ref,
            expected_revision=expected_revision,
            payload={"title": current.payload["title"], "sourceRefs": list(source_refs)},
        )
        return self._projects[ref]

    def open_run(
        self,
        project_ref: Any,
        run_ref: Any,
        run_map_ref: Any,
        *,
        opening_condition: str,
        destination: str,
    ) -> RunState:
        project_ref = self._require_kind(project_ref, "project")
        run_ref = self._require_kind(run_ref, "run")
        run_map_ref = self._require_kind(run_map_ref, "run-map")
        if project_ref not in self._projects:
            raise RunContractError(f"unknown Project: {project_ref}")
        if run_ref in self._maps_by_run:
            raise DuplicateCanonicalRunMap(f"Run already owns canonical RunMap: {run_ref}")
        if run_ref in self._runs:
            raise RunContractError(f"Run already exists: {run_ref}")
        if not opening_condition.strip() or not destination.strip():
            raise RunContractError("Run requires opening condition and destination")
        self.identities.create(
            run_ref,
            {"projectRef": str(project_ref), "openingCondition": opening_condition, "destination": destination},
        )
        self.identities.create(run_map_ref, {"runRef": str(run_ref)})
        state = RunState(run_ref, project_ref, opening_condition, destination, run_map_ref)
        self._runs[run_ref] = state
        self._maps_by_run[run_ref] = RunMapState(
            run_map_ref,
            run_ref,
            1,
            (RunMapNode("opening", "opening-condition"), RunMapNode("destination", "destination")),
            (("opening", "destination", "develops-toward"),),
        )
        return state

    def attach_second_canonical_map(self, run_ref: Any, run_map_ref: Any) -> None:
        run_ref = self._require_kind(run_ref, "run")
        self._require_kind(run_map_ref, "run-map")
        if run_ref in self._maps_by_run:
            raise DuplicateCanonicalRunMap(f"Run already owns canonical RunMap: {run_ref}")
        raise RunContractError("cannot attach RunMap to unknown Run")

    def run(self, ref: Any) -> RunState:
        ref = self._require_kind(ref, "run")
        try:
            return self._runs[ref]
        except KeyError as error:
            raise RunContractError(f"unknown Run: {ref}") from error

    def canonical_map(self, run_ref: Any) -> RunMapState:
        run_ref = self._require_kind(run_ref, "run")
        try:
            return self._maps_by_run[run_ref]
        except KeyError as error:
            raise RunContractError(f"Run has no canonical RunMap: {run_ref}") from error

    def mutation_authority(self, run_ref: Any) -> RunMapAuthority:
        run = self.run(run_ref)
        return RunMapAuthority(run.ref, run.canonical_run_map_ref)

    def _current(self, authority: RunMapAuthority, expected_revision: int) -> RunMapState:
        current = self.canonical_map(authority.run_ref)
        if current.ref != authority.run_map_ref:
            raise RunMapConflict("mutation authority does not name canonical RunMap")
        if current.revision != expected_revision:
            raise RunMapConflict(
                f"stale topology mutation: expected {expected_revision}, current {current.revision}"
            )
        return current

    def add_node(
        self,
        authority: RunMapAuthority,
        *,
        expected_revision: int,
        node_id: str,
        kind: str,
        semantic_ref: Any | None = None,
    ) -> RunMapState:
        current = self._current(authority, expected_revision)
        if not node_id.strip() or not kind.strip():
            raise InvalidTopologyMutation("node id and kind are required")
        if any(node.id == node_id for node in current.nodes):
            raise InvalidTopologyMutation(f"duplicate RunMap node: {node_id}")
        if isinstance(semantic_ref, str):
            semantic_ref = Ref.parse(semantic_ref)
        updated = RunMapState(
            current.ref,
            current.run_ref,
            current.revision + 1,
            (*current.nodes, RunMapNode(node_id, kind, semantic_ref)),
            current.edges,
        )
        self._maps_by_run[current.run_ref] = updated
        return updated

    def add_edge(
        self,
        authority: RunMapAuthority,
        *,
        expected_revision: int,
        source: str,
        target: str,
        relation: str,
    ) -> RunMapState:
        current = self._current(authority, expected_revision)
        node_ids = {node.id for node in current.nodes}
        if source not in node_ids or target not in node_ids or source == target or not relation.strip():
            raise InvalidTopologyMutation("edge must connect distinct existing nodes with a relation")
        edge = (source, target, relation.strip())
        if edge in current.edges:
            raise InvalidTopologyMutation("duplicate RunMap edge")
        updated = RunMapState(
            current.ref,
            current.run_ref,
            current.revision + 1,
            current.nodes,
            (*current.edges, edge),
        )
        self._maps_by_run[current.run_ref] = updated
        return updated

    def resolve_projection(self, projection: Mapping[str, Any]) -> tuple[RunState, RunMapState]:
        run_value = projection.get("canonicalRunRef")
        map_value = projection.get("canonicalRunMapRef")
        if not isinstance(run_value, str) or not isinstance(map_value, str):
            raise RunProjectionLoss("projection must retain canonical Run and RunMap refs")
        run = self.run(run_value)
        run_map = self.canonical_map(run.ref)
        if str(run_map.ref) != map_value:
            raise RunProjectionLoss("projection RunMap ref does not match canonical map")
        return run, run_map
