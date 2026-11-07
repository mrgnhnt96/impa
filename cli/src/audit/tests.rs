use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

use super::{Renderer, audit};

fn write(root: &Path, path: &str, text: &str) {
    let file = root.join(path);
    fs::create_dir_all(file.parent().expect("parent")).expect("create folder");
    fs::write(file, text).expect("write file");
}

fn project(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("temp dir");
    for (path, text) in files {
        write(dir.path(), path, text);
    }
    dir
}

fn found_in(dir: &Path) -> Vec<String> {
    let report = audit(dir, &[], Some(Renderer::Github)).expect("audit runs");
    report
        .findings
        .iter()
        .map(|finding| format!("{} {}:{}", finding.rule.id, finding.file, finding.line))
        .collect()
}

fn found(files: &[(&str, &str)]) -> Vec<String> {
    found_in(project(files).path())
}

fn page(text: &str) -> Vec<String> {
    found(&[("page.md", text)])
}

fn rule_on(found: &[String], rule: &str) -> bool {
    found
        .iter()
        .any(|line| line.starts_with(&format!("{rule} ")))
}

#[track_caller]
fn fires(found: &[String], rule: &str) {
    assert!(rule_on(found, rule), "expected {rule} in {found:?}");
}

#[track_caller]
fn silent(found: &[String], rule: &str) {
    assert!(!rule_on(found, rule), "unexpected {rule} in {found:?}");
}

fn git(root: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(args)
        .output()
        .expect("run git")
        .status;
    assert!(status.success(), "git {args:?}");
}

#[test]
fn opening_that_talks_about_the_page() {
    fires(&page("# Tool\n\nThis guide explains the tool.\n"), "S2");
    silent(&page("# Tool\n\nTool sorts files by size.\n"), "S2");
}

#[test]
fn opening_with_a_notice_or_badge() {
    fires(&page("# Tool\n\n> Note: beta.\n\nTool sorts.\n"), "S3");
    fires(
        &page("# Tool\n\n[![ci](https://a.b/c.svg)](https://a.b)\n\nTool sorts.\n"),
        "S3",
    );
    silent(&page("# Tool\n\nTool sorts.\n\n> Note: beta.\n"), "S3");
}

#[test]
fn one_h1_and_no_skipped_levels() {
    fires(&page("# One\n\nText.\n\n# Two\n\nText.\n"), "S12");
    fires(&page("# One\n\nText.\n\n### Deep\n\nText.\n"), "S12");
    fires(&page("Text with no title.\n"), "S12");
    silent(
        &page("---\ntitle: One\n---\n\nText.\n\n## Part\n\nText.\n"),
        "S12",
    );
    silent(
        &page("<h1 align=\"center\">One</h1>\n\nText.\n\n## Part\n\nText.\n"),
        "S12",
    );
}

#[test]
fn heading_words() {
    fires(&page("# Tool\n\nText.\n\n## Overview\n\nText.\n"), "S14");
    silent(
        &page("# Tool\n\nText.\n\n## Overview of ports\n\nText.\n"),
        "S14",
    );
    fires(&page("# Tool\n\nText.\n\n## Done.\n\nText.\n"), "S15");
    fires(
        &page("# Tool\n\nText.\n\n## See [more](a.md)\n\nText.\n"),
        "S15",
    );
    silent(
        &page("# Tool\n\nText.\n\n## The `run` command\n\nText.\n"),
        "S15",
    );
    fires(
        &page("---\ntype: how-to\n---\n\n# Tool\n\n```sh\nls\n```\n\n## The `run` part\n\nText.\n"),
        "S15",
    );
}

#[test]
fn headings_need_text_under_them() {
    fires(&page("# Tool\n\n## Part\n\nText.\n"), "S16");
    fires(
        &page("# Tool\n\nText.\n\n## Empty\n\n## Full\n\nText.\n"),
        "T4",
    );
    fires(&page("# Tool\n\nText.\n\n## Empty\n"), "T4");
}

#[test]
fn scaffolding_in_prose_only() {
    fires(&page("# Tool\n\nTODO fill this in.\n"), "T4");
    fires(&page("# Tool\n\nLorem ipsum dolor sit amet.\n"), "T4");
    fires(&page("# Tool\n\nMore is coming soon.\n"), "T4");
    silent(
        &page("# Tool\n\nFind each `TODO` left in the code.\n"),
        "T4",
    );
    silent(&page("# Tool\n\nThe word \"TODO\" is banned.\n"), "T4");
    silent(
        &page("# Tool\n\nNever leave lorem ipsum in a page.\n"),
        "T4",
    );
}

#[test]
fn one_item_lists() {
    fires(&page("# Tool\n\nText:\n\n- one\n"), "S20");
    silent(&page("# Tool\n\nText:\n\n- one\n- two\n"), "S20");
}

#[test]
fn tables_need_two_columns_and_filled_cells() {
    fires(&page("# Tool\n\n| A |\n| --- |\n| 1 |\n"), "S25");
    fires(
        &page("# Tool\n\n| A | B |\n| --- | --- |\n| 1 | |\n"),
        "S25",
    );
    fires(&page("# Tool\n\n| | |\n| --- | --- |\n| 1 | 2 |\n"), "S25");
    silent(
        &page("# Tool\n\n| A | B |\n| --- | --- |\n| 1 | `2` |\n"),
        "S25",
    );
}

#[test]
fn link_text_stands_alone() {
    fires(&page("# Tool\n\nRead [here](https://a.b).\n"), "S32");
    fires(&page("# Tool\n\nSee https://a.b/c now.\n"), "S32");
    fires(&page("# Tool\n\nSee <https://a.b/c> now.\n"), "S32");
    silent(&page("# Tool\n\nRead [the guide](https://a.b).\n"), "S32");
    silent(&page("# Tool\n\nRead [a.md](a.md).\n\n"), "S32");
    silent(
        &page("# Tool\n\n| A | B |\n| --- | --- |\n| 1 | https://a.b/c |\n"),
        "S32",
    );
    silent(&page("# Tool\n\nRun `curl https://a.b/c` now.\n"), "S32");
}

#[test]
fn same_link_text_goes_to_one_place() {
    fires(
        &page("# Tool\n\n[guide](https://a.b) and [guide](https://c.d).\n"),
        "S33",
    );
    silent(
        &page("# Tool\n\n[guide](https://a.b) and [Guide](https://a.b).\n"),
        "S33",
    );
}

#[test]
fn images_need_alt_text() {
    let files = [
        ("a.png", ""),
        ("page.md", "# Tool\n\nText.\n\n![](a.png)\n"),
    ];
    fires(&found(&files), "S34");
    fires(&page("# Tool\n\nText.\n\n<img src=\"a.png\">\n"), "S34");
    silent(
        &page("# Tool\n\nText.\n\n<img src=\"a.png\" alt=\"A map\">\n"),
        "S34",
    );
}

#[test]
fn code_blocks_need_a_language_and_a_lead_in() {
    fires(&page("# Tool\n\nRun:\n\n```\nls\n```\n"), "E9");
    silent(&page("# Tool\n\nRun:\n\n```sh\nls\n```\n"), "E9");
    fires(
        &page("# Tool\n\nText.\n\n## Run\n\n```sh\nls\n```\n"),
        "E10",
    );
    silent(&page("# Tool\n\nRun:\n\n```sh\nls\n```\n"), "E10");
    silent(
        &page("# Tool\n\n1. Run:\n\n   ```sh\n   ls\n   ```\n2. Stop.\n"),
        "E10",
    );
}

#[test]
fn task_pages_show_code() {
    let head = "---\ntype: how-to\n---\n\n# Do it\n\n";
    fires(&page(&format!("{head}Just do it.\n")), "E1");
    silent(&page(&format!("{head}Run:\n\n```sh\nls\n```\n")), "E1");
    silent(&page("# Reference\n\nNo code here.\n"), "E1");
}

#[test]
fn prerequisites_come_before_steps() {
    let head = "---\ntype: how-to\n---\n\n# Do it\n\n";
    fires(
        &page(&format!(
            "{head}1. One\n2. Two\n\n## Prerequisites\n\nA shell.\n"
        )),
        "S6",
    );
    silent(
        &page(&format!(
            "{head}## Prerequisites\n\nA shell.\n\n## Steps\n\n1. One\n2. Two\n"
        )),
        "S6",
    );
    silent(
        &page("# Tool\n\n1. One\n2. Two\n\n## Prerequisites\n\nA shell.\n"),
        "S6",
    );
}

#[test]
fn optional_steps_say_so_first() {
    let head = "---\ntype: how-to\n---\n\n# Do it\n\n";
    fires(
        &page(&format!("{head}1. Run it.\n2. Optionally, log it.\n")),
        "S30",
    );
    silent(
        &page(&format!("{head}1. Run it.\n2. Optional: log it.\n")),
        "S30",
    );
}

#[test]
fn page_types() {
    let unknown = "---\ntype: blog\n---\n\n# Tool\n\nText.\n";
    let typed = "---\ntype: reference\n---\n\n# Tool\n\nText.\n";
    fires(
        &found(&[("docs/a.md", unknown), ("docs/b.md", typed)]),
        "T2",
    );
    silent(&found(&[("docs/a.md", unknown)]), "T2");
    let untyped = "# Tool\n\nText.\n";
    fires(
        &found(&[("docs/a.md", typed), ("docs/b.md", untyped)]),
        "T2",
    );
    silent(
        &found(&[("docs/a.md", untyped), ("docs/b.md", untyped)]),
        "T2",
    );
    silent(
        &found(&[("docs/a.md", typed), ("README.md", untyped)]),
        "T2",
    );
}

#[test]
fn reference_pages_share_their_order() {
    let usual =
        "---\ntype: reference\n---\n\n# A\n\nText.\n\n## Args\n\nText.\n\n## Exit\n\nText.\n";
    let other =
        "---\ntype: reference\n---\n\n# A\n\nText.\n\n## Exit\n\nText.\n\n## Args\n\nText.\n";
    let pages = [("a.md", usual), ("b.md", usual), ("c.md", other)];
    assert!(found(&pages).contains(&"S9 c.md:1".to_string()));
    silent(&found(&[("a.md", usual), ("c.md", other)]), "S9");
}

#[test]
fn decision_records_have_their_parts() {
    let head = "---\ntype: decision\n---\n\n# Use Rust\n\n";
    fires(&page(&format!("{head}## Context\n\nText.\n")), "T15");
    let whole =
        "Status: accepted\n\n## Context\n\nA.\n\n## Decision\n\nB.\n\n## Consequences\n\nC.\n";
    silent(&page(&format!("{head}{whole}")), "T15");
}

#[test]
fn config_blocks_parse() {
    fires(&page("# Tool\n\nSet:\n\n```json\n{ \"a\": \n```\n"), "A13");
    fires(&page("# Tool\n\nSet:\n\n```toml\na = \n```\n"), "A13");
    fires(&page("# Tool\n\nSet:\n\n```yaml\na: [1\n```\n"), "A13");
    silent(
        &page("# Tool\n\nSet:\n\n```json\n{ \"a\": 1 }\n```\n"),
        "A13",
    );
    silent(
        &page("# Tool\n\nSet:\n\n```json\n{ \"a\": {{ value }} }\n```\n"),
        "A13",
    );
    silent(&page("# Tool\n\nSet:\n\n```yaml\na: 1\n...\n```\n"), "A13");
}

#[test]
fn links_resolve_from_the_page() {
    fires(&page("# Tool\n\nRead [the guide](gone.md).\n"), "A6");
    let files = [
        (
            "docs/a.md",
            "# A\n\nRead [b](b.md), [root](../README.md), [site](b), [c](b.md?x=1#y).\n",
        ),
        ("docs/b.md", "# B\n\nText.\n"),
        ("README.md", "# R\n\nText.\n"),
    ];
    silent(&found(&files), "A6");
    silent(
        &page("# Tool\n\nRead [site](/docs/gone) and [mail](mailto:a@b.c).\n"),
        "A6",
    );
    silent(&page("# Tool\n\nRead [tmpl]({{ base }}/a.md).\n"), "A6");
}

#[test]
fn encoded_link_targets_are_decoded() {
    let files = [
        ("page.md", "# Tool\n\nRead [it](my%20page.md).\n"),
        ("my page.md", "# M\n\nText.\n"),
    ];
    silent(&found(&files), "A6");
}

#[test]
fn anchors_match_github_slugs() {
    let text = "# Tool\n\nGo to [a](#the-part) and [b](#the-part-1) and [c](#gone).\n\n## The part\n\nA.\n\n## The part\n\nB.\n";
    let result = page(text);
    assert_eq!(
        result.iter().filter(|line| line.starts_with("A7 ")).count(),
        1,
        "{result:?}"
    );
    let files = [
        (
            "page.md",
            "# Tool\n\nGo to [a](b.md#gone) and [b](b.md#real) and [c](b.md#spot).\n",
        ),
        (
            "b.md",
            "# B\n\nText.\n\n## Real\n\n<a id=\"spot\"></a>\n\nText.\n",
        ),
    ];
    let result = found(&files);
    assert_eq!(
        result.iter().filter(|line| line.starts_with("A7 ")).count(),
        1,
        "{result:?}"
    );
}

#[test]
fn anchors_are_left_alone_without_a_renderer() {
    let dir = project(&[("page.md", "# Tool\n\nGo to [a](#gone).\n")]);
    let report = audit(dir.path(), &[], None).expect("audit runs");
    assert!(
        report
            .findings
            .iter()
            .all(|finding| finding.rule.id != "A7")
    );
    assert!(report.not_run.iter().any(|reason| reason.starts_with("A7")));
}

#[test]
fn include_targets_exist() {
    fires(&page("# Tool\n\nText.\n\n{{#include gone.rs}}\n"), "A4");
    fires(&page("# Tool\n\nText.\n\n--8<-- \"gone.md\"\n"), "A4");
    let files = [
        ("page.md", "# Tool\n\nText.\n\n--8<-- \"part.md\"\n"),
        ("part.md", "Text.\n"),
    ];
    silent(&found(&files), "A4");
}

#[test]
fn named_paths_exist() {
    fires(&page("# Tool\n\nThe code is in `src/gone.rs`.\n"), "A8");
    let files = [
        (
            "page.md",
            "# Tool\n\nThe code is in `src/main.rs:12` and `src/main.rs:20-30`.\n",
        ),
        ("src/main.rs", ""),
    ];
    silent(&found(&files), "A8");
    silent(
        &page("# Tool\n\nFor example, `src/gone.rs` could hold it.\n"),
        "A8",
    );
    silent(&page("# Tool\n\nThis creates `out/data.json`.\n"), "A8");
    silent(
        &page("# Tool\n\nEdit `path/to/file.rs` or `foo/bar.rs`.\n"),
        "A8",
    );
    silent(
        &page("# Tool\n\nRead `README.md` and `AGENTS.md` in each project.\n"),
        "A8",
    );
    silent(
        &page("# Tool\n\nFetch `github.com/a/b.json` and `https://a.b/c.md`.\n"),
        "A8",
    );
}

#[test]
fn history_pages_keep_old_names() {
    let text = "---\ntype: changelog\n---\n\n# Changes\n\nMoved `src/old.rs` away.\n";
    silent(&page(text), "A8");
}

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", message]);
}

#[test]
fn vanished_names_are_candidates() {
    let dir = project(&[
        ("src/lib.rs", "fn old_name() {}\nfn kept_name() {}\n"),
        (
            "README.md",
            "# Tool\n\nCall `old_name` and `kept_name` and `never_there`.\n",
        ),
    ]);
    let root = dir.path();
    git(root, &["init", "-q"]);
    commit_all(root, "first");
    write(root, "src/lib.rs", "fn new_name() {}\nfn kept_name() {}\n");
    commit_all(root, "rename");
    let report = audit(root, &[], None).expect("audit runs");
    let vanished: Vec<&str> = report
        .findings
        .iter()
        .filter(|finding| finding.rule.id == "A10")
        .map(|finding| finding.message.as_str())
        .collect();
    assert_eq!(vanished.len(), 1, "{vanished:?}");
    assert!(vanished[0].contains("old_name"));
}

#[test]
fn no_history_means_a10_says_it_did_not_run() {
    let dir = project(&[("page.md", "# Tool\n\nText.\n")]);
    let report = audit(dir.path(), &[], None).expect("audit runs");
    assert!(
        report
            .not_run
            .iter()
            .any(|reason| reason.starts_with("A10"))
    );
}

#[test]
fn git_ignored_paths_are_not_missing() {
    let dir = project(&[
        (".gitignore", "build/\n"),
        (
            "README.md",
            "# Tool\n\nThe binary lands in `build/tool.json`.\n",
        ),
    ]);
    git(dir.path(), &["init", "-q"]);
    silent(&found_in(dir.path()), "A8");
}

#[test]
fn instruction_files_name_their_commands() {
    let manifest = ("Cargo.toml", "[package]\nname = \"a\"\n");
    let bare = ("AGENTS.md", "# Rules\n\nBe kind.\n");
    fires(&found(&[manifest, bare]), "G2");
    silent(&found(&[bare]), "G2");
    let full = (
        "AGENTS.md",
        "# Rules\n\nRun `cargo build`, `cargo test` and `cargo clippy`.\n",
    );
    silent(&found(&[manifest, full]), "G2");
    let split = [
        manifest,
        ("CLAUDE.md", "# Rules\n\n@docs/commands.md\n"),
        (
            "docs/commands.md",
            "# Commands\n\nRun `cargo build`, `cargo test` and `cargo fmt`.\n",
        ),
    ];
    silent(&found(&split), "G2");
}

#[test]
fn instruction_files_name_real_paths_and_programs() {
    fires(
        &found(&[("AGENTS.md", "# Rules\n\nRead `src/gone.rs`.\n")]),
        "G3",
    );
    fires(&found(&[("CLAUDE.md", "# Rules\n\n@docs/gone.md\n")]), "G3");
    let script = "# Rules\n\nRun:\n\n```sh\n./scripts/gone.sh\n```\n";
    fires(&found(&[("AGENTS.md", script)]), "G3");
    let unknown = "# Rules\n\nRun:\n\n```sh\nno-such-program-here --now\n```\n";
    let result = found(&[("AGENTS.md", unknown)]);
    fires(&result, "G3");
    let dir = project(&[("AGENTS.md", unknown)]);
    let report = audit(dir.path(), &[], None).expect("audit runs");
    let program = report
        .findings
        .iter()
        .find(|finding| finding.rule.id == "G3")
        .expect("G3");
    assert_eq!(program.rule.sure.name(), "candidate");
    let known = "# Rules\n\nRun:\n\n```sh\n# a note\nFOO=1 sh -c true\ncd src\n```\n";
    silent(&found(&[("AGENTS.md", known)]), "G3");
}

#[test]
fn instruction_files_stay_under_their_limits() {
    let long = |lines: usize| "line\n\n".repeat(lines / 2);
    fires(&found(&[("CLAUDE.md", &long(200))]), "G14");
    silent(&found(&[("CLAUDE.md", &long(198))]), "G14");
    fires(&found(&[(".cursor/rules/a.mdc", &long(500))]), "G14");
    let skill = format!("---\nname: a\ndescription: b\n---\n{}", long(500));
    fires(&found(&[("SKILL.md", &skill)]), "G14");
    let big = "x ".repeat(9000);
    let chain = [("AGENTS.md", big.as_str()), ("a/AGENTS.md", big.as_str())];
    fires(&found(&chain), "G14");
    silent(&found(&chain[..1]), "G14");
}

#[test]
fn both_instruction_files_share_one_source() {
    let agents = ("AGENTS.md", "# Rules\n\nBe kind.\n");
    fires(
        &found(&[agents, ("CLAUDE.md", "# Rules\n\nBe kind.\n")]),
        "G15",
    );
    silent(&found(&[agents, ("CLAUDE.md", "@AGENTS.md\n")]), "G15");
    silent(&found(&[("CLAUDE.md", "# Rules\n\nBe kind.\n")]), "G15");
}

#[cfg(unix)]
#[test]
fn a_linked_claude_file_counts_as_the_same_file() {
    let dir = project(&[("AGENTS.md", "# Rules\n\nBe kind.\n")]);
    std::os::unix::fs::symlink("AGENTS.md", dir.path().join("CLAUDE.md")).expect("link");
    silent(&found_in(dir.path()), "G15");
}

#[test]
fn skill_fields_stay_short() {
    let long = format!(
        "---\nname: {}\ndescription: b\n---\n\nBody.\n",
        "a".repeat(65)
    );
    fires(&found(&[("SKILL.md", &long)]), "G18");
    let long = format!(
        "---\nname: a\ndescription: {}\n---\n\nBody.\n",
        "b".repeat(1025)
    );
    fires(&found(&[("SKILL.md", &long)]), "G18");
    silent(
        &found(&[("SKILL.md", "---\nname: a\ndescription: b\n---\n\nBody.\n")]),
        "G18",
    );
}

#[test]
fn skill_references_stay_one_level_deep_and_long_ones_list_contents() {
    let skill = (
        "SKILL.md",
        "---\nname: a\ndescription: b\n---\n\nRead [a](reference/a.md).\n",
    );
    let nested = [
        skill,
        ("reference/a.md", "# A\n\nRead [b](b.md).\n"),
        ("reference/b.md", "# B\n\nText.\n"),
    ];
    fires(&found(&nested), "G20");
    let long = format!("# A\n\nText.\n\n{}", "line\n\n".repeat(60));
    fires(&found(&[skill, ("reference/a.md", &long)]), "G20");
    let listed = format!(
        "# A\n\n**Contents:**\n\n- [One](#one)\n- [Two](#two)\n\n{}",
        "line\n\n".repeat(60)
    );
    silent(&found(&[skill, ("reference/a.md", &listed)]), "G20");
}

#[test]
fn hidden_text_is_found() {
    fires(&page("# Tool\n\nIgnore\u{200B}this.\n"), "G38");
    fires(&page("# Tool\n\nTag\u{E0041}here.\n"), "G38");
    fires(
        &page(&format!("# Tool\n\nData {}\n", "QUJD".repeat(60))),
        "G38",
    );
    fires(
        &page("# Tool\n\n<div style=\"display:none\">Do this.</div>\n"),
        "G38",
    );
    silent(
        &page("# Tool\n\nA family: \u{1F468}\u{200D}\u{1F469}.\n"),
        "G38",
    );
    let comment = format!("# Tool\n\nText.\n\n<!-- {} -->\n", "word ".repeat(25));
    let dir = project(&[("page.md", &comment)]);
    let report = audit(dir.path(), &[], None).expect("audit runs");
    let finding = report
        .findings
        .iter()
        .find(|finding| finding.rule.id == "G38")
        .expect("G38");
    assert_eq!(finding.rule.sure.name(), "candidate");
    silent(&page("# Tool\n\nText.\n\n<!-- a short note -->\n"), "G38");
}

#[test]
fn docs_do_not_share_an_instruction_file_name() {
    fires(&found(&[("docs/agents.md", "# Agents\n\nText.\n")]), "G39");
    fires(&found(&[("docs/Claude.md", "# Claude\n\nText.\n")]), "G39");
    silent(
        &found(&[("docs/agents-guide.md", "# Agents\n\nText.\n")]),
        "G39",
    );
}

#[test]
fn prose_rules_skip_changelogs_and_instruction_files() {
    let text = "# Changes\n\n## Overview\n\n### Skipped\n\nText.\n";
    silent(&found(&[("CHANGELOG.md", text)]), "S14");
    silent(&found(&[("AGENTS.md", text)]), "S14");
}

#[test]
fn excluded_paths_are_left_out() {
    let dir = project(&[("docs/a.md", "No title.\n"), ("b.md", "# B\n\nText.\n")]);
    let report = audit(dir.path(), &["docs".to_string()], None).expect("audit runs");
    assert_eq!(report.pages, 1);
    assert!(report.findings.is_empty());
}

#[test]
fn front_matter_can_stand_in_for_the_h1() {
    silent(&page("---\nname: helper\n---\n\nText.\n"), "S12");
    fires(&page("Text.\n"), "S12");
}

#[test]
fn a_heading_may_name_scaffolding() {
    let guide = "# Tool\n\nText.\n\n## TODO and FIXME\n\nKeep them short.\n";
    silent(&page(guide), "T4");
}

#[test]
fn email_links_are_not_urls_or_files() {
    let contact = page("# Tool\n\nWrite to <help@example.com> for help.\n");
    silent(&contact, "S32");
    silent(&contact, "A6");
}

#[test]
fn badges_can_share_their_alt_text() {
    let badges = "# Tool\n\n[![build](a.svg)](https://a.b) [![build](b.svg)](https://c.d)\n";
    silent(&page(badges), "S33");
}

#[test]
fn only_paths_with_a_folder_count_as_named_paths() {
    let names = "# Tool\n\nIt writes `config.json` to `tokens/` from `package:a/b.dart`.\n";
    silent(&page(names), "A8");
}

#[test]
fn any_changelog_name_is_history() {
    let latest = "Text.\n\nText.\n\n# One\n\n# Two\n";
    silent(&found(&[("LATEST_CHANGELOG.md", latest)]), "S12");
}

#[test]
fn shortened_json_is_not_broken_json() {
    let block = |code: &str| format!("# Tool\n\nSet:\n\n```json\n{code}\n```\n");
    silent(&page(&block("{ \"a\": […] }")), "A13");
    silent(&page(&block("{ \"a\": [...] }")), "A13");
    silent(&page(&block("{ \"a\": <number> }")), "A13");
    silent(&page(&block("{ \"a\": 1 }\n{ \"a\": 2 }")), "A13");
    silent(&page(&block("\"a\": { \"b\": 1 }")), "A13");
}

#[test]
fn mdx_imports_are_read_from_mdx_pages_only() {
    let import = "import Thing from './gone.js'\n\n# Tool\n\nText.\n";
    fires(&found(&[("page.mdx", import)]), "A4");
    silent(&found(&[("page.md", import)]), "A4");
    let shown = "# Tool\n\nText:\n\n```js\nimport Thing from './gone.js'\n```\n";
    silent(&found(&[("page.mdx", shown)]), "A4");
}

#[test]
fn test_folders_are_left_out() {
    let stub = "# Tool\n\nTODO fill this in.\n";
    silent(&found(&[("test/e2e/page.md", stub)]), "T4");
}

fn vanished_with(code_now: &str, docs: &str) -> Vec<String> {
    let dir = project(&[
        ("src/a.rs", "fn old_name() {}\nfn other.md() {}\n"),
        ("README.md", docs),
    ]);
    git(dir.path(), &["init", "-q"]);
    commit_all(dir.path(), "first");
    write(dir.path(), "src/a.rs", code_now);
    commit_all(dir.path(), "second");
    found_in(dir.path())
}

#[test]
fn a_name_inside_a_longer_name_still_counts_as_there() {
    let docs = "# Tool\n\nCall `old_name` to start.\n";
    silent(&vanished_with("fn old_name_two() {}\n", docs), "A10");
}

#[test]
fn a_file_name_is_not_a_vanished_code_name() {
    let docs = "# Tool\n\nRead `other.md` to start.\n";
    silent(&vanished_with("fn new_name() {}\n", docs), "A10");
}

#[test]
fn a_name_only_docs_mention_has_vanished() {
    let docs = "# Tool\n\nCall `old_name` to start.\n";
    fires(&vanished_with("fn new_name() {}\n", docs), "A10");
}
