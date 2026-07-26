use std::fmt;

use super::{MdastType, NodeId, RangeIdx};


/// A JSON-friendly representation of an mdast node skeleton.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NodeType {
    pub id: NodeId,
    pub mdast_type: MdastType,
    pub range: RangeIdx,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub depth: u32,
    pub heading: String,
}
 
impl NodeType {
    pub fn new(
        id: NodeId,
        mdast_type: MdastType,
        range: RangeIdx,
        parent: Option<NodeId>,
        children: Vec<NodeId>,
        depth: u32,
        heading: String,
    ) -> Self {
        Self {
            id,
            mdast_type,
            range,
            parent,
            children,
            depth,
            heading,
        }
    }
}
 
impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} \"{}\" {}]", self.id, self.mdast_type, self.heading, self.range)
    }
}
