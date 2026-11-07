use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;

use aho_corasick::AhoCorasick;
use regex::Regex;

use super::Findings;
use super::markdown::{Kind, Node, Page, Role};
use super::project::Project;
use super::rules::{A4, A6, A7, A8, A10};

const FILE_EXTENSIONS: &[&str] = &[
    "md", "mdx", "rs", "dart", "toml", "yaml", "yml", "json", "lock", "ts", "tsx", "js", "jsx",
    "mjs", "py", "go", "rb", "java", "kt", "swift", "c", "h", "cpp", "sh", "txt", "html", "css",
    "sql", "proto", "xml", "gradle", "csv",
];
const CREATED_OR_EXAMPLE: &str = r"(?i)\b(e\.g\.|for example|such as|examples?|creates?|creating|your|named|called|generates?|writes?)\b";
const PLACEHOLDERS: &[&str] = &["path/to", "example", "foo", "my-", "your"];
const NOT_PATH_CHARS: &str = "<>{}[]()$*?|=;,'\"\\ \t@#%&!^";
const COMMIT_LENGTH: usize = 40;
const SHORT_COMMIT: usize = 8;
const DOC_PATHSPECS: &[&str] = &[":(exclude)*.md", ":(exclude)*.mdx", ":(exclude)*.markdown"];
const DOC_EXTENSIONS: &[&str] = &["md", "mdx", "markdown"];
const LARGEST_CODE_FILE: u64 = 8 * 1024 * 1024;
const BINARY_SNIFF_BYTES: usize = 8000;

fn pattern(text: &str) -> Regex {
    Regex::new(text).expect("valid pattern")
}

static MARKED: LazyLock<Regex> = LazyLock::new(|| pattern(CREATED_OR_EXAMPLE));
static SCHEME: LazyLock<Regex> = LazyLock::new(|| pattern(r"^[A-Za-z][A-Za-z0-9+.-]*:"));
static DOMAIN: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"^[\w-]+(\.[\w-]+)*\.(com|org|net|dev|io|ai|app|sh|co)(/|$)"));
static LINE_SUFFIX: LazyLock<Regex> = LazyLock::new(|| pattern(r":\d+([:-]\d+)?$"));
static EMAIL: LazyLock<Regex> = LazyLock::new(|| pattern(r"^[^/\s@]+@[^/\s@]+\.[A-Za-z]+$"));
static MDX_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| pattern(r#"^import\s.+\sfrom\s+['"](\.{1,2}/[^'"]+)['"]"#));
static CODE_NAME: LazyLock<Regex> = LazyLock::new(|| pattern(r"^[A-Za-z_][\w.:/-]*[A-Za-z0-9_]$"));
static CAMEL_CASE: LazyLock<Regex> = LazyLock::new(|| pattern(r"[a-z][A-Z]"));
static HTML_ANCHOR: LazyLock<Regex> =
    LazyLock::new(|| pattern(r#"(?i)\b(?:id|name)\s*=\s*["']([^"']+)["']"#));
static INCLUDES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"\{\{#(?:include|rustdoc_include)\s+([^\s}:]+)",
        r#"--8<--\s+"([^":]+)"#,
        r"^\s*`{3,}\{(?:literal)?include\}\s+(\S+)",
        r"<!--\s*@include:\s*([^\s{#]+)",
    ]
    .iter()
    .map(|text| pattern(text))
    .collect()
});

fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let hex = (bytes[index] == b'%')
            .then(|| text.get(index + 1..index + 3))
            .flatten()
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());
        match hex {
            Some(byte) => {
                decoded.push(byte);
                index += 3;
            }
            None => {
                decoded.push(bytes[index]);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn is_template(dest: &str) -> bool {
    dest.contains('{') || dest.contains('<') || dest.contains('%') && dest.contains("{%")
}

fn split_link(dest: &str) -> (&str, Option<&str>) {
    let (rest, anchor) = dest
        .split_once('#')
        .map_or((dest, None), |(rest, anchor)| (rest, Some(anchor)));
    (rest.split('?').next().unwrap_or(""), anchor)
}

fn is_relative(dest: &str) -> bool {
    !dest.is_empty() && !SCHEME.is_match(dest) && !dest.starts_with("//") && !is_template(dest)
}

fn tidy(path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    for part in path.components() {
        match part {
            Component::ParentDir if parts.last().is_some_and(|last| last != "..") => {
                parts.pop();
            }
            Component::ParentDir => parts.push("..".to_string()),
            Component::Normal(name) => parts.push(name.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    parts.join("/")
}

fn folder_of(path: &str) -> &Path {
    Path::new(path).parent().unwrap_or(Path::new(""))
}

pub fn local_target(dest: &str, page: &str) -> Option<String> {
    let (file, _) = split_link(dest);
    (is_relative(dest) && !file.is_empty() && !file.starts_with('/'))
        .then(|| tidy(&folder_of(page).join(decode(file))))
}

fn site_forms(path: &Path) -> Vec<PathBuf> {
    let html_as_md = path.with_extension("md");
    let text = path.to_string_lossy();
    let text = text.trim_end_matches('/');
    let mut forms = vec![path.to_path_buf()];
    for suffix in [".md", ".mdx", "/index.md", "/index.mdx", "/README.md"] {
        forms.push(PathBuf::from(format!("{text}{suffix}")));
    }
    if path
        .extension()
        .is_some_and(|extension| extension == "html")
    {
        forms.push(html_as_md);
    }
    forms
}

fn target_exists(project: &Project, page: &str, file: &str) -> Option<bool> {
    let decoded = decode(file);
    match decoded.strip_prefix('/') {
        Some(rooted) => {
            let found = site_forms(&project.root.join(rooted))
                .iter()
                .any(|form| form.exists());
            found.then_some(true)
        }
        None => {
            let path = project.root.join(folder_of(page)).join(&decoded);
            Some(site_forms(&path).iter().any(|form| form.exists()))
        }
    }
}

fn link_nodes(page: &Page) -> impl Iterator<Item = (&Node, &str)> {
    page.root
        .all()
        .into_iter()
        .filter_map(|node| match &node.kind {
            Kind::Link { dest, .. } | Kind::Image { dest } => Some((node, dest.as_str())),
            _ => None,
        })
}

pub fn check_links(project: &Project, page: &Page, out: &mut Findings) {
    for (node, dest) in link_nodes(page) {
        let (file, _) = split_link(dest);
        if is_relative(dest)
            && !EMAIL.is_match(dest)
            && !file.is_empty()
            && target_exists(project, &page.path, file) == Some(false)
        {
            out.add(
                A6,
                &page.path,
                node.line,
                format!("Link target {file} doesn't exist"),
            );
        }
    }
}

fn github_slug(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphanumeric() || matches!(char, ' ' | '-' | '_'))
        .map(|char| if char == ' ' { '-' } else { char })
        .collect()
}

fn anchors_of(page: &Page) -> BTreeSet<String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut anchors = BTreeSet::new();
    for node in page.root.all() {
        if node.is_heading() {
            let slug = github_slug(&node.plain());
            let count = seen.entry(slug.clone()).or_default();
            anchors.insert(match *count {
                0 => slug,
                count => format!("{slug}-{count}"),
            });
            *count += 1;
        } else if node.kind == Kind::Html {
            let named = HTML_ANCHOR.captures_iter(&node.text);
            anchors.extend(named.map(|found| found[1].to_lowercase()));
        }
    }
    anchors
}

pub fn check_anchors(pages: &[Page], out: &mut Findings) {
    let anchors: BTreeMap<&str, BTreeSet<String>> = pages
        .iter()
        .map(|page| (page.path.as_str(), anchors_of(page)))
        .collect();
    for page in pages {
        for (node, dest) in link_nodes(page) {
            let (file, anchor) = split_link(dest);
            let Some(anchor) = anchor.filter(|anchor| !anchor.is_empty() && is_relative(dest))
            else {
                continue;
            };
            let target = match file {
                "" => Some(page.path.clone()),
                file => local_target(file, &page.path),
            };
            let known = target.as_deref().and_then(|path| anchors.get(path));
            if known.is_some_and(|known| !known.contains(&decode(anchor).to_lowercase())) {
                let shown = target.unwrap_or_default();
                let message = format!("No heading in {shown} matches #{anchor}");
                out.add(A7, &page.path, node.line, message);
            }
        }
    }
}

fn mdx_imports(page: &Page) -> Vec<(usize, &str)> {
    if !page.path.ends_with(".mdx") {
        return Vec::new();
    }
    let mut in_code = false;
    let mut imports = Vec::new();
    for (index, line) in page.text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
        } else if let Some(found) = MDX_IMPORT.captures(line).filter(|_| !in_code) {
            imports.extend(found.get(1).map(|target| (index + 1, target.as_str())));
        }
    }
    imports
}

fn include_targets(page: &Page) -> Vec<(usize, &str)> {
    let mut targets = mdx_imports(page);
    for (index, line) in page.text.lines().enumerate() {
        let found = INCLUDES.iter().filter_map(|include| include.captures(line));
        targets.extend(found.filter_map(|found| Some((index + 1, found.get(1)?.as_str()))));
    }
    targets
}

pub fn check_includes(project: &Project, page: &Page, out: &mut Findings) {
    let folder = project.root.join(folder_of(&page.path));
    for (line, target) in include_targets(page) {
        let places = [folder.join(target), project.root.join(target)];
        if !places.iter().any(|place| place.exists()) {
            let message = format!("Include target {target} doesn't exist");
            out.add(A4, &page.path, line, message);
        }
    }
}

fn clean_path(span: &str) -> Option<String> {
    let span = span.trim();
    let span = LINE_SUFFIX.replace(span, "");
    let invalid = span.is_empty()
        || SCHEME.is_match(&span)
        || span.starts_with(['/', '~', '-', '.'])
            && !span.starts_with("./")
            && !span.starts_with("../")
        || span.contains("://")
        || span.contains("...")
        || span.contains(|char| NOT_PATH_CHARS.contains(char))
        || DOMAIN.is_match(&span)
        || PLACEHOLDERS
            .iter()
            .any(|mark| span.to_lowercase().contains(mark));
    (!invalid).then(|| span.into_owned())
}

fn has_file_extension(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| FILE_EXTENSIONS.contains(&extension))
}

fn first_part_exists(project: &Project, path: &str) -> bool {
    let first = path.split('/').next().unwrap_or("");
    project.root.join(first).exists()
}

fn path_in_span(project: &Project, span: &str) -> Option<String> {
    let path = clean_path(span)?;
    let shaped = path.trim_end_matches('/').contains('/')
        && (path.ends_with('/')
            || has_file_extension(&path)
            || path.starts_with("./")
            || path.starts_with("../")
            || first_part_exists(project, &path));
    shaped.then_some(path)
}

pub fn is_real_path(project: &Project, page: &str, path: &str) -> bool {
    let trimmed = path.trim_end_matches('/');
    let bare = trimmed.trim_start_matches("./");
    project.root.join(folder_of(page)).join(trimmed).exists()
        || project.root.join(bare).exists()
        || project.has_file_or_folder(bare)
        || project.is_ignored(bare)
}

pub fn missing_paths(project: &Project, page: &Page) -> Vec<(usize, String)> {
    let mut missing = Vec::new();
    for (span, around) in page.root.spans_in_blocks() {
        if MARKED.is_match(&around) {
            continue;
        }
        if let Some(path) = path_in_span(project, &span.text)
            && !is_real_path(project, &page.path, &path)
        {
            missing.push((span.line, path));
        }
    }
    missing
}

pub fn check_paths(project: &Project, page: &Page, out: &mut Findings) {
    if page.role == Role::Instruction || page.is_history() {
        return;
    }
    for (line, path) in missing_paths(project, page) {
        out.add(
            A8,
            &page.path,
            line,
            format!("Names {path}, which isn't in the repo"),
        );
    }
}

fn code_name(project: &Project, span: &str) -> Option<String> {
    let name = span.trim().trim_end_matches("()");
    let shaped = CODE_NAME.is_match(name)
        && (name.contains(['_', '.', '/']) || name.contains("::") || CAMEL_CASE.is_match(name));
    let names_a_file = has_file_extension(name) || path_in_span(project, name).is_some();
    (shaped && !names_a_file).then(|| name.to_string())
}

fn is_doc_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| DOC_EXTENSIONS.contains(&extension))
}

fn read_code_file(project: &Project, path: &str) -> Option<Vec<u8>> {
    let place = project.root.join(path);
    let size = fs::metadata(&place).ok()?.len();
    if size > LARGEST_CODE_FILE || is_doc_file(path) {
        return None;
    }
    let bytes = fs::read(place).ok()?;
    let start = &bytes[..bytes.len().min(BINARY_SNIFF_BYTES)];
    (!start.contains(&0)).then_some(bytes)
}

fn names_in_code_now(project: &Project, names: &BTreeSet<&str>) -> BTreeSet<String> {
    let names: Vec<&str> = names.iter().copied().collect();
    let Ok(finder) = AhoCorasick::new(&names) else {
        return BTreeSet::new();
    };
    let mut found = BTreeSet::new();
    for code in project
        .files
        .iter()
        .filter_map(|path| read_code_file(project, path))
    {
        for hit in finder.find_overlapping_iter(&code) {
            found.insert(names[hit.pattern().as_usize()].to_string());
        }
    }
    found
}

fn names_in_code(project: &Project, names: &BTreeSet<&str>, commit: &str) -> BTreeSet<String> {
    let patterns = names.iter().copied().collect::<Vec<_>>().join("\n");
    let mut args = vec!["grep", "-I", "-F", "-o", "-h", "-f", "-", commit, "--", "."];
    args.extend(DOC_PATHSPECS);
    let Some(output) = project.git_with_input(&args, &patterns) else {
        return BTreeSet::new();
    };
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().map(str::to_string).collect()
}

fn is_commit_line(line: &str) -> bool {
    line.split(' ').next().is_some_and(|word| {
        word.len() == COMMIT_LENGTH && word.chars().all(|char| char.is_ascii_hexdigit())
    })
}

fn commits_by_line(project: &Project, page: &str) -> BTreeMap<usize, String> {
    let args = ["blame", "--no-textconv", "--porcelain", "--", page];
    let Some(output) = project.git(&args).filter(|output| output.status.success()) else {
        return BTreeMap::new();
    };
    let text = String::from_utf8_lossy(&output.stdout);
    let mut commits = BTreeMap::new();
    for line in text.lines().filter(|line| is_commit_line(line)) {
        let mut words = line.split(' ');
        let (commit, at) = (words.next(), words.nth(1).and_then(|at| at.parse().ok()));
        if let (Some(commit), Some(at)) = (commit, at)
            && !commit.chars().all(|char| char == '0')
        {
            commits.insert(at, commit.to_string());
        }
    }
    commits
}

struct Mention<'a> {
    page: &'a str,
    line: usize,
    name: String,
    commit: String,
}

fn mentions_gone_from_head<'a>(project: &Project, pages: &'a [Page]) -> Vec<Mention<'a>> {
    let mut named: Vec<(&Page, usize, String)> = Vec::new();
    for page in pages.iter().filter(|page| !page.is_history()) {
        for (span, _) in page.root.spans_in_blocks() {
            if let Some(name) = code_name(project, &span.text) {
                named.push((page, span.line, name));
            }
        }
    }
    let names: BTreeSet<&str> = named.iter().map(|(_, _, name)| name.as_str()).collect();
    if names.is_empty() {
        return Vec::new();
    }
    let now = names_in_code_now(project, &names);
    let mut blames: BTreeMap<&str, BTreeMap<usize, String>> = BTreeMap::new();
    let mut mentions = Vec::new();
    for (page, line, name) in named.iter().filter(|(_, _, name)| !now.contains(name)) {
        let path = page.path.as_str();
        let commits = blames
            .entry(path)
            .or_insert_with(|| commits_by_line(project, path));
        if let Some(commit) = commits.get(line) {
            mentions.push(Mention {
                page: path,
                line: *line,
                name: name.clone(),
                commit: commit.clone(),
            });
        }
    }
    mentions
}

pub fn check_vanished(project: &Project, pages: &[Page], out: &mut Findings) {
    let mentions = mentions_gone_from_head(project, pages);
    let mut by_commit: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for mention in &mentions {
        by_commit
            .entry(&mention.commit)
            .or_default()
            .insert(&mention.name);
    }
    let then: BTreeMap<&str, BTreeSet<String>> = by_commit
        .iter()
        .map(|(commit, names)| (*commit, names_in_code(project, names, commit)))
        .collect();
    for Mention {
        page,
        line,
        name,
        commit,
    } in &mentions
    {
        let was_there = then
            .get(commit.as_str())
            .is_some_and(|names| names.contains(name));
        if was_there {
            let short = &commit[..commit.len().min(SHORT_COMMIT)];
            let message = format!(
                "`{name}` was in the code at {short}, when this line last changed, and isn't now"
            );
            out.add(A10, page, *line, message);
        }
    }
}
