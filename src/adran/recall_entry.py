from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass
class RecallEntry:
    """Mirror to the rust object of the same name."""

    heading: str | None
    body_range: tuple[int, int] | None
    depth: int

    @classmethod
    def from_json(cls, data: dict[str, Any]) -> "RecallEntry":
        body_range = data.get("body_range")
        return cls(
            heading=data.get("heading"),
            body_range=(body_range["start"], body_range["end"]) if body_range else None,
            depth=data["depth"]
        )

    def __repr__(self) -> str:
        return f"RecallEntry(heading={self.heading!r}, body_range={self.body_range!r}, depth={self.depth!r})"