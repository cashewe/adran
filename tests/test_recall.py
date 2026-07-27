from __future__ import annotations

from pathlib import Path

import pytest

from adran import Node, Parser, RecallEntry

FIXTURE = {
    "root_id": "Root-0",
    "source_len": 3445,
    "nodes": [
        {
            "id": "Root-0", "mdast_type": "Root",
            "range": {"start": 0, "end": 3445},
            "parent": None, "children": ["Section-0"],
            "depth": 0, "heading": "Root",
        },
        {
            "id": "Section-0", "mdast_type": "Section",
            "range": {"start": 0, "end": 3445},
            "parent": "Root-0",
            "children": ["Paragraph-0", "Paragraph-1", "Section-1", "Section-3"],
            "depth": 1, "heading": "adran",
        },
        {
            "id": "Paragraph-0", "mdast_type": "Paragraph",
            "range": {"start": 9, "end": 49},
            "parent": "Section-0", "children": [],
            "depth": 2, "heading": "Paragraph",
        },
        {
            "id": "Paragraph-1", "mdast_type": "Paragraph",
            "range": {"start": 51, "end": 504},
            "parent": "Section-0", "children": [],
            "depth": 2, "heading": "Paragraph",
        },
        {
            "id": "Section-1", "mdast_type": "Section",
            "range": {"start": 506, "end": 1859},
            "parent": "Section-0",
            "children": ["List-0", "Code-0", "List-1", "Paragraph-2", "Section-2"],
            "depth": 2, "heading": "jist",
        },
        {
            "id": "List-0", "mdast_type": "List",
            "range": {"start": 515, "end": 599},
            "parent": "Section-1", "children": [],
            "depth": 3, "heading": "List",
        },
        {
            "id": "Code-0", "mdast_type": {"Other": "Code"},
            "range": {"start": 600, "end": 742},
            "parent": "Section-1", "children": [],
            "depth": 3, "heading": "Code",
        },
        {
            "id": "List-1", "mdast_type": "List",
            "range": {"start": 744, "end": 871},
            "parent": "Section-1", "children": [],
            "depth": 3, "heading": "List",
        },
        {
            "id": "Paragraph-2", "mdast_type": "Paragraph",
            "range": {"start": 872, "end": 1061},
            "parent": "Section-1", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "Section-2", "mdast_type": "Section",
            "range": {"start": 1063, "end": 1859},
            "parent": "Section-1",
            "children": ["List-2", "Paragraph-3", "Paragraph-4"],
            "depth": 3, "heading": "extra bits:",
        },
        {
            "id": "List-2", "mdast_type": "List",
            "range": {"start": 1079, "end": 1392},
            "parent": "Section-2", "children": [],
            "depth": 4, "heading": "List",
        },
        {
            "id": "Paragraph-3", "mdast_type": "Paragraph",
            "range": {"start": 1393, "end": 1549},
            "parent": "Section-2", "children": [],
            "depth": 4, "heading": "Paragraph",
        },
        {
            "id": "Paragraph-4", "mdast_type": "Paragraph",
            "range": {"start": 1551, "end": 1859},
            "parent": "Section-2", "children": [],
            "depth": 4, "heading": "Paragraph",
        },
        {
            "id": "Section-3", "mdast_type": "Section",
            "range": {"start": 1861, "end": 3445},
            "parent": "Section-0",
            "children": [
                "Paragraph-5", "Paragraph-6", "Code-1", "Paragraph-7",
                "List-3", "Paragraph-8", "List-4", "Paragraph-9", "Paragraph-10",
            ],
            "depth": 2, "heading": "jist 2.0: you can (not) change the jist",
        },
        {
            "id": "Paragraph-5", "mdast_type": "Paragraph",
            "range": {"start": 1905, "end": 1950},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "Paragraph-6", "mdast_type": "Paragraph",
            "range": {"start": 1952, "end": 2011},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "Code-1", "mdast_type": {"Other": "Code"},
            "range": {"start": 2013, "end": 2071},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Code",
        },
        {
            "id": "Paragraph-7", "mdast_type": "Paragraph",
            "range": {"start": 2073, "end": 2168},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "List-3", "mdast_type": "List",
            "range": {"start": 2169, "end": 2400},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "List",
        },
        {
            "id": "Paragraph-8", "mdast_type": "Paragraph",
            "range": {"start": 2401, "end": 2434},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "List-4", "mdast_type": "List",
            "range": {"start": 2436, "end": 2798},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "List",
        },
        {
            "id": "Paragraph-9", "mdast_type": "Paragraph",
            "range": {"start": 2799, "end": 2953},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
        {
            "id": "Paragraph-10", "mdast_type": "Paragraph",
            "range": {"start": 2955, "end": 3445},
            "parent": "Section-3", "children": [],
            "depth": 3, "heading": "Paragraph",
        },
    ],
}

# Lands inside Paragraph-3, inside Section-2 ("extra bits:"). Ancestor chain
# from the match upward is: "extra bits:" -> "jist" -> "adran". "jist" has a
# sibling section, "jist 2.0: you can (not) change the jist".
MATCH_START = 1450
MATCH_END = 1460


def _parser() -> Parser:
    """A Parser preloaded with the fixture document, without touching disk -
    `recall_text_indices` only needs `self.node`, so we skip `.parse()`."""
    parser = Parser.__new__(Parser)
    parser.markdown_path = Path("unused.md")
    parser = Parser(node=FIXTURE)
    return parser


def _headings(entries: list[RecallEntry]) -> list[str]:
    return [e.heading for e in entries]


class TestRecallTextIndices:
    def test_full_ancestor_chain_no_siblings(self):
        """text_depth=None, heading_depth=None, no siblings: every real
        ancestor heading gets a full entry, in document order (outermost
        first, since it starts earliest in the source)."""
        entries = _parser().recall_text_indices(MATCH_START, MATCH_END)

        assert _headings(entries) == ["adran", "jist", "extra bits:"]
        assert entries[0].body_range == (9, 3445)
        assert entries[1].body_range == (515, 1859)
        assert entries[2].body_range == (1079, 1859)

    def test_siblings_ignored_by_default(self):
        """"jist 2.0..." is a sibling of "jist" - with both sibling flags
        off it must never appear, no matter how far we climb."""
        entries = _parser().recall_text_indices(MATCH_START, MATCH_END)

        assert "jist 2.0: you can (not) change the jist" not in _headings(entries)

    def test_heading_siblings_show_headings_without_text(self):
        """heading_siblings=True, text_siblings=False: the sibling section
        shows up heading-only, positioned where it actually sits in the
        document (after "extra bits:", since it starts later)."""
        entries = _parser().recall_text_indices(
            MATCH_START, MATCH_END, heading_siblings=True,
        )

        assert _headings(entries) == [
            "adran",
            "jist",
            "extra bits:",
            "jist 2.0: you can (not) change the jist",
        ]
        assert entries[-1].body_range is None  # sibling: heading only
        # the actual match chain keeps its text
        assert entries[0].body_range == (9, 3445)
        assert entries[1].body_range == (515, 1859)
        assert entries[2].body_range == (1079, 1859)

    def test_text_siblings_show_full_sibling_entries(self):
        """text_siblings=True: the sibling section comes back as a full
        entry - heading *and* body_range, not just a heading."""
        entries = _parser().recall_text_indices(
            MATCH_START, MATCH_END, text_siblings=True,
        )

        sibling = next(
            e for e in entries
            if e.heading == "jist 2.0: you can (not) change the jist"
        )
        assert sibling.body_range == (1905, 3445)

    def test_heading_siblings_still_shown_beyond_text_depth(self):
        """When heading_depth reaches further than text_depth, sibling
        headings still appear (heading-only) at those deeper levels - even
        with text_siblings=True, which simply has nothing to contribute
        once text_depth is exhausted."""
        entries = _parser().recall_text_indices(
            MATCH_START,
            MATCH_END,
            text_depth=0,
            heading_depth=1,
            text_siblings=True,
            heading_siblings=True,
        )

        assert _headings(entries) == [
            "jist",
            "extra bits:",
            "jist 2.0: you can (not) change the jist",
        ]
        # the match itself is still fully resolved
        assert entries[1].heading == "extra bits:"
        assert entries[1].body_range == (1079, 1859)
        # one level up: both the ancestor and its sibling are heading-only,
        # despite text_siblings=True, because text_depth stopped at 0
        assert entries[0].body_range is None
        assert entries[2].body_range is None

    def test_text_depth_zero_limits_to_match_only(self):
        """text_depth=0 with heading_depth=0 too: nothing above the match
        is included at all."""
        entries = _parser().recall_text_indices(
            MATCH_START, MATCH_END, text_depth=0, heading_depth=0,
        )

        assert _headings(entries) == ["extra bits:"]
        assert entries[0].body_range == (1079, 1859)

    def test_text_depth_none_is_unlimited(self):
        """text_depth=None pulls in the full ancestor chain with full text,
        regardless of whatever heading_depth is set to."""
        entries = _parser().recall_text_indices(
            MATCH_START, MATCH_END, text_depth=None, heading_depth=0,
        )

        assert _headings(entries) == ["adran", "jist", "extra bits:"]
        assert all(e.body_range is not None for e in entries)

    def test_heading_depth_none_is_unlimited(self):
        """heading_depth=None pulls in the full ancestor chain as headings,
        even when text_depth stops right at the match."""
        entries = _parser().recall_text_indices(
            MATCH_START, MATCH_END, text_depth=0, heading_depth=None,
        )

        assert _headings(entries) == ["adran", "jist", "extra bits:"]
        assert entries[0].body_range is None            # adran: heading only
        assert entries[1].body_range is None             # jist: heading only
        assert entries[2].body_range == (1079, 1859)      # extra bits: the match itself

    def test_no_match_returns_empty(self):
        """A span outside every node's range has nothing to attach to."""
        entries = _parser().recall_text_indices(10_000, 10_010)

        assert entries == []