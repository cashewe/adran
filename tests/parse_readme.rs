use std::{fs, path::PathBuf};

use adran::md_parser::MDParser;

#[test]
fn parse_readme_and_write_output() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let readme_path = manifest_dir.join("README.md");
    let output_path = manifest_dir.join("tests").join("README_parse_output.json");

    let source = fs::read_to_string(&readme_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", readme_path.display()));

    let parser = MDParser::new();
    let parsed = parser
        .parse(&source)
        .unwrap_or_else(|err| panic!("failed to parse README: {err}"));

    assert!(
        !parsed.nodes.iter().any(|node| node.mdast_type.to_string() == "Text"),
        "parser should not emit standalone text nodes"
    );

    let node_by_id: std::collections::HashMap<_, _> = parsed
        .nodes
        .iter()
        .map(|node| (node.id.as_str().to_string(), node))
        .collect();
    for node in &parsed.nodes {
        if let Some(parent_id) = &node.parent {
            let parent = node_by_id
                .get(parent_id.as_str())
                .expect("parent node should exist in parsed output");
            let parent_kind = parent.mdast_type.to_string();
            assert!(
                parent_kind == "Root" || parent_kind == "Section",
                "nodes should be attached only under root or section parents"
            );
        }
    }

    let json = serde_json::to_string_pretty(&parsed)
        .expect("failed to serialize parsed document as JSON");

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("failed to create tests output directory");
    }

    fs::write(&output_path, json).expect("failed to write parser output file");

    assert!(output_path.exists(), "expected output file to be created");
    println!("wrote parser output to {}", output_path.display());
}
