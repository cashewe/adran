from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass
class RecallEntry:
    """Mirror to the rust object of the same name."""

    heading: str | None
    body_range: tuple[int, int] | None
    depth: int
    filtered: bool = False

    @classmethod
    def from_json(cls, data: dict[str, Any]) -> "RecallEntry":
        body_range = data.get("body_range")
        return cls(
            heading=data.get("heading"),
            body_range=(body_range["start"], body_range["end"]) if body_range else None,
            depth=data["depth"],
            filtered=data.get("filtered", False),
        )

    def __repr__(self) -> str:
        return f"RecallEntry(heading={self.heading!r}, body_range={self.body_range!r}, depth={self.depth!r})"


@dataclass
class Entries:
    entries: list[RecallEntry]
    md_filepath: str = ""

    def __len__(self):
        return len(self.entries)

    def __repr__(self) -> str:
        return f"Entries(entries={self.entries!r}, md_filepath={self.md_filepath!r})"

    def __getitem__(self, key):
        return self.entries[key]

    def __setitem__(self, key, value):
        self.entries[key] = value

    def __delitem__(self, key):
        del self.entries[key]

    def rehydrate_range(
        self,
        md_filepath: str | None = None,
        show_filtered_ranges: bool = False,
    ) -> str:
        text_path = self._get_filepath(md_filepath)
        with open(text_path, "r", encoding="utf-8") as f:
            content = f.read()

        content_bytes = content.encode('utf-8')

        output = ""
        for entry in self.entries:
            output += ('#' * entry.depth) + entry.heading + "\n"
            if entry.body_range:
                start, end = entry.body_range
                if entry.filtered:
                    if show_filtered_ranges:
                        output += f"[{start}:{end}]\n\n"
                    else:
                        output += '...\n\n'
                else:
                    text_chunk = content_bytes[start:end].decode('utf-8')
                    output += text_chunk
            else:
                output += '\n\n'
        
        return output
        

    def _get_filepath(self, md_filepath: str | None) -> str:
        if md_filepath:
            return md_filepath
        elif self.md_filepath:
            return self.md_filepath
        else:
            raise ValueError("md_filepath must be provided either in the method call or in the Entries object.")