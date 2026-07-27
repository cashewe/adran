use adran::md_parser::MDParser;
use adran::text_recall::{RecallQuery, discover_section, recall_text_indices};

fn parse_markdown(source: &str) {
    let parser = MDParser::new();
    let _ = parser.parse(source).expect("markdown should parse");
}

#[test]
fn discover_section_returns_the_nearest_real_heading() {
    let source = "# Top Level\n\nIntro paragraph.\n\n## Child Section\n\nSome content here.\n";
    let document = MDParser::new().parse(source).expect("markdown should parse");

    let heading = discover_section(&document, 45, 60).expect("section should be discovered");
    assert_eq!(heading, "Child Section");
}

#[test]
fn recall_text_indices_returns_heading_and_body_ranges() {
    let source = "# Top Level\n\nIntro paragraph.\n\n## Child Section\n\nSome content here.\n";
    let document = MDParser::new().parse(source).expect("markdown should parse");

    let query = RecallQuery::new(45, 60, Some(1), Some(1), false, false);
    let entries = recall_text_indices(&document, &query);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].heading.as_deref(), Some("Child Section"));
    assert!(entries[0].body_range.is_some());
}
