use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, LinkType, Options, Parser, Tag};
use regex::Regex;
use yaml_rust2::{Yaml, YamlLoader};

static QUOTED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""[^"]*"|“[^”]*”"#).expect("valid pattern"));

const INSTRUCTION_NAMES: &[&str] = &["AGENTS.md", "CLAUDE.md", "GEMINI.md"];
const CHANGELOG_NAMES: &[&str] = &["CHANGELOG", "CHANGES", "HISTORY", "RELEASES", "NEWS"];
const TASK_TYPES: &[&str] = &["how-to", "how-to-guide", "tutorial", "quickstart"];

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Root,
    Heading(u8),
    Paragraph,
    Quote,
    Code { fenced: bool, language: String },
    Html,
    List { ordered: bool },
    Item,
    Table,
    Row,
    Cell,
    Link { dest: String, autolink: bool },
    Image { dest: String },
    Span,
    Text,
    Other,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: Kind,
    pub line: usize,
    pub text: String,
    pub children: Vec<Node>,
}

impl Node {
    fn new(kind: Kind, line: usize) -> Self {
        Node {
            kind,
            line,
            text: String::new(),
            children: Vec::new(),
        }
    }

    fn leaf(kind: Kind, line: usize, text: &str) -> Self {
        Node {
            text: text.to_string(),
            ..Node::new(kind, line)
        }
    }

    pub fn plain(&self) -> String {
        match self.kind {
            Kind::Text | Kind::Span => self.text.clone(),
            Kind::Code { .. } | Kind::Html => String::new(),
            _ => self.children.iter().map(Node::plain).collect(),
        }
    }

    pub fn all(&self) -> Vec<&Node> {
        let mut found = vec![self];
        for child in &self.children {
            found.extend(child.all());
        }
        found
    }

    pub fn is_comment(&self) -> bool {
        self.kind == Kind::Html && self.text.trim_start().starts_with("<!--")
    }

    pub fn is_heading(&self) -> bool {
        matches!(self.kind, Kind::Heading(_))
    }

    pub fn has(&self, wanted: impl Fn(&Kind) -> bool) -> bool {
        self.all().iter().any(|node| wanted(&node.kind))
    }

    pub fn prose(&self, cells: bool) -> Vec<(usize, String)> {
        let mut found = Vec::new();
        self.collect_prose(cells, &mut found);
        found
    }

    fn collect_prose(&self, cells: bool, found: &mut Vec<(usize, String)>) {
        match &self.kind {
            Kind::Quote | Kind::Code { .. } | Kind::Html | Kind::Span | Kind::Image { .. } => {}
            Kind::Link { autolink: true, .. } => {}
            Kind::Cell if !cells => {}
            Kind::Text => found.push((self.line, QUOTED.replace_all(&self.text, " ").into_owned())),
            _ => {
                for child in &self.children {
                    child.collect_prose(cells, found);
                }
            }
        }
    }

    pub fn spans_in_blocks(&self) -> Vec<(&Node, String)> {
        let mut found = Vec::new();
        self.collect_spans("", &mut found);
        found
    }

    fn collect_spans<'a>(&'a self, around: &str, found: &mut Vec<(&'a Node, String)>) {
        let own;
        let around = match self.kind {
            Kind::Paragraph | Kind::Heading(_) | Kind::Item | Kind::Row => {
                own = self.plain();
                own.as_str()
            }
            _ => around,
        };
        if self.kind == Kind::Span {
            found.push((self, around.to_string()));
        }
        for child in &self.children {
            child.collect_spans(around, found);
        }
    }
}

fn level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn kind_of(tag: &Tag) -> Kind {
    match tag {
        Tag::Heading { level, .. } => Kind::Heading(level_number(*level)),
        Tag::Paragraph => Kind::Paragraph,
        Tag::BlockQuote(_) => Kind::Quote,
        Tag::CodeBlock(CodeBlockKind::Fenced(info)) => Kind::Code {
            fenced: true,
            language: info.split([' ', ',', '{']).next().unwrap_or("").to_string(),
        },
        Tag::CodeBlock(CodeBlockKind::Indented) => Kind::Code {
            fenced: false,
            language: String::new(),
        },
        Tag::HtmlBlock => Kind::Html,
        Tag::List(start) => Kind::List {
            ordered: start.is_some(),
        },
        Tag::Item => Kind::Item,
        Tag::Table(_) => Kind::Table,
        Tag::TableHead | Tag::TableRow => Kind::Row,
        Tag::TableCell => Kind::Cell,
        Tag::Link {
            link_type,
            dest_url,
            ..
        } => Kind::Link {
            dest: dest_url.to_string(),
            autolink: matches!(link_type, LinkType::Autolink | LinkType::Email),
        },
        Tag::Image { dest_url, .. } => Kind::Image {
            dest: dest_url.to_string(),
        },
        _ => Kind::Other,
    }
}

fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_GFM
}

fn line_starts(text: &str) -> Vec<usize> {
    let breaks = text.match_indices('\n').map(|(at, _)| at + 1);
    std::iter::once(0).chain(breaks).collect()
}

pub fn parse(text: &str) -> Node {
    let starts = line_starts(text);
    let mut stack = vec![Node::new(Kind::Root, 1)];
    for (event, range) in Parser::new_ext(text, options()).into_offset_iter() {
        let line = starts.partition_point(|start| *start <= range.start);
        let top = stack.len() - 1;
        let in_block = matches!(stack[top].kind, Kind::Code { .. } | Kind::Html);
        match event {
            Event::Start(tag) => stack.push(Node::new(kind_of(&tag), line)),
            Event::End(_) if top > 0 => {
                let node = stack.remove(top);
                stack[top - 1].children.push(node);
            }
            Event::Text(text) | Event::Html(text) if in_block => stack[top].text.push_str(&text),
            Event::Text(text) => stack[top]
                .children
                .push(Node::leaf(Kind::Text, line, &text)),
            Event::Html(text) | Event::InlineHtml(text) => {
                stack[top]
                    .children
                    .push(Node::leaf(Kind::Html, line, &text));
            }
            Event::Code(text) | Event::InlineMath(text) | Event::DisplayMath(text) => {
                stack[top]
                    .children
                    .push(Node::leaf(Kind::Span, line, &text));
            }
            Event::SoftBreak | Event::HardBreak => {
                stack[top].children.push(Node::leaf(Kind::Text, line, " "));
            }
            Event::Rule => stack[top].children.push(Node::new(Kind::Other, line)),
            _ => {}
        }
    }
    stack.swap_remove(0)
}

fn yaml_text(value: &Yaml) -> Option<String> {
    match value {
        Yaml::String(text) | Yaml::Real(text) => Some(text.clone()),
        Yaml::Integer(number) => Some(number.to_string()),
        Yaml::Boolean(value) => Some(value.to_string()),
        _ => None,
    }
}

fn parse_front(yaml: &str) -> BTreeMap<String, String> {
    let documents = YamlLoader::load_from_str(yaml).unwrap_or_default();
    let Some(Yaml::Hash(fields)) = documents.first() else {
        return BTreeMap::new();
    };
    fields
        .iter()
        .filter_map(|(key, value)| Some((yaml_text(key)?, yaml_text(value)?)))
        .collect()
}

fn split_front(text: &str) -> Option<(&str, &str)> {
    let rest = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))?;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Some((&rest[..offset], &rest[offset + line.len()..]));
        }
        offset += line.len();
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Page,
    Instruction,
    Skill,
    Changelog,
}

fn is_cursor_rule(path: &str) -> bool {
    path.starts_with(".cursor/rules/") || path.contains("/.cursor/rules/")
}

fn role_of(path: &str, front: &BTreeMap<String, String>) -> Role {
    let name = file_name(path);
    let stem = name.split('.').next().unwrap_or("").to_uppercase();
    if INSTRUCTION_NAMES.contains(&name)
        || name == "copilot-instructions.md"
        || is_cursor_rule(path)
    {
        Role::Instruction
    } else if name == "SKILL.md" {
        Role::Skill
    } else if CHANGELOG_NAMES.contains(&stem.as_str())
        || stem.contains("CHANGELOG")
        || front.get("type").is_some_and(|kind| kind == "changelog")
    {
        Role::Changelog
    } else {
        Role::Page
    }
}

pub fn file_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
}

#[derive(Debug)]
pub struct Page {
    pub path: String,
    pub text: String,
    pub front: BTreeMap<String, String>,
    pub front_lines: usize,
    pub root: Node,
    pub role: Role,
}

impl Page {
    pub fn new(path: &str, text: String) -> Self {
        let (front, front_lines, body) = match split_front(&text) {
            Some((yaml, rest)) => {
                let lines = text[..text.len() - rest.len()].matches('\n').count();
                (parse_front(yaml), lines, "\n".repeat(lines) + rest)
            }
            None => (BTreeMap::new(), 0, text.clone()),
        };
        Page {
            path: path.to_string(),
            role: role_of(path, &front),
            root: parse(&body),
            text,
            front,
            front_lines,
        }
    }

    pub fn doc_type(&self) -> Option<&str> {
        self.front.get("type").map(String::as_str)
    }

    pub fn is_task_page(&self) -> bool {
        self.doc_type()
            .is_some_and(|kind| TASK_TYPES.contains(&kind))
    }

    pub fn is_history(&self) -> bool {
        self.role == Role::Changelog
            || self
                .doc_type()
                .is_some_and(|kind| kind.starts_with("migration"))
    }

    pub fn top_blocks(&self) -> impl Iterator<Item = &Node> {
        self.root.children.iter().filter(|node| !node.is_comment())
    }

    pub fn headings(&self) -> Vec<&Node> {
        self.root
            .children
            .iter()
            .filter(|node| node.is_heading())
            .collect()
    }
}
