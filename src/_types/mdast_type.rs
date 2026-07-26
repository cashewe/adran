use std::fmt;

use markdown::mdast::Node;

/// Enum to mostly mirror the more complex Enums in
/// markdown crate. This keeps the JSON-facing representation
/// intentionally narrow and aligned with the README plan.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MdastType {
    Root,
    List,
    Text,
    Heading,
    Table,
    Paragraph,
    Section, // needed to link section bodies to their headers
    Other(String),
}

impl MdastType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Root => "Root",
            Self::List => "List",
            Self::Text => "Text",
            Self::Heading => "Heading",
            Self::Table => "Table",
            Self::Paragraph => "Paragraph",
            Self::Section => "Section",
            Self::Other(name) => name,
        }
    }
}

impl fmt::Display for MdastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MdastType {
    pub fn from_mdast_node(node: &Node) -> Option<Self> {
        match node {
            Node::Root(_) => Some(Self::Root),
            Node::List(_) => Some(Self::List),
            Node::Text(_) => Some(Self::Text),
            Node::Heading(_) => Some(Self::Heading),
            Node::Table(_) => Some(Self::Table),
            Node::Paragraph(_) => Some(Self::Paragraph),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use markdown::mdast::{Heading, List, Node, Paragraph, Root, Table, Text, ThematicBreak};

    #[test]
    fn maps_supported_markdown_nodes() {
        assert_eq!(
            MdastType::from_mdast_node(&Node::Root(Root {
                children: vec![],
                position: None,
            })),
            Some(MdastType::Root)
        );
        assert_eq!(
            MdastType::from_mdast_node(&Node::Paragraph(Paragraph {
                children: vec![],
                position: None,
            })),
            Some(MdastType::Paragraph)
        );
        assert_eq!(
            MdastType::from_mdast_node(&Node::Heading(Heading {
                children: vec![],
                position: None,
                depth: 2,
            })),
            Some(MdastType::Heading)
        );
        assert_eq!(
            MdastType::from_mdast_node(&Node::Table(Table {
                children: vec![],
                position: None,
                align: vec![],
            })),
            Some(MdastType::Table)
        );
        assert_eq!(
            MdastType::from_mdast_node(&Node::List(List {
                children: vec![],
                position: None,
                ordered: false,
                start: None,
                spread: false,
            })),
            Some(MdastType::List)
        );
        assert_eq!(
            MdastType::from_mdast_node(&Node::Text(Text {
                value: "hello".into(),
                position: None,
            })),
            Some(MdastType::Text)
        );
    }

    #[test]
    fn ignores_structural_nodes_outside_the_supported_plan() {
        assert_eq!(
            MdastType::from_mdast_node(&Node::ThematicBreak(ThematicBreak { position: None })),
            None
        );
    }
}
