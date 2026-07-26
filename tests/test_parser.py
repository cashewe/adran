"""SLOP generated tests for the Python `Parser` class.

prompt:

please write unit tests on the python functionality that cover all the core requirements
ive given you. dont test any implementation details, only overall behaviours. your tests
should implicitly cover the rust code by testing the python interface, but i only care to
test the python part as that is the user facing bit.

your tests should be modularised for the node and for the parser separately.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from mdparser import Node, NodeMismatchError, Parser

SAMPLE_MARKDOWN = """\
# Top Level

Intro paragraph.

## Child Section

Some content here.

## Second Child

More content.
"""

OTHER_MARKDOWN = """\
# A totally different, much shorter document.
"""


@pytest.fixture
def markdown_file(tmp_path: Path) -> Path:
    path = tmp_path / "doc.md"
    path.write_text(SAMPLE_MARKDOWN, encoding="utf-8")
    return path


@pytest.fixture
def other_markdown_file(tmp_path: Path) -> Path:
    path = tmp_path / "other.md"
    path.write_text(OTHER_MARKDOWN, encoding="utf-8")
    return path


class TestConstruction:
    def test_accepts_a_string_path(self, markdown_file: Path):
        Parser(str(markdown_file))  # must not raise

    def test_accepts_a_path_object(self, markdown_file: Path):
        Parser(markdown_file)  # must not raise

    def test_with_no_node_argument_performs_no_validation(self, tmp_path: Path):
        # A path that doesn't even exist yet is fine at construction time,
        # since nothing is read or validated until parse() is called.
        Parser(tmp_path / "does_not_exist.md")

    def test_node_attribute_is_unset_until_parse_is_called(self, markdown_file: Path):
        parser = Parser(markdown_file)
        assert parser.node is None


class TestParse:
    def test_parse_returns_a_node(self, markdown_file: Path):
        parser = Parser(markdown_file)
        result = parser.parse()
        assert isinstance(result, Node)

    def test_parse_sets_the_node_attribute_to_its_return_value(self, markdown_file: Path):
        parser = Parser(markdown_file)
        result = parser.parse()
        assert parser.node is result

    def test_parsed_node_reflects_the_document_structure(self, markdown_file: Path):
        node = Parser(markdown_file).parse()
        assert "Top Level" in str(node)
        assert "Child Section" in str(node)
        assert "Second Child" in str(node)
        assert len(node) > 0
        assert node.depth > 0

    def test_parse_always_produces_a_fresh_node_object(self, markdown_file: Path):
        # Even when a Node was supplied at construction (for validation
        # purposes only), parse() must not simply hand that object back.
        preexisting_node = Parser(markdown_file).parse()

        parser = Parser(markdown_file, node=preexisting_node)
        fresh_node = parser.parse()

        assert fresh_node is not preexisting_node
        assert parser.node is fresh_node


class TestNodeValidation:
    def test_construction_succeeds_when_node_object_matches_the_file(self, markdown_file: Path):
        matching_node = Parser(markdown_file).parse()
        Parser(markdown_file, node=matching_node)  # must not raise

    def test_construction_succeeds_when_node_json_file_matches(
        self, markdown_file: Path, tmp_path: Path
    ):
        matching_node = Parser(markdown_file).parse()
        node_json_path = tmp_path / "structure.json"
        node_json_path.write_text(matching_node.to_json(), encoding="utf-8")

        Parser(markdown_file, node=node_json_path)  # must not raise

    def test_construction_raises_when_node_does_not_match_the_file(
        self, markdown_file: Path, other_markdown_file: Path
    ):
        mismatched_node = Parser(other_markdown_file).parse()

        with pytest.raises(NodeMismatchError):
            Parser(markdown_file, node=mismatched_node)

    def test_construction_raises_when_node_json_file_does_not_match(
        self, markdown_file: Path, other_markdown_file: Path, tmp_path: Path
    ):
        mismatched_node = Parser(other_markdown_file).parse()
        node_json_path = tmp_path / "structure.json"
        node_json_path.write_text(mismatched_node.to_json(), encoding="utf-8")

        with pytest.raises(NodeMismatchError):
            Parser(markdown_file, node=node_json_path)

    def test_validation_is_based_on_utf8_bytes_not_python_characters(self, tmp_path: Path):
        # A document containing multi-byte UTF-8 characters, where
        # character count and byte count genuinely differ - validation
        # must still succeed against the *same* file.
        unicode_markdown = "# café — 日本語\n\nsome naïve text.\n"
        path = tmp_path / "unicode.md"
        path.write_text(unicode_markdown, encoding="utf-8")
        assert len(unicode_markdown) != len(unicode_markdown.encode("utf-8"))

        matching_node = Parser(path).parse()
        Parser(path, node=matching_node)  # must not raise despite the byte/char mismatch