import json
import os
from pathlib import Path
from typing import Union
 
from . import adran as _native
from ._errors import MarkdownParseError, NodeMismatchError
from .node import Node
from .recall_entry import RecallEntry
 
 
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
 
    def __init__(self, markdown_path: str | os.PathLike[str] | None = None, node: str | None = None):
        self.markdown_path = Path(markdown_path) if markdown_path else None
        self.node = node
 
    def parse(self) -> Node:
        """Parses `markdown_path`, stores the result on `self.node`, and
        returns it."""
        if not self.markdown_path:
            raise ValueError("Parser was constructed without a markdown_path, cannot parse!")

        source = self._read_source()
        try:
            raw_json = _native.parse_markdown(source)
        except ValueError as exc:
            raise MarkdownParseError(str(exc)) from exc
 
        self.node = raw_json
        return Node.from_json(self.node)

    def recall_text_indices(
        self,
        start: int,
        end: int,
        text_depth: int | None = None,
        heading_depth: int | None = None,
        text_siblings: bool = False,
        heading_siblings: bool = False,
    ) -> list[RecallEntry]:
        """Finds the section (and body range) containing the [start, end)
        span, using the already-parsed `self.node`.

        Parameters
        ----------
        start: the start idx of the snippet of text you are looking for.
        end: the end idx of the snippet of text you are looking for.
        text_depth: how far up the text tree to include in the rehydration (leave None to take the full text).
        heading_depth: how far up the heading tree to include in the rehydration (leave None to take the full heading heirarchy).
        text_siblings: whether to include sibling text nodes within the stated depth.
        heading_siblings: whether to include sibling headings within the stated depth.
        """
        if not self.node:
            raise RuntimeError(
                "no parsed document available - call .parse() before .recall_text_indices()"
            )

        raw_json = _native.run_recall_text_indices(
            self.node,
            start,
            end,
            text_depth,
            heading_depth,
            text_siblings,
            heading_siblings,
        )
        return [RecallEntry.from_json(entry) for entry in json.loads(raw_json)]
  
    def _read_source(self) -> str:
        return self.markdown_path.read_text(encoding="utf-8")