use std::fmt;

/// Enum to mostly mirror the more complex Enums in
/// markdwon crate. this way we can control quite
/// what serde looks like for the object.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MdastType {
    Root,
    Blockquote,
    FootnoteDefinition,
    MdxJsxFlowElement,
    List,
    MdxjsEsm,
    Toml,
    Yaml,
    Break,
    InlineCode,
    InlineMath,
    Delete,
    Emphasis,
    MdxTextExpression,
    FootnoteReference,
    Html,
    Image,
    ImageReference,
    MdxJsxTextElement,
    Link,
    LinkReference,
    Strong,
    Text,
    Code,
    Math,
    MdxFlowExpression,
    Heading,
    Table,
    ThematicBreak,
    TableRow,
    TableCell,
    ListItem,
    Definition,
    Paragraph,
    Section, // needed to link section bodies to their headers
}

impl MdastType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Root => "Root",
            Self::Blockquote => "Blockquote",
            Self::FootnoteDefinition => "FootnoteDefinition",
            Self::MdxJsxFlowElement => "MdxJsxFlowElement",
            Self::List => "List",
            Self::MdxjsEsm => "MdxjsEsm",
            Self::Toml => "Toml",
            Self::Yaml => "Yaml",
            Self::Break => "Break",
            Self::InlineCode => "InlineCode",
            Self::InlineMath => "InlineMath",
            Self::Delete => "Delete",
            Self::Emphasis => "Emphasis",
            Self::MdxTextExpression => "MdxTextExpression",
            Self::FootnoteReference => "FootnoteReference",
            Self::Html => "Html",
            Self::Image => "Image",
            Self::ImageReference => "ImageReference",
            Self::MdxJsxTextElement => "MdxJsxTextElement",
            Self::Link => "Link",
            Self::LinkReference => "LinkReference",
            Self::Strong => "Strong",
            Self::Text => "Text",
            Self::Code => "Code",
            Self::Math => "Math",
            Self::MdxFlowExpression => "MdxFlowExpression",
            Self::Heading => "Heading",
            Self::Table => "Table",
            Self::ThematicBreak => "ThematicBreak",
            Self::TableRow => "TableRow",
            Self::TableCell => "TableCell",
            Self::ListItem => "ListItem",
            Self::Definition => "Definition",
            Self::Paragraph => "Paragraph",
            Self::Section => "Section",
        }
    }
}

impl fmt::Display for MdastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&MdastNode> for MdastType {
    fn from(node: &MdastNode) -> Self {
        match node {
            MdastNode::Root(_) => Self::Root,
            MdastNode::Blockquote(_) => Self::Blockquote,
            MdastNode::FootnoteDefinition(_) => Self::FootnoteDefinition,
            MdastNode::MdxJsxFlowElement(_) => Self::MdxJsxFlowElement,
            MdastNode::List(_) => Self::List,
            MdastNode::MdxjsEsm(_) => Self::MdxjsEsm,
            MdastNode::Toml(_) => Self::Toml,
            MdastNode::Yaml(_) => Self::Yaml,
            MdastNode::Break(_) => Self::Break,
            MdastNode::InlineCode(_) => Self::InlineCode,
            MdastNode::InlineMath(_) => Self::InlineMath,
            MdastNode::Delete(_) => Self::Delete,
            MdastNode::Emphasis(_) => Self::Emphasis,
            MdastNode::MdxTextExpression(_) => Self::MdxTextExpression,
            MdastNode::FootnoteReference(_) => Self::FootnoteReference,
            MdastNode::Html(_) => Self::Html,
            MdastNode::Image(_) => Self::Image,
            MdastNode::ImageReference(_) => Self::ImageReference,
            MdastNode::MdxJsxTextElement(_) => Self::MdxJsxTextElement,
            MdastNode::Link(_) => Self::Link,
            MdastNode::LinkReference(_) => Self::LinkReference,
            MdastNode::Strong(_) => Self::Strong,
            MdastNode::Text(_) => Self::Text,
            MdastNode::Code(_) => Self::Code,
            MdastNode::Math(_) => Self::Math,
            MdastNode::MdxFlowExpression(_) => Self::MdxFlowExpression,
            MdastNode::Heading(_) => Self::Heading,
            MdastNode::Table(_) => Self::Table,
            MdastNode::ThematicBreak(_) => Self::ThematicBreak,
            MdastNode::TableRow(_) => Self::TableRow,
            MdastNode::TableCell(_) => Self::TableCell,
            MdastNode::ListItem(_) => Self::ListItem,
            MdastNode::Definition(_) => Self::Definition,
            MdastNode::Paragraph(_) => Self::Paragraph,
        }
    }
}
