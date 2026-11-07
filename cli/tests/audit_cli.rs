use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const BIN: &str = env!("CARGO_BIN_EXE_impa");
const CLEAN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/audit/clean");
const BROKEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../tests/fixtures/audit/broken"
);

fn audit(args: &[&str]) -> Output {
    Command::new(BIN)
        .arg("audit")
        .args(args)
        .output()
        .expect("run impa")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn json(args: &[&str]) -> Value {
    let output = audit(&[args, &["--json"]].concat());
    serde_json::from_slice(&output.stdout).expect("json output")
}

fn has(result: &Value, rule: &str, file: &str) -> bool {
    let findings = result["findings"].as_array().expect("findings");
    findings
        .iter()
        .any(|finding| finding["rule"] == rule && finding["file"] == file)
}

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    let mut folders = vec![root.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(folder).expect("read folder").flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else {
                files.push((path.clone(), fs::read(path).expect("read file")));
            }
        }
    }
    files.sort();
    files
}

#[test]
fn clean_project_exits_zero_and_reports_nothing() {
    let output = audit(&["--dir", CLEAN, "--anchors", "github"]);
    assert_eq!(output.status.code(), Some(0), "{}", stdout(&output));
    assert!(stdout(&output).contains("0 blocks, 0 warnings"));
}

#[test]
fn block_findings_exit_one() {
    assert_eq!(audit(&["--dir", BROKEN]).status.code(), Some(1));
}

#[test]
fn missing_or_unusable_dir_exits_two() {
    let no_dir = audit(&[]);
    assert_eq!(no_dir.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&no_dir.stderr).contains("usage: impa audit --dir"));
    assert_eq!(audit(&["--dir", "/no/such/folder"]).status.code(), Some(2));
}

#[test]
fn write_flags_do_not_exist() {
    for flag in ["--fix", "--out"] {
        assert_eq!(audit(&["--dir", BROKEN, flag]).status.code(), Some(2));
    }
}

#[test]
fn broken_project_reports_each_kind_of_rule() {
    let result = json(&["--dir", BROKEN, "--anchors", "github"]);
    for (rule, file) in [
        ("S2", "README.md"),
        ("S12", "README.md"),
        ("S14", "README.md"),
        ("S15", "README.md"),
        ("S20", "README.md"),
        ("S32", "README.md"),
        ("S34", "README.md"),
        ("T4", "README.md"),
        ("E9", "README.md"),
        ("A6", "README.md"),
        ("A13", "README.md"),
        ("A7", "docs/guide.md"),
        ("A8", "docs/guide.md"),
        ("E1", "docs/guide.md"),
        ("S6", "docs/guide.md"),
        ("S30", "docs/guide.md"),
        ("T2", "docs/untyped.md"),
        ("T2", "docs/kind.md"),
        ("G2", "AGENTS.md"),
        ("G3", "AGENTS.md"),
        ("G15", "CLAUDE.md"),
    ] {
        assert!(has(&result, rule, file), "no {rule} on {file}: {result:#}");
    }
}

#[test]
fn json_says_how_sure_each_finding_is() {
    let result = json(&["--dir", BROKEN]);
    let findings = result["findings"].as_array().expect("findings");
    let certainty = |rule: &str| {
        let found = findings.iter().find(|finding| finding["rule"] == rule);
        found.expect("finding")["certainty"].clone()
    };
    assert_eq!(certainty("A6"), "certain");
    assert_eq!(certainty("A8"), "candidate");
    assert!(result["blocks"].as_u64().expect("blocks") > 0);
    assert!(result["warnings"].as_u64().expect("warnings") > 0);
    assert!(result["pages"].as_u64().expect("pages") > 0);
}

#[test]
fn text_output_puts_blocks_first_and_names_what_did_not_run() {
    let text = stdout(&audit(&["--dir", BROKEN]));
    let first_warning = text.find("\nwarn").expect("a warning");
    assert!(text.rfind("\nblock").expect("a block") < first_warning);
    assert!(text.contains("README.md:"));
    assert!(text.contains("Rules run:"));
    assert!(text.contains("Not run: A7"));
}

#[test]
fn exclude_leaves_a_path_out() {
    let result = json(&["--dir", BROKEN, "--exclude", "docs"]);
    assert!(!has(&result, "T2", "docs/kind.md"));
    assert!(has(&result, "E9", "README.md"));
}

#[test]
fn audit_never_writes() {
    let root = Path::new(BROKEN);
    let before = snapshot(root);
    audit(&["--dir", BROKEN, "--anchors", "github", "--json"]);
    assert_eq!(snapshot(root), before);
}
