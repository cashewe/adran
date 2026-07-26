use markdown::mdast::Node;

use crate::_types::MdastType;
use crate::md_parser::node_name;

/// What to do with an mdast node during the tree walk.
pub enum Classification {
    Mapped(MdastType),
    Ignored,
    Fallback,
}
 
const IGNORED: &[&str] = &[
    "Text",
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
    if let Some(mapped) = MdastType::from_mdast_node(node) {
        return Classification::Mapped(mapped);
    }
    if IGNORED.contains(&node_name(node)) {
        Classification::Ignored
    } else {
        Classification::Fallback
    }
}
