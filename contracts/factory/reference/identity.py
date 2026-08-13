"""Reference implementation of the Factory identity contract."""

from dataclasses import dataclass


@dataclass(frozen=True)
class Ref:
    kind: str
    id: str

    def __str__(self) -> str:
        return f"factory:{self.kind}:{self.id}"
