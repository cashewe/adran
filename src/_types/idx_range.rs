use std::fmt;

/// A source range describing the span of a node in the original markdown.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RangeIdx {
    pub start: usize,
    pub end: usize,
}

impl RangeIdx {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
 
    /// Number of bytes covered by this range.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
 
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
 
    /// Whether a single byte offset falls inside this range.
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
 
    /// Whether `other` overlaps this range at all (shared byte(s)).
    pub fn overlaps(&self, other: &RangeIdx) -> bool {
        self.start < other.end && other.start < self.end
    }
}

impl fmt::Display for RangeIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}