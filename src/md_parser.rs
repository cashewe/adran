mod calculate_depth;
mod classify;
mod id_generator;
mod md_parser;
mod parser_error;
mod walk;
mod node_info;

pub use calculate_depth::compute_depths;
pub use classify::{classify, Classification};
pub use id_generator::IdGenerator;
pub use md_parser::{MDParser, ParsedDocument};
pub use parser_error::ParseError;
pub use walk::build;
pub use node_info::{heading_level, node_name, plain_text, range_of};