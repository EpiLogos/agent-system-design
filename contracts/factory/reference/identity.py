"""Executable reference for canonical Factory identity semantics.

`Ref` is semantic identity. Paths, provider IDs, sessions and projection IDs
may point at it; none of them define it.
"""
from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
import json
import re
from typing import Any, Iterable, Mapping

_REF = re.compile(r"^factory:([a-z][a-z0-9-]*):([A-Za-z0-9][A-Za-z0-9._-]*)$")


class IdentityError(ValueError):
    pass


class InvalidRef(IdentityError):
    pass


class IdentityConflict(IdentityError):
    pass


class UnknownRef(IdentityError):
    pass


class StaleRevision(IdentityError):
    pass


class TombstonedIdentity(IdentityError):
    pass


class ProjectionIdentityLoss(IdentityError):
    pass


@dataclass(frozen=True, order=True)
class Ref:
    kind: str
    id: str

    @classmethod
    def parse(cls, value: str) -> "Ref":
        match = _REF.fullmatch(value)
        if not match:
            raise InvalidRef(f"invalid canonical Ref: {value!r}")
        return cls(match.group(1), match.group(2))

    @classmethod
    def make(cls, kind: str, opaque_id: str) -> "Ref":
        return cls.parse(f"factory:{kind}:{opaque_id}")

    def __str__(self) -> str:
        return f"factory:{self.kind}:{self.id}"


@dataclass(frozen=True)
class IdentityRecord:
    ref: Ref
    revision: int
    aliases: tuple[str, ...]
    tombstoned: bool
    payload: Any
    tombstone_reason: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "ref": str(self.ref),
            "revision": self.revision,
            "aliases": list(self.aliases),
            "tombstoned": self.tombstoned,
            "tombstoneReason": self.tombstone_reason,
            "payload": deepcopy(self.payload),
        }


class IdentityStore:
    """Deterministic semantic reference model; not a persistence choice."""

    SCHEMA_VERSION = "factory.identity-store/v1"

    def __init__(self) -> None:
        self._records: dict[Ref, IdentityRecord] = {}
        self._aliases: dict[str, Ref] = {}
        self._retired: set[Ref] = set()

    @staticmethod
    def _payload(value: Any) -> Any:
        try:
            return json.loads(json.dumps(value, sort_keys=True))
        except (TypeError, ValueError) as error:
            raise IdentityError("payload must be JSON-serializable") from error

    @staticmethod
    def _aliases_normalized(values: Iterable[str]) -> tuple[str, ...]:
        result: set[str] = set()
        for value in values:
            if not isinstance(value, str) or not value.strip():
                raise IdentityError("alias must be a non-empty string")
            value = value.strip()
            if value.startswith("factory:"):
                raise IdentityError("alias cannot use canonical Ref syntax")
            result.add(value)
        return tuple(sorted(result))

    def _aliases_available(self, ref: Ref, aliases: Iterable[str]) -> None:
        for alias in aliases:
            owner = self._aliases.get(alias)
            if owner is not None and owner != ref:
                raise IdentityConflict(f"alias already belongs to {owner}")

    def create(self, ref: Ref | str, payload: Any, aliases: Iterable[str] = ()) -> IdentityRecord:
        ref = Ref.parse(ref) if isinstance(ref, str) else ref
        if ref in self._records or ref in self._retired:
            raise IdentityConflict(f"canonical Ref cannot be reused: {ref}")
        aliases = self._aliases_normalized(aliases)
        self._aliases_available(ref, aliases)
        record = IdentityRecord(ref, 1, aliases, False, self._payload(payload))
        self._records[ref] = record
        for alias in aliases:
            self._aliases[alias] = ref
        return record

    def resolve_ref(self, value: Ref | str) -> Ref:
        if isinstance(value, Ref):
            return value
        if value.startswith("factory:"):
            return Ref.parse(value)
        if value not in self._aliases:
            raise UnknownRef(f"unknown Ref or alias: {value!r}")
        return self._aliases[value]

    def get(self, value: Ref | str) -> IdentityRecord:
        ref = self.resolve_ref(value)
        if ref not in self._records:
            raise UnknownRef(f"unknown canonical Ref: {ref}")
        return self._records[ref]

    @staticmethod
    def _writable(record: IdentityRecord, expected_revision: int) -> None:
        if record.revision != expected_revision:
            raise StaleRevision(
                f"stale write for {record.ref}: expected {expected_revision}, current {record.revision}"
            )
        if record.tombstoned:
            raise TombstonedIdentity(f"identity is tombstoned: {record.ref}")

    def update(self, value: Ref | str, *, expected_revision: int, payload: Any) -> IdentityRecord:
        current = self.get(value)
        self._writable(current, expected_revision)
        updated = IdentityRecord(
            current.ref, current.revision + 1, current.aliases, False, self._payload(payload)
        )
        self._records[current.ref] = updated
        return updated

    def add_alias(self, value: Ref | str, alias: str, *, expected_revision: int) -> IdentityRecord:
        current = self.get(value)
        self._writable(current, expected_revision)
        aliases = self._aliases_normalized((*current.aliases, alias))
        self._aliases_available(current.ref, aliases)
        updated = IdentityRecord(
            current.ref, current.revision + 1, aliases, False, deepcopy(current.payload)
        )
        self._records[current.ref] = updated
        for item in aliases:
            self._aliases[item] = current.ref
        return updated

    def tombstone(self, value: Ref | str, *, expected_revision: int, reason: str) -> IdentityRecord:
        current = self.get(value)
        self._writable(current, expected_revision)
        if not isinstance(reason, str) or not reason.strip():
            raise IdentityError("tombstone reason is required")
        record = IdentityRecord(
            current.ref,
            current.revision + 1,
            current.aliases,
            True,
            deepcopy(current.payload),
            reason.strip(),
        )
        self._records[current.ref] = record
        self._retired.add(current.ref)
        return record

    def resolve_projection(self, projection: Mapping[str, Any]) -> IdentityRecord:
        canonical = projection.get("canonicalRef")
        if not isinstance(canonical, str):
            raise ProjectionIdentityLoss("projection lost canonicalRef")
        try:
            return self.get(Ref.parse(canonical))
        except (InvalidRef, UnknownRef) as error:
            raise ProjectionIdentityLoss("projection cannot recover canonical identity") from error

    def to_dict(self) -> dict[str, Any]:
        return {
            "schemaVersion": self.SCHEMA_VERSION,
            "records": [self._records[ref].to_dict() for ref in sorted(self._records)],
            "retiredRefs": [str(ref) for ref in sorted(self._retired)],
        }

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), sort_keys=True, separators=(",", ":"))

    @classmethod
    def from_dict(cls, data: Mapping[str, Any]) -> "IdentityStore":
        if data.get("schemaVersion") != cls.SCHEMA_VERSION:
            raise IdentityError("unsupported identity-store schemaVersion")
        store = cls()
        for raw in data.get("records", []):
            ref = Ref.parse(raw["ref"])
            if ref in store._records:
                raise IdentityConflict(f"duplicate canonical Ref: {ref}")
            revision = raw.get("revision")
            if not isinstance(revision, int) or revision < 1:
                raise IdentityError(f"invalid revision for {ref}")
            aliases = store._aliases_normalized(raw.get("aliases", []))
            store._aliases_available(ref, aliases)
            tombstoned = raw.get("tombstoned") is True
            reason = raw.get("tombstoneReason")
            if tombstoned and (not isinstance(reason, str) or not reason.strip()):
                raise IdentityError(f"tombstone reason missing for {ref}")
            record = IdentityRecord(
                ref,
                revision,
                aliases,
                tombstoned,
                store._payload(raw.get("payload")),
                reason,
            )
            store._records[ref] = record
            for alias in aliases:
                store._aliases[alias] = ref
            if tombstoned:
                store._retired.add(ref)
        retired = {Ref.parse(value) for value in data.get("retiredRefs", [])}
        if retired != store._retired:
            raise IdentityError("retiredRefs must exactly match tombstoned records")
        return store

    @classmethod
    def from_json(cls, value: str) -> "IdentityStore":
        try:
            data = json.loads(value)
        except json.JSONDecodeError as error:
            raise IdentityError("invalid identity-store JSON") from error
        if not isinstance(data, dict):
            raise IdentityError("identity-store serialization must be an object")
        return cls.from_dict(data)
