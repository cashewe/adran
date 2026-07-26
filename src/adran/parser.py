from __future__ import annotations
 
import os
from pathlib import Path
from typing import Union
 
from . import adran as _native
from ._errors import MarkdownParseError, NodeMismatchError
from .node import Node
 
NodeSource = Union[Node, str, "os.PathLike[str]"]
 
 
class Parser:
    """Parses a single markdown file into a `Node`.
 
    Parameters
    ----------
    markdown_path:
        Path to the markdown file to parse.
    node:
        Optional. Either an existing `Node`, or a path to a JSON file
        produced by `Node.to_json`. If given, its root span is checked
        against `markdown_path` at construction time as a sanity check
        that the two are talking about the same document (see
        `_validate_against`). If omitted, no check is performed.
 
        Note this is *only* used for that one-time validation - it is
        never reused as the result. `.parse()` always re-parses
        `markdown_path` from scratch via the Rust core.
    """
 
    def __init__(self, markdown_path: str | os.PathLike[str], node: NodeSource | None = None):
        self.markdown_path = Path(markdown_path)
        self.node: Node | None = None
 
        if node is not None:
            self._validate_against(self._resolve_node(node))
 
    def parse(self) -> Node:
        """Parses `markdown_path`, stores the result on `self.node`, and
        returns it."""
        source = self._read_source()
        try:
            raw_json = _native.parse_markdown(source)
        except ValueError as exc:
            raise MarkdownParseError(str(exc)) from exc
 
        self.node = Node.from_json(raw_json)
        return self.node
  
    def _resolve_node(self, node: NodeSource) -> Node:
        if isinstance(node, Node):
            return node
        return Node.from_json(Path(node).read_text(encoding="utf-8"))
 
    def _validate_against(self, candidate: Node) -> None:
        """Ensures the Node is valid repr of the markdown file.
        
        NOTE
        ----
        Rust uses byte offsets for node spans, while Python uses character offsets.
        This code appears more complex than excpeted as it has to account for this.
        """
        source_bytes = len(self._read_source().encode("utf-8"))
        root_span = candidate.root.end - candidate.root.start
 
        if root_span != source_bytes:
            raise NodeMismatchError(
                f"the given node's root span is {root_span} bytes, but "
                f"{self.markdown_path} is {source_bytes} bytes (UTF-8) - "
                "these don't look like the same document."
            )
 
    def _read_source(self) -> str:
        return self.markdown_path.read_text(encoding="utf-8")
