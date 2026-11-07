use std::sync::LazyLock;

use regex::Regex;
use yaml_rust2::YamlLoader;

use super::Findings;
use super::markdown::{Kind, Node, Page, Role};
use super::rules::{A13, E1, E9, E10};

const TEMPLATE_MARKS: &[&str] = &["{{", "{%", "<%", "...", "…"];

static ANGLE_PLACEHOLDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<[\w -]+>").expect("valid pattern"));

pub fn check(page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        if let Kind::Code {
            fenced: true,
            language,
        } = &node.kind
        {
            if language.is_empty() {
                out.add(E9, &page.path, node.line, "Code block has no language tag");
            }
            parses(page, node, language, out);
        }
    }
    if page.role == Role::Page {
        lead_ins(&page.root, &page.path, out);
    }
    if page.is_task_page() && !page.root.has(|kind| matches!(kind, Kind::Code { .. })) {
        out.add(E1, &page.path, 1, "A task page has no code block");
    }
}

fn is_lead_in(before: &Node) -> bool {
    matches!(
        before.kind,
        Kind::Paragraph | Kind::List { .. } | Kind::Text | Kind::Html
    )
}

fn lead_ins(parent: &Node, path: &str, out: &mut Findings) {
    let blocks: Vec<&Node> = parent
        .children
        .iter()
        .filter(|node| !node.is_comment())
        .collect();
    for (index, node) in blocks.iter().enumerate() {
        if matches!(node.kind, Kind::Code { .. }) && parent.kind != Kind::Quote {
            let before = index.checked_sub(1).and_then(|at| blocks.get(at));
            if before.is_some_and(|before| !is_lead_in(before)) || index == 0 {
                out.add(
                    E10,
                    path,
                    node.line,
                    "No sentence comes before this code block",
                );
            }
        }
        lead_ins(node, path, out);
    }
}

fn is_template(code: &str) -> bool {
    TEMPLATE_MARKS.iter().any(|mark| code.contains(mark)) || ANGLE_PLACEHOLDER.is_match(code)
}

fn is_json(code: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(code).is_ok()
}

fn is_json_lines(code: &str) -> bool {
    code.lines()
        .filter(|line| !line.trim().is_empty())
        .all(is_json)
}

fn is_json_members(code: &str) -> bool {
    is_json(&format!("{{{code}}}"))
}

fn json_error(code: &str) -> Option<String> {
    if is_json_lines(code) || is_json_members(code) {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(code)
        .err()
        .map(|error| error.to_string())
}

fn parse_error(language: &str, code: &str) -> Option<String> {
    match language {
        "json" => json_error(code),
        "yaml" | "yml" => YamlLoader::load_from_str(code)
            .err()
            .map(|error| error.to_string()),
        "toml" => code
            .parse::<toml::Table>()
            .err()
            .map(|error| error.message().to_string()),
        _ => None,
    }
}

fn parses(page: &Page, node: &Node, language: &str, out: &mut Findings) {
    if is_template(&node.text) {
        return;
    }
    if let Some(error) = parse_error(&language.to_lowercase(), &node.text) {
        let message = format!("{language} block doesn't parse: {error}");
        out.add(A13, &page.path, node.line, message);
    }
}
