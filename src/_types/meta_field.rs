/// Controlled labels for structured node metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MetaFieldLabel {
    Wording,
    AltText,
    Url,
    Columns,
}

/// A single metadata entry attached to a node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaField {
    pub label: MetaFieldLabel,
    pub value: String,
}

impl MetaField {
    pub fn new(label: MetaFieldLabel, value: impl Into<String>) -> Self {
        Self {
            label,
            value: value.into(),
        }
    }
}
