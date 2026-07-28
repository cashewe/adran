use std::collections::HashMap;

use crate::_types::{MdastType, RangeIdx};
use crate::md_parser::ParsedDocument;
use crate::text_recall::types::{RecallEntry, RecallQuery, RecallResult};

fn discover_section_id(document: &ParsedDocument, start: usize, end: usize) -> Option<String> {
    let target = RangeIdx::new(start, end);
    let containing_node = document
        .nodes
        .iter()
        .filter(|node| node.range.overlaps(&target) || node.range.contains(start) || node.range.contains(end.saturating_sub(1)))
        .max_by_key(|node| node.depth)?;

    let mut current = Some(containing_node.id.as_str().to_string());
    while let Some(id) = current {
        let node = document.nodes.iter().find(|candidate| candidate.id.as_str() == id)?;
        if node.mdast_type == MdastType::Section {
            return Some(node.id.as_str().to_string());
        }
        current = node.parent.as_ref().map(|parent| parent.as_str().to_string());
    }

    None
}

pub fn discover_section(document: &ParsedDocument, start: usize, end: usize) -> Option<String> {
    let id = discover_section_id(document, start, end)?;
    document
        .nodes
        .iter()
        .find(|node| node.id.as_str() == id)
        .map(|node| node.heading.trim().to_string())
}

pub fn recall_text_indices(document: &ParsedDocument, query: &RecallQuery) -> RecallResult {
    let by_id: HashMap<&str, _> = document.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    let Some(match_id) = discover_section_id(document, query.start, query.end) else {
        return Vec::new();
    };
    let Some(match_node) = by_id.get(match_id.as_str()).copied() else {
        return Vec::new();
    };

    // (document position, entry) - sorted into document order before returning.
    let mut staged: Vec<(usize, RecallEntry)> = Vec::new();

    let mut current = Some(match_node);
    let mut level: usize = 0;

    while let Some(node) = current {
        if node.mdast_type != MdastType::Section {
            break;
        }

        let heading = node.heading.trim();
        let rank = level + 1;
        let text_included = query.text_depth.map_or(true, |d| rank <= d);
        let heading_included = query.heading_depth.map_or(true, |d| rank <= d);

        if !text_included && !heading_included {
            break;
        }

        if text_included {
            staged.push((
                node.range.start,
                RecallEntry {
                    heading: Some(heading.to_string()),
                    body_range: body_range_for_section(document, node.id.as_str()),
                    depth: node.depth,
                },
            ));
        } else if heading_included {
            staged.push((
                node.range.start,
                RecallEntry {
                    heading: Some(heading.to_string()),
                    body_range: None,
                    depth: node.depth,
                },
            ));
        }

        if let Some(parent_id) = node.parent.as_ref() {
            if let Some(parent) = by_id.get(parent_id.as_str()) {
                let parent_rank = rank + 1;
                let parent_text_included = query.text_depth.map_or(true, |d| parent_rank <= d);
                let parent_heading_included = query.heading_depth.map_or(true, |d| parent_rank <= d);
                let parent_included = parent_text_included || parent_heading_included;

                if parent_included {
                    for sibling_id in &parent.children {
                        if sibling_id.as_str() == node.id.as_str() {
                            continue;
                        }
                        let Some(sibling) = by_id.get(sibling_id.as_str()).copied() else {
                            continue;
                        };
                        if sibling.mdast_type != MdastType::Section {
                            continue;
                        }
                        let sib_heading = sibling.heading.trim();

                        if query.text_siblings && text_included {
                            staged.push((
                                sibling.range.start,
                                RecallEntry {
                                    heading: Some(sib_heading.to_string()),
                                    body_range: body_range_for_section(document, sibling.id.as_str()),
                                    depth: sibling.depth,
                                },
                            ));
                        } else if query.heading_siblings && heading_included {
                            staged.push((
                                sibling.range.start,
                                RecallEntry {
                                    heading: Some(sib_heading.to_string()),
                                    body_range: None,
                                    depth: sibling.depth,
                                },
                            ));
                            push_subsection_headings(document, &by_id, sibling, false, &mut staged);
                        }
                    }
                }
            }
        }

        current = node.parent.as_ref().and_then(|id| by_id.get(id.as_str()).copied());
        level += 1;
    }

    staged.sort_by_key(|(start, _)| *start);
    staged.into_iter().map(|(_, entry)| entry).collect()
}

fn body_range_for_section(document: &ParsedDocument, section_id: &str) -> Option<RangeIdx> {
    let section = document.nodes.iter().find(|node| node.id.as_str() == section_id)?;

    let mut content_start: Option<usize> = None;
    let mut first_subsection_start: Option<usize> = None;

    for child_id in &section.children {
        let Some(child) = document.nodes.iter().find(|node| node.id.as_str() == child_id.as_str()) else {
            continue;
        };
        if child.mdast_type == MdastType::Section {
            first_subsection_start = Some(first_subsection_start.map_or(child.range.start, |s| s.min(child.range.start)));
        } else {
            content_start = Some(content_start.map_or(child.range.start, |s| s.min(child.range.start)));
        }
    }

    let body_start = content_start?;
    let body_end = first_subsection_start.unwrap_or(section.range.end);

    if body_start >= body_end {
        return None;
    }

    Some(RangeIdx::new(body_start, body_end))
}


fn push_subsection_headings(
    document: &ParsedDocument,
    by_id: &HashMap<&str, &crate::_types::NodeType>,
    node: &crate::_types::NodeType,
    include_text: bool,
    staged: &mut Vec<(usize, RecallEntry)>,
) {
    for child_id in &node.children {
        let Some(child) = by_id.get(child_id.as_str()).copied() else { continue };
        if child.mdast_type != MdastType::Section {
            continue;
        }
        staged.push((
            child.range.start,
            RecallEntry {
                heading: Some(child.heading.trim().to_string()),
                body_range: if include_text { body_range_for_section(document, child.id.as_str()) } else { None },
                depth: child.depth,
            },
        ));
        push_subsection_headings(document, by_id, child, include_text, staged);
    }
}