class MdparserError(Exception):
    """Base class for every exception this package raises."""
 
 
class MarkdownParseError(MdparserError):
    """Raised when the Rust core fails to parse a markdown document."""
 
 
class NodeMismatchError(MdparserError):
    """Raised when a `Node`/node-JSON file passed into `Parser.__init__`
    doesn't look like it was produced from the given markdown file."""
