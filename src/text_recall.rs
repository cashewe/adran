mod types;
mod recall;

pub use recall::{discover_section, recall_text_indices};
pub use types::{RecallEntry, RecallQuery, RecallResult};