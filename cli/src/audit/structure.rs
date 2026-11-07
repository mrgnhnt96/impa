use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::LazyLock;

use regex::Regex;

use super::Findings;
use super::markdown::{Kind, Node, Page, Role};
use super::rules::{S2, S3, S6, S9, S12, S14, S15, S16, S20, S25, S30, S32, S33, S34, T2, T4, T15};

const GENERIC_HEADINGS: &[&str] = &[
    "introduction",
    "overview",
    "details",
    "misc",
    "other",
    "general",
    "background",
];
const VAGUE_LINK_TEXT: &[&str] = &["click here", "here", "this", "read more", "more"];
const PAGE_TYPES: &[&str] = &[
    "tutorial",
    "quickstart",
    "how-to",
    "how-to-guide",
    "reference",
    "explanation",
    "troubleshooting",
    "changelog",
    "migration",
    "migration-guide",
    "readme",
    "decision",
    "decision-record",
    "runbook",
];
const DECISION_PARTS: &[&str] = &["context", "decision", "status", "consequences"];
const FEWEST_PAGES_TO_COMPARE: usize = 3;

fn pattern(text: &str) -> Regex {
    Regex::new(text).expect("valid pattern")
}

static SELF_REFERENCE: LazyLock<Regex> = LazyLock::new(|| {
    pattern(r"(?i)^(in )?this (page|topic|guide|document|doc|article|tutorial|section|file)\b")
});
static SCAFFOLDING: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"\b(TODO|TBD)\b|(?i:\bcoming soon\b|\blorem ipsum dolor\b)"));
static RAW_URL: LazyLock<Regex> = LazyLock::new(|| pattern(r"\bhttps?://[^\s)>]+"));
static IMAGE_TAG: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?is)<img\b[^>]*>"));
static FILLED_ALT: LazyLock<Regex> =
    LazyLock::new(|| pattern(r#"(?is)\balt\s*=\s*("[^"\s][^"]*"|'[^'\s][^']*'|[^\s"'>]+)"#));
static HTML_H1: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?i)<h1\b"));
static PREREQUISITES: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)^(prerequisites?|requirements|before you (begin|start))\b"));
static STEPS: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?i)^steps?\b"));
static OPTIONAL: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?i)\boptional(ly)?\b"));

pub fn check(page: &Page, out: &mut Findings) {
    link_text(page, out);
    images(page, out);
    if page.role != Role::Page {
        return;
    }
    opening(page, out);
    heading_levels(page, out);
    heading_words(page, out);
    heading_spacing(page, out);
    lists(page, out);
    tables(page, out);
    one_target_per_text(page, out);
    scaffolding(page, out);
    steps(page, out);
    decision_record(page, out);
}

pub fn check_project(pages: &[Page], out: &mut Findings) {
    declared_types(pages, out);
    reference_section_order(pages, out);
}

fn text_beside_images(node: &Node) -> String {
    match node.kind {
        Kind::Image { .. } => String::new(),
        Kind::Text | Kind::Span => node.text.clone(),
        _ => node.children.iter().map(text_beside_images).collect(),
    }
}

fn is_badge_only(node: &Node) -> bool {
    let has_image = node.has(|kind| matches!(kind, Kind::Image { .. }));
    has_image && text_beside_images(node).trim().is_empty()
}

fn holds_image(node: &Node) -> bool {
    node.kind == Kind::Html && IMAGE_TAG.is_match(&node.text)
}

fn opening(page: &Page, out: &mut Findings) {
    let mut blocks = page.top_blocks();
    if !blocks.any(|node| node.kind == Kind::Heading(1)) {
        return;
    }
    let Some(first) = blocks.next() else {
        return;
    };
    let path = &page.path;
    if first.kind == Kind::Quote {
        out.add(
            S3,
            path,
            first.line,
            "A notice comes before the page's first sentence",
        );
    } else if is_badge_only(first) || holds_image(first) {
        out.add(
            S3,
            path,
            first.line,
            "An image or badge comes before the page's first sentence",
        );
    } else if first.kind == Kind::Paragraph && SELF_REFERENCE.is_match(first.plain().trim()) {
        out.add(
            S2,
            path,
            first.line,
            "The first sentence talks about the page, not the subject",
        );
    }
}

fn html_h1_count(page: &Page) -> usize {
    let html = page.root.all();
    html.iter()
        .filter(|node| node.kind == Kind::Html)
        .map(|node| HTML_H1.find_iter(&node.text).count())
        .sum()
}

fn heading_levels(page: &Page, out: &mut Findings) {
    let headings = page.headings();
    let path = &page.path;
    let top_level = |node: &&&Node| node.kind == Kind::Heading(1);
    let titles = headings.iter().filter(top_level).count() + html_h1_count(page);
    if titles == 0 && page.front.is_empty() {
        out.add(S12, path, 1, "The page has no H1");
    }
    for extra in headings.iter().filter(top_level).skip(1) {
        out.add(S12, path, extra.line, "The page has more than one H1");
    }
    for pair in headings.windows(2) {
        if let (Kind::Heading(before), Kind::Heading(after)) = (&pair[0].kind, &pair[1].kind)
            && *after > before + 1
        {
            out.add(
                S12,
                path,
                pair[1].line,
                format!("Heading skips from H{before} to H{after}"),
            );
        }
    }
    for heading in &headings {
        let has_image = heading.has(|kind| matches!(kind, Kind::Image { .. }));
        if heading.plain().trim().is_empty() && !has_image {
            out.add(S12, path, heading.line, "Empty heading");
        }
    }
}

fn heading_words(page: &Page, out: &mut Findings) {
    let path = &page.path;
    let code_is_fine = page.doc_type().is_none_or(|kind| kind == "reference");
    for heading in page.headings() {
        let text = heading.plain();
        let text = text.trim();
        if GENERIC_HEADINGS.contains(&text.to_lowercase().as_str()) {
            let message = format!("Heading \"{text}\" doesn't say what the section covers");
            out.add(S14, path, heading.line, message);
        }
        if text.ends_with('.') && !text.ends_with("...") {
            out.add(
                S15,
                path,
                heading.line,
                format!("Heading \"{text}\" ends with a period"),
            );
        }
        if heading.has(|kind| matches!(kind, Kind::Link { .. })) {
            out.add(
                S15,
                path,
                heading.line,
                format!("Heading \"{text}\" holds a link"),
            );
        }
        if !code_is_fine && heading.has(|kind| *kind == Kind::Span) {
            let message = format!("Heading \"{text}\" holds code on a non-reference page");
            out.add(S15, path, heading.line, message);
        }
    }
}

fn heading_spacing(page: &Page, out: &mut Findings) {
    let blocks: Vec<&Node> = page.top_blocks().collect();
    let path = &page.path;
    for (index, block) in blocks.iter().enumerate() {
        let Kind::Heading(level) = block.kind else {
            continue;
        };
        let text = block.plain();
        let text = text.trim();
        match blocks.get(index + 1).map(|next| &next.kind) {
            None => out.add(T4, path, block.line, format!("Section \"{text}\" is empty")),
            Some(Kind::Heading(next)) if *next <= level => {
                out.add(T4, path, block.line, format!("Section \"{text}\" is empty"));
            }
            Some(Kind::Heading(_)) => {
                let message = format!("Heading \"{text}\" has no text before the next heading");
                out.add(S16, path, block.line, message);
            }
            Some(_) => {}
        }
    }
}

fn lists(page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        if matches!(node.kind, Kind::List { .. }) && node.children.len() == 1 {
            out.add(S20, &page.path, node.line, "List has one item");
        }
    }
}

fn is_blank_cell(cell: &Node) -> bool {
    cell.plain().trim().is_empty() && !cell.has(|kind| matches!(kind, Kind::Image { .. }))
}

fn table(table: &Node, path: &str, out: &mut Findings) {
    let Some(header) = table.children.first() else {
        return;
    };
    if header.children.len() < 2 {
        out.add(S25, path, table.line, "Table has fewer than two columns");
    }
    if header.children.iter().all(is_blank_cell) {
        out.add(S25, path, table.line, "Table has no header row");
        return;
    }
    let cells = table.children.iter().flat_map(|row| &row.children);
    let blank = cells.filter(|cell| is_blank_cell(cell)).count();
    if blank > 0 {
        out.add(
            S25,
            path,
            table.line,
            format!("Table has {blank} blank cells"),
        );
    }
}

fn tables(page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        if node.kind == Kind::Table {
            table(node, &page.path, out);
        }
    }
}

fn link_text(page: &Page, out: &mut Findings) {
    let path = &page.path;
    for node in page.root.all() {
        let Kind::Link { dest, autolink } = &node.kind else {
            continue;
        };
        let text = node.plain();
        let text = text.trim();
        if VAGUE_LINK_TEXT.contains(&text.to_lowercase().as_str()) {
            let message = format!("Link text \"{text}\" doesn't make sense alone");
            out.add(S32, path, node.line, message);
        } else if (*autolink || text == dest) && RAW_URL.is_match(text) {
            out.add(
                S32,
                path,
                node.line,
                format!("Raw URL as link text: {text}"),
            );
        }
    }
    for (line, text) in page.root.prose(false) {
        for url in RAW_URL.find_iter(&text) {
            out.add(
                S32,
                path,
                line,
                format!("Raw URL in prose: {}", url.as_str()),
            );
        }
    }
}

fn one_target_per_text(page: &Page, out: &mut Findings) {
    let mut targets: HashMap<String, &str> = HashMap::new();
    for node in page.root.all() {
        let Kind::Link { dest, .. } = &node.kind else {
            continue;
        };
        let text = node.plain().trim().to_lowercase();
        if text.is_empty() || is_badge_only(node) {
            continue;
        }
        match targets.get(&text) {
            Some(first) if *first != dest => {
                let message = format!("Link text \"{text}\" points to both {first} and {dest}");
                out.add(S33, &page.path, node.line, message);
            }
            Some(_) => {}
            None => {
                targets.insert(text, dest);
            }
        }
    }
}

fn images(page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        match &node.kind {
            Kind::Image { dest } if node.plain().trim().is_empty() => {
                out.add(
                    S34,
                    &page.path,
                    node.line,
                    format!("Image {dest} has no alt text"),
                );
            }
            Kind::Html => {
                for tag in IMAGE_TAG.find_iter(&node.text) {
                    if !FILLED_ALT.is_match(tag.as_str()) {
                        out.add(S34, &page.path, node.line, "HTML image has no alt text");
                    }
                }
            }
            _ => {}
        }
    }
}

fn scaffolding(page: &Page, out: &mut Findings) {
    let heading_lines: HashSet<usize> = page.headings().iter().map(|node| node.line).collect();
    for (line, text) in page.root.prose(true) {
        if let Some(found) = SCAFFOLDING
            .find(&text)
            .filter(|_| !heading_lines.contains(&line))
        {
            let message = format!("Unfinished text in prose: \"{}\"", found.as_str());
            out.add(T4, &page.path, line, message);
        }
    }
}

fn prerequisites_come_first(page: &Page, out: &mut Findings) {
    let is_steps = |node: &Node| {
        node.kind == (Kind::List { ordered: true })
            || (node.is_heading() && STEPS.is_match(node.plain().trim()))
    };
    let is_prerequisites =
        |node: &Node| node.is_heading() && PREREQUISITES.is_match(node.plain().trim());
    let blocks: Vec<&Node> = page.top_blocks().collect();
    let first_step = blocks.iter().position(|node| is_steps(node));
    let prerequisites = blocks.iter().position(|node| is_prerequisites(node));
    if let (Some(step), Some(needs)) = (first_step, prerequisites)
        && needs > step
    {
        let message = "Prerequisites come after the first step";
        out.add(S6, &page.path, blocks[needs].line, message);
    }
}

fn optional_steps(page: &Page, out: &mut Findings) {
    for list in page.root.all() {
        if list.kind != (Kind::List { ordered: true }) {
            continue;
        }
        for item in &list.children {
            let text = item.plain();
            if OPTIONAL.is_match(&text) && !text.trim_start().starts_with("Optional:") {
                let message = "A step that mentions \"optional\" doesn't start with `Optional:`";
                out.add(S30, &page.path, item.line, message);
            }
        }
    }
}

fn steps(page: &Page, out: &mut Findings) {
    if page.is_task_page() {
        prerequisites_come_first(page, out);
        optional_steps(page, out);
    }
}

fn decision_record(page: &Page, out: &mut Findings) {
    if !page
        .doc_type()
        .is_some_and(|kind| kind.starts_with("decision"))
    {
        return;
    }
    let text = page.root.plain().to_lowercase();
    let headings: Vec<String> = page
        .headings()
        .iter()
        .map(|heading| heading.plain().to_lowercase())
        .collect();
    let has_part = |part: &&str| {
        page.front.contains_key(*part)
            || headings.iter().any(|heading| heading.contains(*part))
            || text.contains(&format!("{part}:"))
    };
    let missing: Vec<&str> = DECISION_PARTS
        .iter()
        .filter(|part| !has_part(part))
        .copied()
        .collect();
    if !missing.is_empty() {
        let message = format!("Decision record is missing: {}", missing.join(", "));
        out.add(T15, &page.path, 1, message);
    }
}

fn top_folder(path: &str) -> Option<&str> {
    path.split_once('/').map(|(folder, _)| folder)
}

fn has_listed_type(page: &Page) -> bool {
    page.doc_type()
        .is_some_and(|kind| PAGE_TYPES.contains(&kind))
}

fn declared_types(pages: &[Page], out: &mut Findings) {
    let typed_pages = || pages.iter().filter(|page| has_listed_type(page));
    if typed_pages().next().is_none() {
        return;
    }
    let typed_folders: Vec<&str> = typed_pages()
        .filter_map(|page| top_folder(&page.path))
        .collect();
    for page in pages.iter().filter(|page| page.role == Role::Page) {
        match page.doc_type() {
            Some(kind) if !PAGE_TYPES.contains(&kind) => {
                out.add(
                    T2,
                    &page.path,
                    1,
                    format!("Page type \"{kind}\" isn't on the list"),
                );
            }
            None if top_folder(&page.path)
                .is_some_and(|folder| typed_folders.contains(&folder)) =>
            {
                let message = "Page has no type, and other pages in this folder declare one";
                out.add(T2, &page.path, 1, message);
            }
            _ => {}
        }
    }
}

fn section_order(page: &Page) -> Vec<String> {
    page.headings()
        .iter()
        .filter(|heading| heading.kind == Kind::Heading(2))
        .map(|heading| heading.plain().trim().to_lowercase())
        .collect()
}

fn reference_section_order(pages: &[Page], out: &mut Findings) {
    let references: Vec<&Page> = pages
        .iter()
        .filter(|page| page.doc_type() == Some("reference"))
        .collect();
    if references.len() < FEWEST_PAGES_TO_COMPARE {
        return;
    }
    let mut counts: BTreeMap<Vec<String>, usize> = BTreeMap::new();
    for page in &references {
        *counts.entry(section_order(page)).or_default() += 1;
    }
    let Some((usual, count)) = counts.iter().max_by_key(|(_, count)| **count) else {
        return;
    };
    if count * 2 <= references.len() {
        return;
    }
    for page in references
        .iter()
        .filter(|page| section_order(page) != *usual)
    {
        let message = format!("Sections differ from the usual order: {}", usual.join(", "));
        out.add(S9, &page.path, 1, message);
    }
}
