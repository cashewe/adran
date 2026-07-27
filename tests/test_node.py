"""SLOP generated tests for the Python `Node` class.

prompt:

please write unit tests on the python functionality that cover all the core requirements
ive given you. dont test any implementation details, only overall behaviours. your tests
should implicitly cover the rust code by testing the python interface, but i only care to
test the python part as that is the user facing bit.

your tests should be modularised for the node and for the parser separately.
"""

from __future__ import annotations
 
import json
 
import pytest
 
from adran import Node
 
 
def make_node_json(
    id: str,
    kind,
    start: int,
    end: int,
    depth: int,
    heading: str,
    parent: str | None = None,
    children: list[str] | None = None,
) -> dict:
    """Builds one flat node entry matching the schema `Node.from_json`
    expects. `kind` may be a plain string (e.g. "Section") or a dict like
    `{"Other": "Code"}`, mirroring how the Rust side serializes
    `MdastType`."""
    return {
        "id": id,
        "mdast_type": kind,
        "range": {"start": start, "end": end},
        "parent": parent,
        "children": children or [],
        "depth": depth,
        "heading": heading,
    }
 
 
def make_payload(nodes: list[dict], root_id: str = "Root-0", source_len: int = 1000) -> dict:
    return {"root_id": root_id, "source_len": source_len, "nodes": nodes}
 
 
@pytest.fixture
def nested_document_payload() -> dict:
    """A small document tree, four levels deep:
 
    Root
      Section "Top Level" (depth 1)
        Paragraph                        (depth 2)
        Section "Child Section" (depth 2)
          Paragraph                      (depth 3)
          Section "Grandchild" (depth 3)
            Paragraph                    (depth 4)
        Section "Second Child" (depth 2)
          Paragraph                      (depth 3)
    """
    nodes = [
        make_node_json("Root-0", "Root", 0, 1000, depth=0, heading="Root",
                        children=["Section-0"]),
        make_node_json("Section-0", "Section", 0, 900, depth=1, heading="Top Level",
                        parent="Root-0",
                        children=["Paragraph-0", "Section-1", "Section-2"]),
        make_node_json("Paragraph-0", "Paragraph", 20, 40, depth=2, heading="Paragraph",
                        parent="Section-0"),
        make_node_json("Section-1", "Section", 60, 500, depth=2, heading="Child Section",
                        parent="Section-0", children=["Paragraph-1", "Section-3"]),
        make_node_json("Paragraph-1", "Paragraph", 80, 100, depth=3, heading="Paragraph",
                        parent="Section-1"),
        make_node_json("Section-3", "Section", 120, 480, depth=3, heading="Grandchild",
                        parent="Section-1", children=["Paragraph-2"]),
        make_node_json("Paragraph-2", "Paragraph", 140, 160, depth=4, heading="Paragraph",
                        parent="Section-3"),
        make_node_json("Section-2", "Section", 520, 900, depth=2, heading="Second Child",
                        parent="Section-0", children=["Paragraph-3"]),
        make_node_json("Paragraph-3", "Paragraph", 540, 560, depth=3, heading="Paragraph",
                        parent="Section-2"),
    ]
    return make_payload(nodes)
 
 
class TestJsonRoundTrip:
    def test_from_json_accepts_a_json_string(self, nested_document_payload):
        node = Node.from_json(json.dumps(nested_document_payload))
        assert len(node) == len(nested_document_payload["nodes"])
 
    def test_to_json_produces_valid_json(self, nested_document_payload):
        node = Node.from_json(nested_document_payload)
        parsed = json.loads(node.to_json())
        assert parsed["root_id"] == nested_document_payload["root_id"]
        assert parsed["source_len"] == nested_document_payload["source_len"]
        assert len(parsed["nodes"]) == len(nested_document_payload["nodes"])
 
    def test_round_trip_is_lossless(self, nested_document_payload):
        original = Node.from_json(nested_document_payload)
        round_tripped = Node.from_json(original.to_json())
 
        assert round_tripped.to_json() == original.to_json()
        assert len(round_tripped) == len(original)
        assert round_tripped.depth == original.depth
        assert str(round_tripped) == str(original)
 
    def test_round_trip_preserves_other_kind_fallback_nodes(self):
        payload = make_payload([
            make_node_json("Root-0", "Root", 0, 50, depth=0, heading="Root",
                            children=["Other-0"]),
            make_node_json("Other-0", {"Other": "Code"}, 0, 50, depth=1, heading="Code",
                            parent="Root-0"),
        ])
        node = Node.from_json(payload)
        round_tripped = Node.from_json(node.to_json())
        assert round_tripped.to_json() == node.to_json()
 
 
class TestLen:
    def test_len_returns_total_number_of_nodes(self, nested_document_payload):
        node = Node.from_json(nested_document_payload)
        assert len(node) == 9
 
    def test_len_of_a_document_with_only_a_root(self):
        payload = make_payload([make_node_json("Root-0", "Root", 0, 0, depth=0, heading="Root")])
        node = Node.from_json(payload)
        assert len(node) == 1
 
 
class TestDepth:
    def test_depth_is_the_highest_depth_value_in_the_tree(self, nested_document_payload):
        node = Node.from_json(nested_document_payload)
        assert node.depth == 4
 
    def test_depth_is_zero_for_a_root_only_document(self):
        payload = make_payload([make_node_json("Root-0", "Root", 0, 0, depth=0, heading="Root")])
        node = Node.from_json(payload)
        assert node.depth == 0
 
 
class TestStringRepresentation:
    def test_str_shows_only_section_headings_indented_by_nesting(self, nested_document_payload):
        node = Node.from_json(nested_document_payload)
 
        expected = (
            "Top Level\n"
            "  Child Section\n"
            "    Grandchild\n"
            "  Second Child"
        )
        assert str(node) == expected
 
    def test_str_omits_non_section_content(self, nested_document_payload):
        node = Node.from_json(nested_document_payload)
        rendered = str(node)
        # Leaf kinds' generic labels must never show up in the heading tree.
        assert "Paragraph" not in rendered
 
    def test_str_of_a_document_with_no_sections_is_empty_or_placeholder(self):
        payload = make_payload([
            make_node_json("Root-0", "Root", 0, 40, depth=0, heading="Root",
                            children=["Paragraph-0"]),
            make_node_json("Paragraph-0", "Paragraph", 0, 40, depth=1, heading="Paragraph",
                            parent="Root-0"),
        ])
        node = Node.from_json(payload)
        assert "Paragraph" not in str(node)
