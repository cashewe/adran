use crate::_types::RangeIdx;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecallQuery {
    pub start: usize,
    pub end: usize,
    pub text_depth: Option<usize>,
    pub heading_depth: Option<usize>,
    pub text_siblings: bool,
    pub heading_siblings: bool,
}

impl RecallQuery {
    pub fn new(
        start: usize,
        end: usize,
        text_depth: Option<usize>,
        heading_depth: Option<usize>,
        text_siblings: bool,
        heading_siblings: bool,
    ) -> Self {
        Self {
            start,
            end,
            text_depth,
            heading_depth,
            text_siblings,
            heading_siblings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecallEntry {
    pub heading: Option<String>,
    pub body_range: Option<RangeIdx>,
    pub depth: u32,
}

pub type RecallResult = Vec<RecallEntry>;
