use std::fmt;
use crate::_types::{MdastType, MetaField, NodeId};

/// NodeType is a JSON-flattenable representation of an MDAST node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeType {
    pub id: NodeId,
    pub mdast_type: MdastType,
    pub start_idx: u32,
    pub end_idx: u32,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub depth: u32,
    pub meta: Vec<MetaField>,
}

/// allow users to easily view the obj.
impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} ({}, {})]", self.id, self.mdast_type, self.start_idx, self.end_idx)
    }
}