use markdown::mdast::Node;

use crate::_types::MdastType;

/// What to do with an mdast node during the tree walk.
pub enum Classification {
    Mapped(MdastType),
    Ignored,
    Fallback,
}

// explicitly ignore nodes of these types when walking the tree.
const IGNORED: &[&str] = &[
    "Emphasis",
    "Strong",
    "Delete",
    "InlineCode",
    "Link",
    "LinkReference",
    "Image",
    "ImageReference",
    "Break",
    "TableRow",
    "TableCell",
    "ListItem",
    "FootnoteReference",
];

pub fn classify(node: &Node) -> Classification {
    if matches!(node, Node::Text(_)) {
        return Classification::Ignored;
    }
    if let Some(mapped) = MdastType::from_mdast_node(node) {
        return Classification::Mapped(mapped);
    }
    if IGNORED.contains(&node_name(node).as_str()) {
        Classification::Ignored
    } else {
        Classification::Fallback
    }
}

fn node_name(node: &Node) -> String {
    match node {
        Node::Root(_) => "Root".to_string(),
        Node::Paragraph(_) => "Paragraph".to_string(),
        Node::Heading(_) => "Heading".to_string(),
        Node::Text(_) => "Text".to_string(),
        Node::List(_) => "List".to_string(),
        Node::Table(_) => "Table".to_string(),
        Node::Emphasis(_) => "Emphasis".to_string(),
        Node::Strong(_) => "Strong".to_string(),
        Node::Delete(_) => "Delete".to_string(),
        Node::InlineCode(_) => "InlineCode".to_string(),
        Node::Link(_) => "Link".to_string(),
        Node::LinkReference(_) => "LinkReference".to_string(),
        Node::Image(_) => "Image".to_string(),
        Node::ImageReference(_) => "ImageReference".to_string(),
        Node::Break(_) => "Break".to_string(),
        Node::TableRow(_) => "TableRow".to_string(),
        Node::TableCell(_) => "TableCell".to_string(),
        Node::ListItem(_) => "ListItem".to_string(),
        Node::FootnoteReference(_) => "FootnoteReference".to_string(),
        _ => "Other".to_string(),
    }
}