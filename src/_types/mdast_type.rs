use std::fmt;

use markdown::mdast::Node;

/// Enum to mostly mirror the more complex Enums in
/// markdown crate. This keeps the JSON-facing representation
/// intentionally narrow and aligned with the README plan.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MdastType {
    Root,
    Section, // synthesized from the heading hierarchy - see outline
    Paragraph,
    List,
    Table,
    Other(String),
}
 
impl MdastType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Root => "Root",
            Self::Section => "Section",
            Self::Paragraph => "Paragraph",
            Self::List => "List",
            Self::Table => "Table",
            Self::Other(name) => name.as_str(),
        }
    }
}
 
impl fmt::Display for MdastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
 
impl MdastType {
    /// Maps a raw mdast node onto first-class kinds. Everything
    /// else including `Text` and `Heading` now - returns `None` and is
    /// resolved one layer up by `classify` 
    pub fn from_mdast_node(node: &Node) -> Option<Self> {
        match node {
            Node::Root(_) => Some(Self::Root),
            Node::List(_) => Some(Self::List),
            Node::Table(_) => Some(Self::Table),
            Node::Paragraph(_) => Some(Self::Paragraph),
            _ => None,
        }
    }
}
