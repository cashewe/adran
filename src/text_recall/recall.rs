use std::collections::{HashMap, HashSet};

use crate::_types::{MdastType, NodeType, RangeIdx};
use crate::md_parser::ParsedDocument;
use crate::text_recall::types::{RecallEntry, RecallQuery, RecallResult};

fn enclosing_section_id(document: &ParsedDocument, node: &NodeType) -> Option<String> {
    let mut current = Some(node.id.as_str().to_string());
    while let Some(id) = current {
        let candidate = document.nodes.iter().find(|n| n.id.as_str() == id)?;
        if candidate.mdast_type == MdastType::Section {
            return Some(candidate.id.as_str().to_string());
        }
        current = candidate.parent.as_ref().map(|parent| parent.as_str().to_string());
    }
    None
}

fn is_ancestor(document: &ParsedDocument, ancestor_id: &str, node_id: &str) -> bool {
    let Some(mut node) = document.nodes.iter().find(|n| n.id.as_str() == node_id) else {
        return false;
    };
    while let Some(parent_id) = node.parent.as_ref() {
        if parent_id.as_str() == ancestor_id {
            return true;
        }
        let Some(parent) = document.nodes.iter().find(|n| n.id.as_str() == parent_id.as_str()) else {
            break;
        };
        node = parent;
    }
    false
}

pub fn discover_section_ids(document: &ParsedDocument, start: usize, end: usize) -> Vec<String> {
    let target = RangeIdx::new(start, end);

    let mut candidates: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for node in document
        .nodes
        .iter()
        .filter(|node| node.range.overlaps(&target) || node.range.contains(start) || node.range.contains(end.saturating_sub(1)))
    {
        let Some(section_id) = enclosing_section_id(document, node) else {
            continue;
        };
        if seen.insert(section_id.clone()) {
            candidates.push(section_id);
        }
    }

    candidates
        .iter()
        .filter(|candidate| {
            !candidates
                .iter()
                .any(|other| other.as_str() != candidate.as_str() && is_ancestor(document, candidate, other))
        })
        .cloned()
        .collect()
}

pub fn discover_section(document: &ParsedDocument, start: usize, end: usize) -> Vec<String> {
    discover_section_ids(document, start, end)
        .into_iter()
        .filter_map(|id| document.nodes.iter().find(|node| node.id.as_str() == id))
        .map(|node| node.heading.trim().to_string())
        .collect()
}

pub fn recall_text_indices(document: &ParsedDocument, query: &RecallQuery) -> RecallResult {
    let by_id: HashMap<&str, &NodeType> = document.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    let anchors = discover_section_ids(document, query.start, query.end);
    if anchors.is_empty() {
        return Vec::new();
    }

    let mut staged: Vec<(usize, RecallEntry)> = Vec::new();
    let mut seen_ids: HashSet<String> = HashSet::new();

    for anchor_id in &anchors {
        let Some(anchor) = by_id.get(anchor_id.as_str()).copied() else {
            continue;
        };
        stage_upward(document, &by_id, anchor, query, &mut seen_ids, &mut staged);
        stage_downward(document, &by_id, anchor, 2, query, &mut seen_ids, &mut staged);
    }

    staged.sort_by_key(|(start, _)| *start);
    staged.into_iter().map(|(_, entry)| entry).collect()
}


fn stage_upward(
    document: &ParsedDocument,
    by_id: &HashMap<&str, &NodeType>,
    anchor: &NodeType,
    query: &RecallQuery,
    seen_ids: &mut HashSet<String>,
    staged: &mut Vec<(usize, RecallEntry)>,
) {
    let mut current = Some(anchor);
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

        if seen_ids.insert(node.id.as_str().to_string()) {
            if text_included {
                staged.push((
                    node.range.start,
                    RecallEntry {
                        id: node.id.as_str().to_string(),
                        heading: Some(heading.to_string()),
                        body_range: body_range_for_section(document, node.id.as_str()),
                        depth: node.depth,
                        filtered: false,
                    },
                ));
            } else if heading_included {
                staged.push((
                    node.range.start,
                    RecallEntry {
                        id: node.id.as_str().to_string(),
                        heading: Some(heading.to_string()),
                        body_range: body_range_for_section(document, node.id.as_str()),
                        depth: node.depth,
                        filtered: true,
                    },
                ));
            }
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
                            if seen_ids.insert(sibling.id.as_str().to_string()) {
                                staged.push((
                                    sibling.range.start,
                                    RecallEntry {
                                        id: sibling.id.as_str().to_string(),
                                        heading: Some(sib_heading.to_string()),
                                        body_range: body_range_for_section(document, sibling.id.as_str()),
                                        depth: sibling.depth,
                                        filtered: false,
                                    },
                                ));
                            }
                        } else if query.heading_siblings && heading_included {
                            if seen_ids.insert(sibling.id.as_str().to_string()) {
                                staged.push((
                                    sibling.range.start,
                                    RecallEntry {
                                        id: sibling.id.as_str().to_string(),
                                        heading: Some(sib_heading.to_string()),
                                        body_range: body_range_for_section(document, sibling.id.as_str()),
                                        depth: sibling.depth,
                                        filtered: true,
                                    },
                                ));
                            }
                            push_subsection_headings(document, by_id, sibling, false, seen_ids, staged);
                        }
                    }
                }
            }
        }

        current = node.parent.as_ref().and_then(|id| by_id.get(id.as_str()).copied());
        level += 1;
    }
}


fn stage_downward(
    document: &ParsedDocument,
    by_id: &HashMap<&str, &NodeType>,
    node: &NodeType,
    rank: usize,
    query: &RecallQuery,
    seen_ids: &mut HashSet<String>,
    staged: &mut Vec<(usize, RecallEntry)>,
) {
    let text_included = query.text_depth.map_or(true, |d| rank <= d);
    let heading_included = query.heading_depth.map_or(true, |d| rank <= d);

    if !text_included && !heading_included {
        return;
    }

    for child_id in &node.children {
        let Some(child) = by_id.get(child_id.as_str()).copied() else {
            continue;
        };
        if child.mdast_type != MdastType::Section {
            continue;
        }

        if seen_ids.insert(child.id.as_str().to_string()) {
            staged.push((
                child.range.start,
                RecallEntry {
                    id: child.id.as_str().to_string(),
                    heading: Some(child.heading.trim().to_string()),
                    body_range: body_range_for_section(document, child.id.as_str()),
                    depth: child.depth,
                    filtered: !text_included,
                },
            ));
        }

        stage_downward(document, by_id, child, rank + 1, query, seen_ids, staged);
    }
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
    by_id: &HashMap<&str, &NodeType>,
    node: &NodeType,
    include_text: bool,
    seen_ids: &mut HashSet<String>,
    staged: &mut Vec<(usize, RecallEntry)>,
) {
    for child_id in &node.children {
        let Some(child) = by_id.get(child_id.as_str()).copied() else { continue };
        if child.mdast_type != MdastType::Section {
            continue;
        }
        if seen_ids.insert(child.id.as_str().to_string()) {
            staged.push((
                child.range.start,
                RecallEntry {
                    id: child.id.as_str().to_string(),
                    heading: Some(child.heading.trim().to_string()),
                    body_range: body_range_for_section(document, child.id.as_str()),
                    depth: child.depth,
                    filtered: !include_text,
                },
            ));
        }
        push_subsection_headings(document, by_id, child, include_text, seen_ids, staged);
    }
}