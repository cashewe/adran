use std::fmt;

/// A source range describing the span of a node in the original markdown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangeIdx {
    pub start: usize,
    pub end: usize,
}

impl RangeIdx {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for RangeIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}