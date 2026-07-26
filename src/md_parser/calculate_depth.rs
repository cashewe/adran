use std::collections::HashMap;

use crate::_types::{NodeId, NodeType};

/// Assigns `depth` (distance from the document root) to every node.
pub fn compute_depths(root_id: &NodeId, all: &mut HashMap<NodeId, NodeType>) {
    let mut stack = vec![(root_id.clone(), 0u32)];

    while let Some((id, depth)) = stack.pop() {
        let children = match all.get_mut(&id) {
            Some(node) => {
                node.depth = depth;
                node.children.clone()
            }
            None => continue,
        };
        stack.extend(children.into_iter().map(|child| (child, depth + 1)));
    }
}
