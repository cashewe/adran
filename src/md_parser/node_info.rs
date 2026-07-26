use markdown::mdast::Node;

use crate::_types::RangeIdx;


pub fn range_of(node: &Node) -> Option<RangeIdx> {
    node.position()
        .map(|p| RangeIdx::new(p.start.offset, p.end.offset))
}


pub fn node_name(node: &Node) -> &'static str {
    match node {
        Node::Root(_) => "Root",
        Node::Paragraph(_) => "Paragraph",
        Node::Heading(_) => "Heading",
        Node::Text(_) => "Text",
        Node::List(_) => "List",
        Node::Table(_) => "Table",
        Node::Emphasis(_) => "Emphasis",
        Node::Strong(_) => "Strong",
        Node::Delete(_) => "Delete",
        Node::InlineCode(_) => "InlineCode",
        Node::Link(_) => "Link",
        Node::LinkReference(_) => "LinkReference",
        Node::Image(_) => "Image",
        Node::ImageReference(_) => "ImageReference",
        Node::Break(_) => "Break",
        Node::TableRow(_) => "TableRow",
        Node::TableCell(_) => "TableCell",
        Node::ListItem(_) => "ListItem",
        Node::FootnoteReference(_) => "FootnoteReference",
        Node::Blockquote(_) => "Blockquote",
        Node::Code(_) => "Code",
        Node::ThematicBreak(_) => "ThematicBreak",
        Node::Html(_) => "Html",
        Node::Definition(_) => "Definition",
        Node::FootnoteDefinition(_) => "FootnoteDefinition",
        Node::Yaml(_) => "Yaml",
        Node::Toml(_) => "Toml",
        _ => "Other",
    }
}


pub fn heading_level(node: &Node) -> Option<u8> {
    match node {
        Node::Heading(h) => Some(h.depth),
        _ => None,
    }
}


pub fn plain_text(node: &Node) -> String {
    match node {
        Node::Text(t) => t.value.clone(),
        Node::InlineCode(c) => c.value.clone(),
        _ => node
            .children()
            .map(|kids| kids.iter().map(plain_text).collect::<Vec<_>>().join(""))
            .unwrap_or_default(),
    }
}