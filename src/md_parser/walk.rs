use std::collections::HashMap;
 
use markdown::mdast::Node;
 
use crate::_types::{MdastType, NodeId, NodeType};
 
use crate::md_parser::{classify, Classification};
use crate::md_parser::IdGenerator;
use crate::md_parser::{heading_level, node_name, plain_text, range_of};
 

pub fn build(root: &Node, ids: &mut IdGenerator) -> (NodeId, HashMap<NodeId, NodeType>) {
    let mut out = HashMap::new();
 
    let root_range = range_of(root).expect("document root always has a position");
    let root_id = ids.next(MdastType::Root.as_str());
 
    let raw_children: &[Node] = root.children().map(Vec::as_slice).unwrap_or(&[]);
    let root_children = process_children(raw_children, &root_id, ids, &mut out);
 
    out.insert(
        root_id.clone(),
        NodeType::new(
            root_id.clone(),
            MdastType::Root,
            root_range,
            None,
            root_children,
            0,
            MdastType::Root.as_str().to_string(),
        ),
    );
 
    (root_id, out)
}
 
struct OpenSection {
    level: u8,
    id: NodeId,
}
 

fn process_children(
    children: &[Node],
    scope_id: &NodeId,
    ids: &mut IdGenerator,
    out: &mut HashMap<NodeId, NodeType>,
) -> Vec<NodeId> {
    let mut stack: Vec<OpenSection> = Vec::new();
    let mut top_level: Vec<NodeId> = Vec::new();
 
    for child in children {
        if let Some(level) = heading_level(child) {
            close_sections_at_or_above(level, &mut stack, &mut top_level, out);
            let parent_id = current_parent(&stack, scope_id);
            let section_id = open_section(child, parent_id, ids, out);
            stack.push(OpenSection { level, id: section_id });
            continue;
        }
 
        let parent_id = current_parent(&stack, scope_id);
        if let Some(id) = emit_leaf(child, parent_id, ids, out) {
            attach(&stack, id, &mut top_level, out);
        }
    }
 
    close_sections_at_or_above(0, &mut stack, &mut top_level, out);
    top_level
}
 

fn emit_leaf(
    node: &Node,
    parent: NodeId,
    ids: &mut IdGenerator,
    out: &mut HashMap<NodeId, NodeType>,
) -> Option<NodeId> {
    let range = range_of(node)?;
    let mdast_type = match classify(node) {
        Classification::Mapped(t) => t,
        Classification::Fallback => MdastType::Other(node_name(node).to_string()),
        Classification::Ignored => return None,
    };
 
    let id = ids.next(mdast_type.as_str());
    let heading = mdast_type.as_str().to_string();
    out.insert(
        id.clone(),
        NodeType::new(id.clone(), mdast_type, range, Some(parent), Vec::new(), 0, heading),
    );
    Some(id)
}
 

fn current_parent(stack: &[OpenSection], scope_id: &NodeId) -> NodeId {
    stack
        .last()
        .map(|s| s.id.clone())
        .unwrap_or_else(|| scope_id.clone())
}
 

fn attach(
    stack: &[OpenSection],
    id: NodeId,
    top_level: &mut Vec<NodeId>,
    out: &mut HashMap<NodeId, NodeType>,
) {
    match stack.last() {
        Some(open) => extend_section(&open.id, id, out),
        None => top_level.push(id),
    }
}
 

fn open_section(
    heading_node: &Node,
    parent_id: NodeId,
    ids: &mut IdGenerator,
    out: &mut HashMap<NodeId, NodeType>,
) -> NodeId {
    let range = range_of(heading_node).expect("heading nodes always have a position");
    let heading = plain_text(heading_node);
    let section_id = ids.next(MdastType::Section.as_str());
 
    out.insert(
        section_id.clone(),
        NodeType::new(
            section_id.clone(),
            MdastType::Section,
            range,
            Some(parent_id),
            Vec::new(),
            0,
            heading,
        ),
    );
    section_id
}
 

fn close_sections_at_or_above(
    level: u8,
    stack: &mut Vec<OpenSection>,
    top_level: &mut Vec<NodeId>,
    out: &mut HashMap<NodeId, NodeType>,
) {
    while let Some(top) = stack.last() {
        if top.level < level {
            break;
        }
        let closed = stack.pop().unwrap();
        match stack.last() {
            Some(parent) => extend_section(&parent.id, closed.id, out),
            None => top_level.push(closed.id),
        }
    }
}
 

fn extend_section(section_id: &NodeId, child_id: NodeId, out: &mut HashMap<NodeId, NodeType>) {
    let child_end = out[&child_id].range.end;
    let section = out.get_mut(section_id).unwrap();
    section.children.push(child_id);
    if child_end > section.range.end {
        section.range.end = child_end;
    }
}
