use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::Findings;
use super::markdown::{Kind, Node, Page, Role, file_name};
use super::names::{self, is_real_path, local_target};
use super::project::Project;
use super::rules::{G2, G3, G3_PROGRAM, G14, G15, G18, G20, G38, G38_COMMENT, G39};

const ROOT_INSTRUCTIONS: &[&str] = &["AGENTS.md", "CLAUDE.md", "GEMINI.md"];
const MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "package.json",
    "pubspec.yaml",
    "go.mod",
    "pyproject.toml",
    "Makefile",
    "justfile",
    "build.gradle",
    "pom.xml",
];
const CLAUDE_LINE_LIMIT: usize = 200;
const LONG_FILE_LINE_LIMIT: usize = 500;
const CODEX_CHAIN_BYTES: usize = 32 * 1024;
const NAME_LIMIT: usize = 64;
const DESCRIPTION_LIMIT: usize = 1024;
const REFERENCE_LINES_WITHOUT_CONTENTS: usize = 100;
const LINES_TO_FIND_CONTENTS: usize = 20;
const HIDDEN_COMMENT_WORDS: usize = 20;
const SHELL_LANGUAGES: &[&str] = &["sh", "bash", "zsh", "shell", "console", "terminal"];
const SHELL_WORDS: &[&str] = &[
    "cd", "export", "source", ".", "set", "unset", "echo", "exit", "eval", "exec", "test", "[",
    "if", "then", "else", "fi", "for", "do", "done", "while", "case", "esac", "alias", "true",
    "false", "read", "wait", "pushd", "popd", "time", "command", "which", "type", "ulimit", "trap",
    "sudo", "env", "{", "}", "(", ")",
];
const REFERENCE_FOLDERS: &[&str] = &["reference", "references"];

fn pattern(text: &str) -> Regex {
    Regex::new(text).expect("valid pattern")
}

static BUILD: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)\b(build|compile|assemble|make|bundle)\b"));
static TEST: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?i)\b(tests?|pytest|jest|vitest|rspec|ctest)\b"));
static LINT: LazyLock<Regex> = LazyLock::new(|| {
    pattern(
        r"(?i)\b(lint|clippy|analy[sz]e|fmt|format|eslint|ruff|check|vet|prettier|rubocop|tsc)\b",
    )
});
static AGENTS_IMPORT: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?m)(^|\s)@(\./)?AGENTS\.md\b"));
static IMPORT: LazyLock<Regex> = LazyLock::new(|| pattern(r"(?m)(?:^|\s)@([\w./-]+\.\w+)"));
static BASE64_RUN: LazyLock<Regex> = LazyLock::new(|| pattern(r"[A-Za-z0-9+/]{200,}={0,2}"));
static HIDDEN_STYLE: LazyLock<Regex> = LazyLock::new(|| {
    pattern(
        r#"(?i)display\s*:\s*none|visibility\s*:\s*hidden|font-size\s*:\s*0|opacity\s*:\s*0\b|<[a-z][^>]*\shidden(\s|=|>)"#,
    )
});
static CONTENTS: LazyLock<Regex> =
    LazyLock::new(|| pattern(r"(?im)^(#+\s*|\*\*)?(table of )?contents\b"));
static ENV_PREFIX: LazyLock<Regex> = LazyLock::new(|| pattern(r"^[A-Za-z_]\w*=\S*$"));

pub fn is_root_instruction(path: &str) -> bool {
    ROOT_INSTRUCTIONS.contains(&path)
}

fn is_hidden_char(found: char, before: Option<char>) -> bool {
    let joins_emoji = found == '\u{200D}' && before.is_some_and(|char| char as u32 >= 0x2190);
    let at_start = found == '\u{FEFF}' && before.is_none();
    !joins_emoji
        && !at_start
        && matches!(found as u32,
            0x200B..=0x200F | 0x202A..=0x202E | 0x2060..=0x2064 | 0x2066..=0x2069 | 0xFEFF
            | 0xE0000..=0xE007F)
}

fn hidden_chars(page: &Page, out: &mut Findings) {
    for (index, line) in page.text.lines().enumerate() {
        let mut before = (index > 0).then_some('\n');
        for found in line.chars() {
            if is_hidden_char(found, before) {
                let code = found as u32;
                out.add(
                    G38,
                    &page.path,
                    index + 1,
                    format!("Hidden character U+{code:04X}"),
                );
                break;
            }
            before = Some(found);
        }
        if BASE64_RUN.is_match(line) {
            out.add(G38, &page.path, index + 1, "Long encoded run in the text");
        }
    }
}

fn hidden_html(page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        if node.kind != Kind::Html {
            continue;
        }
        if HIDDEN_STYLE.is_match(&node.text) {
            out.add(
                G38,
                &page.path,
                node.line,
                "HTML that hides text from readers",
            );
        }
        if node.is_comment() && node.text.split_whitespace().count() > HIDDEN_COMMENT_WORDS {
            let message = "Long HTML comment: agents read it, and readers of the page don't";
            out.add(G38_COMMENT, &page.path, node.line, message);
        }
    }
}

pub fn check_hidden_text(page: &Page, out: &mut Findings) {
    hidden_chars(page, out);
    hidden_html(page, out);
}

fn is_instruction_name_in_wrong_case(path: &str) -> bool {
    let name = file_name(path);
    ROOT_INSTRUCTIONS
        .iter()
        .any(|exact| name.eq_ignore_ascii_case(exact) && name != *exact)
}

fn wrong_case_names(pages: &[Page], out: &mut Findings) {
    for page in pages
        .iter()
        .filter(|page| is_instruction_name_in_wrong_case(&page.path))
    {
        let message = "This doc shares a name with an instruction file on a case-insensitive disk";
        out.add(G39, &page.path, 1, message);
    }
}

fn command_text(root: &Node) -> String {
    let blocks = root.all();
    let code = blocks
        .iter()
        .filter(|node| matches!(node.kind, Kind::Code { .. }))
        .map(|node| node.text.clone());
    let labelled_spans = root.spans_in_blocks().into_iter().map(|(_, around)| around);
    code.chain(labelled_spans).collect::<Vec<_>>().join("\n")
}

fn imported_pages<'a>(page: &Page, pages: &'a [Page]) -> Vec<&'a Page> {
    let folder = Path::new(&page.path).parent().unwrap_or(Path::new(""));
    IMPORT
        .captures_iter(&page.text)
        .filter_map(|found| {
            let target = folder.join(found[1].trim_start_matches("./"));
            pages.iter().find(|other| Path::new(&other.path) == target)
        })
        .collect()
}

fn exact_commands(project: &Project, pages: &[Page], out: &mut Findings) {
    if !MANIFESTS
        .iter()
        .any(|name| project.has_file_or_folder(name))
    {
        return;
    }
    for page in pages.iter().filter(|page| is_root_instruction(&page.path)) {
        let mut code = command_text(&page.root);
        for imported in imported_pages(page, pages) {
            code.push('\n');
            code.push_str(&command_text(&imported.root));
        }
        let kinds = [("build", &*BUILD), ("test", &*TEST), ("lint", &*LINT)];
        let missing: Vec<&str> = kinds
            .iter()
            .filter(|(_, words)| !words.is_match(&code))
            .map(|(name, _)| *name)
            .collect();
        if !missing.is_empty() {
            let message = format!("No exact command for: {}", missing.join(", "));
            out.add(G2, &page.path, 1, message);
        }
    }
}

fn parent_folder(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(folder, _)| folder)
}

fn same_file(project: &Project, one: &str, other: &str) -> bool {
    let canonical = |path: &str| fs::canonicalize(project.root.join(path)).ok();
    canonical(one).is_some() && canonical(one) == canonical(other)
}

fn claude_imports_agents(project: &Project, pages: &[Page], out: &mut Findings) {
    for claude in pages
        .iter()
        .filter(|page| file_name(&page.path) == "CLAUDE.md")
    {
        let folder = parent_folder(&claude.path);
        let agents = match folder {
            "" => "AGENTS.md".to_string(),
            folder => format!("{folder}/AGENTS.md"),
        };
        if project.files.contains(&agents)
            && !AGENTS_IMPORT.is_match(&claude.text)
            && !same_file(project, &claude.path, &agents)
        {
            let message = "AGENTS.md is here too, and CLAUDE.md doesn't import it, so Claude Code won't read it";
            out.add(G15, &claude.path, 1, message);
        }
    }
}

fn line_limits(page: &Page, out: &mut Findings) {
    let lines = page.text.lines().count();
    let name = file_name(&page.path);
    let cursor_rule = page.path.contains(".cursor/rules/");
    if name == "CLAUDE.md" && lines >= CLAUDE_LINE_LIMIT {
        let message = format!("{lines} lines: keep it under {CLAUDE_LINE_LIMIT}");
        out.add(G14, &page.path, 1, message);
    }
    if cursor_rule && lines >= LONG_FILE_LINE_LIMIT {
        let message = format!("{lines} lines: keep a Cursor rule under {LONG_FILE_LINE_LIMIT}");
        out.add(G14, &page.path, 1, message);
    }
    let body = lines.saturating_sub(page.front_lines);
    if page.role == Role::Skill && body >= LONG_FILE_LINE_LIMIT {
        let message = format!("{body} lines of body: keep it under {LONG_FILE_LINE_LIMIT}");
        out.add(G14, &page.path, 1, message);
    }
}

fn chain_of(path: &str, sizes: &BTreeMap<&str, usize>) -> usize {
    let mut folder = parent_folder(path);
    let mut total = 0;
    loop {
        let file = match folder {
            "" => "AGENTS.md".to_string(),
            folder => format!("{folder}/AGENTS.md"),
        };
        total += sizes.get(file.as_str()).copied().unwrap_or(0);
        if folder.is_empty() {
            return total;
        }
        folder = parent_folder(folder);
    }
}

fn codex_chain(pages: &[Page], out: &mut Findings) {
    let agents: Vec<&Page> = pages
        .iter()
        .filter(|page| file_name(&page.path) == "AGENTS.md")
        .collect();
    let sizes: BTreeMap<&str, usize> = agents
        .iter()
        .map(|page| (page.path.as_str(), page.text.len()))
        .collect();
    for page in agents {
        let total = chain_of(&page.path, &sizes);
        if total > CODEX_CHAIN_BYTES {
            let message = format!(
                "The AGENTS.md chain to here is {total} bytes, over Codex's {CODEX_CHAIN_BYTES}"
            );
            out.add(G14, &page.path, 1, message);
        }
    }
}

fn skill_fields(page: &Page, out: &mut Findings) {
    let fields = [("name", NAME_LIMIT), ("description", DESCRIPTION_LIMIT)];
    for (field, limit) in fields {
        let length = page.front.get(field).map_or(0, |text| text.chars().count());
        if length > limit {
            let message = format!("Skill {field} is {length} characters, over {limit}");
            out.add(G18, &page.path, 1, message);
        }
    }
}

fn link_targets(page: &Page) -> BTreeSet<String> {
    page.root
        .all()
        .iter()
        .filter_map(|node| match &node.kind {
            Kind::Link { dest, .. } => local_target(dest, &page.path),
            _ => None,
        })
        .filter(|target| target.ends_with(".md"))
        .collect()
}

fn is_in_skill(path: &str, skill_folder: &str) -> bool {
    skill_folder.is_empty() || path.starts_with(&format!("{skill_folder}/"))
}

fn reference_files<'a>(skill: &Page, pages: &'a [Page]) -> Vec<&'a Page> {
    let folder = parent_folder(&skill.path);
    let linked = link_targets(skill);
    pages
        .iter()
        .filter(|page| page.role == Role::Page && is_in_skill(&page.path, folder))
        .filter(|page| {
            let inner = page.path.strip_prefix(folder).unwrap_or(&page.path);
            let inner = inner.trim_start_matches('/');
            let in_folder = REFERENCE_FOLDERS
                .iter()
                .any(|name| inner.starts_with(&format!("{name}/")));
            in_folder || linked.contains(&page.path)
        })
        .collect()
}

fn skill_references(skill: &Page, pages: &[Page], out: &mut Findings) {
    let linked = link_targets(skill);
    let references = reference_files(skill, pages);
    let is_reference = |path: &String| references.iter().any(|page| page.path == *path);
    for page in &references {
        for target in link_targets(page)
            .iter()
            .filter(|target| is_reference(target))
        {
            if !linked.contains(target) && *target != page.path {
                let message = format!("Links to {target}, a second level below SKILL.md");
                out.add(G20, &page.path, 1, message);
            }
        }
        let lines = page.text.lines().count();
        let opening: Vec<&str> = page.text.lines().take(LINES_TO_FIND_CONTENTS).collect();
        if lines > REFERENCE_LINES_WITHOUT_CONTENTS && !CONTENTS.is_match(&opening.join("\n")) {
            let message = format!("{lines} lines and no table of contents in the first lines");
            out.add(G20, &page.path, 1, message);
        }
    }
}

pub fn check_project(project: &Project, pages: &[Page], out: &mut Findings) {
    wrong_case_names(pages, out);
    exact_commands(project, pages, out);
    claude_imports_agents(project, pages, out);
    codex_chain(pages, out);
    for page in pages {
        if page.role == Role::Instruction {
            named_paths(project, page, out);
            commands(project, page, out);
        }
        line_limits(page, out);
        if page.role == Role::Skill {
            skill_fields(page, out);
            skill_references(page, pages, out);
        }
    }
}

fn named_paths(project: &Project, page: &Page, out: &mut Findings) {
    for (line, path) in names::missing_paths(project, page) {
        out.add(
            G3,
            &page.path,
            line,
            format!("Names {path}, which doesn't exist"),
        );
    }
    let folder = Path::new(&page.path).parent().unwrap_or(Path::new(""));
    for (index, line) in page.text.lines().enumerate() {
        for found in IMPORT.captures_iter(line) {
            let target = &found[1];
            if !project.root.join(folder).join(target).exists() && !target.contains('@') {
                out.add(
                    G3,
                    &page.path,
                    index + 1,
                    format!("Imports {target}, which doesn't exist"),
                );
            }
        }
    }
}

fn on_path(program: &str) -> bool {
    env::var_os("PATH")
        .is_some_and(|path| env::split_paths(&path).any(|folder| folder.join(program).is_file()))
}

fn program_of(line: &str) -> Option<&str> {
    line.split_whitespace()
        .find(|word| !ENV_PREFIX.is_match(word))
}

fn is_checkable_program(program: &str) -> bool {
    !SHELL_WORDS.contains(&program)
        && program.chars().next().is_some_and(char::is_alphanumeric)
        && program
            .chars()
            .all(|char| char.is_alphanumeric() || "-_.+".contains(char))
}

fn command_lines(node: &Node, language: &str) -> Vec<(usize, String)> {
    let mut lines = Vec::new();
    let mut continued = false;
    for (index, line) in node.text.lines().enumerate() {
        let line = line.trim();
        if line.contains("<<") {
            break;
        }
        let prompted = line.strip_prefix("$ ");
        let was_continued = continued;
        continued = line.ends_with('\\');
        let usable = language != "console" || prompted.is_some();
        if !was_continued && usable && !line.is_empty() && !line.starts_with('#') {
            lines.push((node.line + index + 1, prompted.unwrap_or(line).to_string()));
        }
    }
    lines
}

fn check_command(project: &Project, page: &Page, at: usize, program: &str, out: &mut Findings) {
    if program.contains('/') {
        if !is_real_path(project, &page.path, program) {
            out.add(
                G3,
                &page.path,
                at,
                format!("Runs {program}, which doesn't exist"),
            );
        }
    } else if is_checkable_program(program)
        && !on_path(program)
        && !project.has_file_or_folder(program)
    {
        let message = format!("Runs `{program}`, which isn't on the PATH or in the repo");
        out.add(G3_PROGRAM, &page.path, at, message);
    }
}

fn commands(project: &Project, page: &Page, out: &mut Findings) {
    for node in page.root.all() {
        let Kind::Code { language, .. } = &node.kind else {
            continue;
        };
        if !SHELL_LANGUAGES.contains(&language.as_str()) {
            continue;
        }
        for (at, line) in command_lines(node, language) {
            if let Some(program) = program_of(&line) {
                check_command(project, page, at, program, out);
            }
        }
    }
}
