use std::collections::HashMap;

use crate::_types::NodeId;

/// Generates short, readable, unique node ids of the form `{prefix}-{n}`,
/// e.g. "heading-0", "heading-1", "paragraph-0", "other-code-0". 
#[derive(Default)]
pub struct IdGenerator {
    counters: HashMap<String, usize>,
}


impl IdGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a new unique id for the given prefix.
    pub fn next(&mut self, prefix: &str) -> NodeId {
        let counter = self.counters.entry(prefix.to_string()).or_insert(0);
        let id = NodeId::new(format!("{prefix}-{counter}"));
        *counter += 1;
        id
    }
}