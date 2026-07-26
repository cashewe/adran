use std::collections::HashMap;

use markdown::mdast::Node;

use crate::_types::{MdastType, MetaField, MetaFieldLabel, NodeId, NodeType, RangeIdx};

use crate::md_parser::classify::{Classification, classify};
use crate::md_parser::IdGenerator;

/// Recursively walks the raw mdast tree and produces a flat map of
/// `NodeType`s for every node we choose to keep (mapped or fallback),
/// wired up with parent/child ids.
pub fn walk(root: &Node, ids: &mut IdGenerator) -> (NodeId, HashMap<NodeId, NodeType>) {
    let mut out = HashMap::new();
    let created = visit(root, None, ids, &mut out);
    let root_id = created
        .into_iter()
        .next()
        .expect("document root always has a position and always maps to MdastType::Root");
    (root_id, out)
}

fn visit(
    node: &Node,
    parent: Option<NodeId>,
    ids: &mut IdGenerator,
    out: &mut HashMap<NodeId, NodeType>,
) -> Vec<NodeId> {
    let Some(range) = range_of(node) else {
        return visit_children(node, parent, ids, out);
    };

    let should_emit = match &parent {
        None => true,
        Some(parent_id) => match out.get(parent_id) {
            Some(parent_node) => matches!(parent_node.mdast_type, MdastType::Root | MdastType::Section),
            None => true,
        },
    };

    if !should_emit {
        return Vec::new();
    }

    match classify(node) {
        Classification::Ignored => visit_children(node, parent, ids, out),

        Classification::Mapped(mdast_type) => {
            let id = ids.next(mdast_type.as_str());
            let meta = build_meta(node, &mdast_type);
            let children = if mdast_type == MdastType::Root || mdast_type == MdastType::Section {
                visit_children(node, Some(id.clone()), ids, out)
            } else {
                Vec::new()
            };
            out.insert(
                id.clone(),
                NodeType::new(id.clone(), mdast_type, range, parent, children, 0, meta),
            );
            vec![id]
        }

        Classification::Fallback => {
            let name = node_name(node);
            let id = ids.next(&format!("other-{name}"));
            let children = Vec::new();
            out.insert(
                id.clone(),
                NodeType::new(
                    id.clone(),
                    MdastType::Other(name),
                    range,
                    parent,
                    children,
                    0,
                    Vec::new(),
                ),
            );
            vec![id]
        }
    }
}

fn visit_children(
    node: &Node,
    parent: Option<NodeId>,
    ids: &mut IdGenerator,
    out: &mut HashMap<NodeId, NodeType>,
) -> Vec<NodeId> {
    node.children()
        .map(|kids| {
            kids.iter()
                .flat_map(|child| visit(child, parent.clone(), ids, out))
                .collect()
        })
        .unwrap_or_default()
}

fn build_meta(node: &Node, mdast_type: &MdastType) -> Vec<MetaField> {
    match mdast_type {
        MdastType::Heading => {
            let mut meta = vec![MetaField::new(MetaFieldLabel::Wording, plain_text(node))];
            if let Some(level) = heading_level(node) {
                meta.push(MetaField::new(
                    MetaFieldLabel::HeadingLevel,
                    level.to_string(),
                ));
            }
            meta
        }
        MdastType::Table => table_column_count(node)
            .map(|n| vec![MetaField::new(MetaFieldLabel::Columns, n.to_string())])
            .unwrap_or_default(),
        _ => Vec::new(),
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

fn heading_level(node: &Node) -> Option<u8> {
    match node {
        Node::Heading(heading) => Some(heading.depth),
        _ => None,
    }
}

fn plain_text(node: &Node) -> String {
    match node {
        Node::Text(text) => text.value.clone(),
        Node::InlineCode(code) => code.value.clone(),
        _ => node
            .children()
            .map(|children| children.iter().map(plain_text).collect::<Vec<_>>().join(""))
            .unwrap_or_default(),
    }
}

fn range_of(node: &Node) -> Option<RangeIdx> {
    let position = match node {
        Node::Root(root) => root.position.as_ref(),
        Node::Paragraph(paragraph) => paragraph.position.as_ref(),
        Node::Heading(heading) => heading.position.as_ref(),
        Node::Text(text) => text.position.as_ref(),
        Node::Table(table) => table.position.as_ref(),
        Node::List(list) => list.position.as_ref(),
        Node::Blockquote(block_quote) => block_quote.position.as_ref(),
        Node::Code(code) => code.position.as_ref(),
        Node::InlineCode(code) => code.position.as_ref(),
        Node::Emphasis(emphasis) => emphasis.position.as_ref(),
        Node::Strong(strong) => strong.position.as_ref(),
        Node::Delete(delete) => delete.position.as_ref(),
        Node::Link(link) => link.position.as_ref(),
        Node::Image(image) => image.position.as_ref(),
        Node::LinkReference(link_reference) => link_reference.position.as_ref(),
        Node::ImageReference(image_reference) => image_reference.position.as_ref(),
        Node::TableRow(row) => row.position.as_ref(),
        Node::TableCell(cell) => cell.position.as_ref(),
        Node::ListItem(item) => item.position.as_ref(),
        _ => None,
    };

    position.map(|pos| RangeIdx::new(pos.start.offset, pos.end.offset))
}

fn table_column_count(node: &Node) -> Option<usize> {
    match node {
        Node::Table(table) => Some(table.align.len().max(1)),
        _ => None,
    }
}
