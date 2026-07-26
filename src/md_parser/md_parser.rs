use markdown::{to_mdast, ParseOptions};

use crate::_types::{NodeId, NodeType};
use crate::md_parser::{IdGenerator, compute_depths, build, ParseError};


pub struct MDParser {
    options: ParseOptions,
}
 
impl Default for MDParser {
    fn default() -> Self {
        Self::new()
    }
}
 
impl MDParser {
    pub fn new() -> Self {
        // gfm() enables GitHub-flavored constructs, notably tables, which
        // `MdastType::Table` depends on.
        Self {
            options: ParseOptions::gfm(),
        }
    }
 
    pub fn parse(&self, source: &str) -> Result<ParsedDocument, ParseError> {
        let ast =
            to_mdast(source, &self.options).map_err(|e| ParseError::Markdown(e.to_string()))?;
 
        let mut ids = IdGenerator::new();
        let (root_id, mut nodes) = build(&ast, &mut ids);
 
        compute_depths(&root_id, &mut nodes);
 
        let mut nodes: Vec<NodeType> = nodes.into_values().collect();
        // Parent-before-child, source order: makes the flat JSON list
        // readable, and gives a stable, deterministic output order.
        nodes.sort_by(|a, b| a.range.start.cmp(&b.range.start).then(a.depth.cmp(&b.depth)));
 
        Ok(ParsedDocument {
            source_len: source.len(),
            root_id,
            nodes,
        })
    }
}
 
/// Output of a parse: a flat, sorted node list plus enough metadata to
/// sanity-check or reconstruct the tree without re-parsing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedDocument {
    pub root_id: NodeId,
    pub source_len: usize,
    pub nodes: Vec<NodeType>,
}
