use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    Markdown(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Markdown(msg) => write!(f, "failed to parse markdown: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}