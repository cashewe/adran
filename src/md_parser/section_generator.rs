use std::collections::HashMap;

use crate::_types::{MdastType, MetaFieldLabel, NodeId, NodeType};

use crate::md_parser::IdGenerator;

struct OpenSection {
    level: u8,
    id: NodeId,
}

/// Synthesizes `Section` nodes from a flat sequence of top-level nodes
pub fn sectionize(root_id: &NodeId, all: &mut HashMap<NodeId, NodeType>, ids: &mut IdGenerator) {
    let original_children = all
        .get(root_id)
        .expect("root must exist")
        .children
        .clone();

    let mut stack: Vec<OpenSection> = Vec::new();
    let mut new_root_children: Vec<NodeId> = Vec::new();

    for child_id in original_children {
        let is_heading = all[&child_id].mdast_type == MdastType::Heading;

        if is_heading {
            let level = heading_level_of(&all[&child_id]).unwrap_or(1);

            close_sections_at_or_above(level, &mut stack, &mut new_root_children, all);

            let parent_id = stack
                .last()
                .map(|s| s.id.clone())
                .unwrap_or_else(|| root_id.clone());
            let section_id = open_section(&child_id, parent_id, all, ids);
            stack.push(OpenSection {
                level,
                id: section_id,
            });
        } else if let Some(open) = stack.last() {
            attach_child(&open.id, &child_id, all);
        } else {
            new_root_children.push(child_id);
        }
    }

    // Close anything still open at end of document.
    close_sections_at_or_above(0, &mut stack, &mut new_root_children, all);

    all.get_mut(root_id).unwrap().children = new_root_children;
}

fn heading_level_of(node: &NodeType) -> Option<u8> {
    node.meta
        .iter()
        .find(|m| m.label == MetaFieldLabel::HeadingLevel)
        .and_then(|m| m.value.parse().ok())
}

fn close_sections_at_or_above(
    level: u8,
    stack: &mut Vec<OpenSection>,
    new_root_children: &mut Vec<NodeId>,
    all: &mut HashMap<NodeId, NodeType>,
) {
    while let Some(top) = stack.last() {
        if top.level < level {
            break;
        }
        let closed = stack.pop().unwrap();
        match stack.last() {
            Some(parent) => attach_child(&parent.id, &closed.id, all),
            None => new_root_children.push(closed.id),
        }
    }
}

fn open_section(
    heading_id: &NodeId,
    parent_id: NodeId,
    all: &mut HashMap<NodeId, NodeType>,
    ids: &mut IdGenerator,
) -> NodeId {
    let heading_range = all[heading_id].range.clone();
    let section_id = ids.next("section");

    all.get_mut(heading_id).unwrap().parent = Some(section_id.clone());

    let section = NodeType::new(
        section_id.clone(),
        MdastType::Section,
        heading_range,
        Some(parent_id),
        vec![heading_id.clone()],
        0,
        Vec::new(),
    );
    all.insert(section_id.clone(), section);
    section_id
}

/// Reparents `child_id` under `section_id` and extends the section's range
/// to cover it. The walker visits nodes in source order, so extending to
/// the latest child's end is sufficient
fn attach_child(section_id: &NodeId, child_id: &NodeId, all: &mut HashMap<NodeId, NodeType>) {
    let child_end = all[child_id].range.end;
    all.get_mut(child_id).unwrap().parent = Some(section_id.clone());

    let section = all.get_mut(section_id).unwrap();
    section.children.push(child_id.clone());
    if child_end > section.range.end {
        section.range.end = child_end;
    }
}
