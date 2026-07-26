from __future__ import annotations
 
import json
from dataclasses import dataclass, field
from typing import Any, Iterable, Iterator
 
 
@dataclass(frozen=True)
class Kind:
    """A node's kind, e.g. `Root`, `Section`, `Paragraph`, or an `Other`.
    """
 
    name: str
    is_other: bool = False
 
    @classmethod
    def from_json(cls, value: Any) -> "Kind":
        if isinstance(value, dict):
            return cls(name=value["Other"], is_other=True)
        return cls(name=value, is_other=False)
 
    def to_json(self) -> Any:
        return {"Other": self.name} if self.is_other else self.name
 
    def __str__(self) -> str:  # noqa: D105
        return self.name
 
 
@dataclass
class NodeRecord:
    """One entry from the flat node list - mirrors the Rust `NodeType`
    struct field-for-field."""
 
    id: str
    kind: Kind
    start: int
    end: int
    parent: str | None
    children: list[str]
    depth: int
    heading: str
 
    @classmethod
    def from_json(cls, data: dict[str, Any]) -> "NodeRecord":
        range_ = data["range"]
        return cls(
            id=data["id"],
            kind=Kind.from_json(data["mdast_type"]),
            start=range_["start"],
            end=range_["end"],
            parent=data.get("parent"),
            children=list(data.get("children", [])),
            depth=data["depth"],
            heading=data["heading"],
        )
 
    def to_json(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "mdast_type": self.kind.to_json(),
            "range": {"start": self.start, "end": self.end},
            "parent": self.parent,
            "children": list(self.children),
            "depth": self.depth,
            "heading": self.heading,
        }
 
 
class Node:
    """A parsed document: the flat node list plus enough bookkeeping to
    walk it as a tree.
 
    This wraps the *output* of parsing (JSON), not a live handle into the
    Rust object - once built it has no further dependency on the
    extension module, so it round-trips through storage cleanly (see
    `to_json`/`from_json`) and is safe to construct in tests without a
    Rust build.
    """
 
    def __init__(self, root_id: str, source_len: int, nodes: Iterable[NodeRecord]):
        self.root_id = root_id
        self.source_len = source_len
        self._by_id: dict[str, NodeRecord] = {n.id: n for n in nodes}
 
        if root_id not in self._by_id:
            raise ValueError(f"root_id {root_id!r} is not present in the node list")
  
    @classmethod
    def from_json(cls, data: str | dict[str, Any]) -> "Node":
        payload = json.loads(data) if isinstance(data, str) else data
        nodes = (NodeRecord.from_json(n) for n in payload["nodes"])
        return cls(root_id=payload["root_id"], source_len=payload["source_len"], nodes=nodes)
 
    def to_json(self) -> str:
        payload = {
            "root_id": self.root_id,
            "source_len": self.source_len,
            "nodes": [n.to_json() for n in self.nodes],
        }
        return json.dumps(payload)
  
    @property
    def nodes(self) -> list[NodeRecord]:
        return list(self._by_id.values())
 
    @property
    def root(self) -> NodeRecord:
        return self._by_id[self.root_id]
 
    def get(self, node_id: str) -> NodeRecord | None:
        return self._by_id.get(node_id)
 
    def children_of(self, node_id: str) -> list[NodeRecord]:
        node = self._by_id.get(node_id)
        if node is None:
            return []
        return [self._by_id[c] for c in node.children if c in self._by_id]
 
    @property
    def depth(self) -> int:
        """Highest `depth` value present in the tree."""
        return max((n.depth for n in self._by_id.values()), default=0)
  
    def __len__(self) -> int:
        """Number of nodes in the document (Root + every Section/leaf)."""
        return len(self._by_id)
 
    def __iter__(self) -> Iterator[NodeRecord]:
        return iter(self.nodes)
 
    def __str__(self) -> str:
        """A tree display of the section headings only, indented one level
        per nesting depth. Leaf content (paragraphs, tables, ...) is
        omitted - use `.nodes` for the full flat list."""
        lines = self._render_section_tree(self.root_id, indent=0)
        return "\n".join(lines) if lines else "(no sections)"
 
    def __repr__(self) -> str:
        return f"Node(root_id={self.root_id!r}, nodes={len(self)}, depth={self.depth})"
 
    def _render_section_tree(self, node_id: str, indent: int) -> list[str]:
        node = self._by_id.get(node_id)
        if node is None:
            return []
 
        lines: list[str] = []
        next_indent = indent
        if node.kind.name == "Section":
            lines.append(("  " * indent) + node.heading)
            next_indent = indent + 1
 
        for child in node.children:
            lines.extend(self._render_section_tree(child, next_indent))
        return lines
