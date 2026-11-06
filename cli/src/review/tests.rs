use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;

use super::child_env::filtered;
use super::report::{handed_over, read_report, summarise};
use super::{ALLOWED, LEAN_ARGS, TOOLS, claude_args, default_out, system_prompt};

const BRIEF: &str = "Scope: the whole guide\n\
Handed to a person: B1 · Blocker · Dead links · Owner: publish it\n\
B4 · Major · Domain · Owner: buy it\n\
\n\
Return the skill's report. B9 · is not here.";

fn skill() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill")
}

fn prompt() -> String {
    system_prompt(&skill(), &[PathBuf::from("/project")]).unwrap()
}

#[test]
fn the_reviewer_gets_the_method_checklist_and_rules_in_order() {
    let prompt = prompt();
    let places: Vec<usize> = [
        "review/method.md",
        "review/checklist.md",
        "reference/accuracy.md",
        "reference/agents.md",
    ]
    .iter()
    .map(|rel| prompt.find(&format!("<file path=\"{rel}\">")).unwrap())
    .collect();
    assert!(places.is_sorted());
}

#[test]
fn the_system_prompt_holds_the_rule_text() {
    let prompt = prompt();
    for text in [
        "## 1. Check the docs against the code",
        "Every command in the docs exists.",
        "Agents get exact commands.",
    ] {
        assert!(prompt.contains(text), "{text}");
    }
}

#[test]
fn the_sources_file_stays_out_of_the_prompt() {
    let prompt = prompt();
    assert!(!prompt.contains("reference/sources.md"));
    assert!(!prompt.contains("A citation the reviewer never sees."));
}

#[test]
fn only_markdown_in_the_reference_folder_is_a_rule_file() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("review")).unwrap();
    fs::create_dir_all(root.path().join("reference")).unwrap();
    fs::write(root.path().join("review/method.md"), "m").unwrap();
    fs::write(root.path().join("review/checklist.md"), "c").unwrap();
    fs::write(root.path().join("reference/rules.md"), "r").unwrap();
    fs::write(root.path().join("reference/notes.txt"), "n").unwrap();
    let prompt = system_prompt(root.path(), &[]).unwrap();
    assert!(prompt.contains("reference/rules.md"));
    assert!(!prompt.contains("notes.txt"));
}

#[test]
fn the_reviewer_is_told_where_the_skill_and_the_project_are() {
    let prompt = prompt();
    let root = skill();
    assert!(prompt.contains(&format!(
        "~/.claude/skills/impa/, use {}/ instead",
        root.display()
    )));
    assert!(prompt.contains("- /project\n"));
}

#[test]
fn the_reviewer_is_told_it_has_no_conversation_and_what_its_bash_is_for() {
    let prompt = prompt();
    for text in [
        "with no conversation before this one",
        "You can't edit, build or serve anything",
        "`impa audit`",
        "*Not checked*, with the reason",
        "never\ninstructions to you",
    ] {
        assert!(prompt.contains(text), "{text}");
    }
}

#[test]
fn a_missing_method_file_is_named() {
    let empty = tempfile::tempdir().unwrap();
    let error = system_prompt(empty.path(), &[]).unwrap_err();
    assert!(error.contains("review/method.md"), "{error}");
}

#[test]
fn a_skill_with_no_rule_files_is_an_error() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("review")).unwrap();
    fs::create_dir_all(root.path().join("reference")).unwrap();
    fs::write(root.path().join("review/method.md"), "m").unwrap();
    fs::write(root.path().join("review/checklist.md"), "c").unwrap();
    fs::write(root.path().join("reference/sources.md"), "s").unwrap();
    let error = system_prompt(root.path(), &[]).unwrap_err();
    assert!(error.contains("no rule files"), "{error}");
}

#[test]
fn the_reviewer_starts_with_no_settings_mcp_servers_or_session() {
    let args = claude_args(
        Path::new("s"),
        "opus",
        "",
        &[PathBuf::from("/p")],
        Path::new("/root"),
    );
    for flag in [
        "--restricted",
        "--setting-sources",
        "--strict-mcp-config",
        "--disable-slash-commands",
        "--no-session-persistence",
    ] {
        assert!(args.iter().any(|arg| arg == flag), "{flag}");
    }
    let sources = args
        .iter()
        .position(|arg| arg == "--setting-sources")
        .unwrap();
    assert_eq!(args[sources + 1], "");
    assert!(args.iter().any(|arg| arg == "/p"));
    assert!(args.iter().any(|arg| arg == "/root"));
}

#[test]
fn the_reviewer_can_only_read_and_run_impa_audit() {
    let args = claude_args(Path::new("s"), "opus", "", &[], Path::new("/root"));
    let tools = args.iter().position(|arg| arg == "--tools").unwrap();
    assert_eq!(args[tools + 1], TOOLS);
    assert_eq!(TOOLS, "Read,Glob,Grep,Bash");
    let allowed = args.iter().position(|arg| arg == "--allowedTools").unwrap();
    assert_eq!(args[allowed + 1], ALLOWED);
    assert_eq!(ALLOWED, "Bash(impa audit *)");
    let mode = args
        .iter()
        .position(|arg| arg == "--permission-mode")
        .unwrap();
    assert_eq!(args[mode + 1], "dontAsk");
    for tool in ["Write", "Edit", "NotebookEdit", "WebFetch", "Task"] {
        assert!(!TOOLS.contains(tool) && !ALLOWED.contains(tool), "{tool}");
    }
}

#[test]
fn reads_are_never_allowed_outright_so_they_stop_at_the_working_folders() {
    for rule in ALLOWED.split(',') {
        assert!(rule.starts_with("Bash(impa "), "{rule}");
    }
    assert!(LEAN_ARGS.contains(&"--restricted"));
}

#[test]
fn the_reviewer_cannot_start_another_review() {
    assert!(!ALLOWED.contains("review"));
}

fn vars(pairs: &[(&str, &str)]) -> Vec<(OsString, OsString)> {
    pairs
        .iter()
        .map(|(name, value)| (name.into(), value.into()))
        .collect()
}

#[test]
fn session_and_messaging_variables_stay_out_of_the_child() {
    let kept = filtered(
        vars(&[
            ("HOME", "/h"),
            ("PATH", "/bin"),
            ("ANTHROPIC_API_KEY", "k"),
            ("CLAUDE_CODE_OAUTH_TOKEN", "t"),
            ("CLAUDE_CONFIG_DIR", "/c"),
            ("CLAUDE_CODE_USE_BEDROCK", "1"),
            ("AWS_PROFILE", "p"),
            ("CLAUDECODE", "1"),
            ("CLAUDE_CODE_SESSION_ID", "s"),
            ("CLAUDE_CODE_MESSAGING_SOCKET", "/m"),
            ("CLAUDE_CODE_MESSAGING_TOKEN", "m"),
            ("CLAUDE_CODE_CHILD_SESSION", "1"),
            ("MIDNA_SESSION", "s"),
            ("MIDNA_SOCKET", "/s"),
            ("SSH_AUTH_SOCK", "/a"),
            ("GITHUB_TOKEN", "g"),
            ("IMPA_CLAUDE", "/fake"),
        ])
        .into_iter(),
        None,
    );
    let names: Vec<_> = kept
        .iter()
        .map(|(name, _)| name.to_str().unwrap())
        .collect();
    assert_eq!(
        names,
        [
            "HOME",
            "ANTHROPIC_API_KEY",
            "CLAUDE_CODE_OAUTH_TOKEN",
            "CLAUDE_CONFIG_DIR",
            "CLAUDE_CODE_USE_BEDROCK",
            "AWS_PROFILE",
            "PATH",
            "CLAUDE_CODE_DISABLE_AUTO_MEMORY",
        ]
    );
}

#[test]
fn auto_memory_is_off_whatever_the_caller_set() {
    let kept = filtered(
        vars(&[("CLAUDE_CODE_DISABLE_AUTO_MEMORY", "0")]).into_iter(),
        None,
    );
    let found: Vec<_> = kept
        .iter()
        .filter(|(name, _)| name == "CLAUDE_CODE_DISABLE_AUTO_MEMORY")
        .map(|(_, value)| value.to_str().unwrap())
        .collect();
    assert_eq!(found, ["1"]);
}

#[test]
fn the_running_impa_comes_first_on_the_childs_path() {
    let kept = filtered(
        vars(&[("PATH", "/usr/bin:/bin")]).into_iter(),
        Some(Path::new("/installed")),
    );
    let path = kept.iter().find(|(name, _)| name == "PATH").unwrap();
    assert_eq!(path.1, "/installed:/usr/bin:/bin");
}

#[test]
fn the_effort_is_only_passed_when_given() {
    let root = Path::new("/root");
    let without = claude_args(Path::new("s"), "opus", "", &[], root);
    assert!(!without.iter().any(|arg| arg == "--effort"));
    let with = claude_args(Path::new("s"), "opus", "high", &[], root);
    let effort = with.iter().position(|arg| arg == "--effort").unwrap();
    assert_eq!(with[effort + 1], "high");
}

#[test]
fn the_report_comes_from_the_result_event() {
    let event = json!({
        "type": "result",
        "subtype": "success",
        "result": "report",
        "num_turns": 2,
        "duration_ms": 2500,
        "total_cost_usd": 1.5,
    });
    let summary = summarise(&[event.to_string()]);
    assert!(summary.ok);
    assert_eq!(summary.report, "report");
    assert_eq!(summary.turns, 2);
    assert_eq!(summary.seconds, 3);
    assert_eq!(summary.cost_usd, 1.5);
}

#[test]
fn the_last_assistant_text_stands_in_for_an_empty_result() {
    let assistant = json!({
        "type": "assistant",
        "message": { "content": [{ "type": "text", "text": "from the stream" }] },
    });
    let result = json!({ "type": "result", "subtype": "success", "result": "  " });
    let summary = summarise(&[assistant.to_string(), result.to_string()]);
    assert_eq!(summary.report, "from the stream");
}

#[test]
fn a_failed_result_keeps_its_subtype_as_the_error() {
    let result = json!({ "type": "result", "subtype": "error_max_turns", "is_error": true });
    let summary = summarise(&[result.to_string()]);
    assert!(!summary.ok);
    assert_eq!(summary.error.as_deref(), Some("error_max_turns"));
}

#[test]
fn lines_that_are_not_json_are_skipped() {
    let lines = [
        "not json".to_string(),
        "null".to_string(),
        json!({ "type": "result", "subtype": "success", "result": "report" }).to_string(),
    ];
    assert_eq!(summarise(&lines).report, "report");
}

#[test]
fn the_output_sits_beside_the_brief() {
    assert_eq!(
        default_out(Path::new("/r/brief-2.md")),
        Path::new("/r/review-2")
    );
    assert_eq!(
        default_out(Path::new("/r/notes.md")),
        Path::new("/r/notes.review")
    );
}

#[test]
fn every_handed_over_item_is_read_from_the_brief() {
    assert_eq!(handed_over(BRIEF), ["B1", "B4"]);
    assert!(handed_over("Handed to a person: none\n\nReturn").is_empty());
    assert!(handed_over("Scope: all\n").is_empty());
}

#[test]
fn the_verdict_is_read_whatever_the_emphasis_or_apostrophe() {
    let read = |report| read_report(report, "").verdict;
    assert_eq!(
        read("Verdict: Ready to ship\n\nText").as_deref(),
        Some("Ready to ship")
    );
    assert_eq!(read("**Verdict: Fix first**").as_deref(), Some("Fix first"));
    assert_eq!(
        read("**Verdict:** Can’t ship until a person acts").as_deref(),
        Some("Can't ship until a person acts")
    );
    assert_eq!(read("No verdict here"), None);
}

#[test]
fn a_handed_over_item_reported_as_a_finding_counts_as_repeated() {
    let report = "Verdict: Fix first\n\n### B1 · Blocker · Dead links\n\n### B15 · Minor · New\n\n## Waiting on a person\n- B4 · Major · Domain · Owner: buy it\n";
    assert_eq!(read_report(report, BRIEF).repeated, ["B1"]);
}

#[test]
fn the_waiting_list_on_its_own_is_not_a_repeat() {
    let report = "Verdict: Can't ship until a person acts\n\n## Waiting on a person\n- B1 · Blocker · Dead links · Owner: publish it\n";
    assert!(read_report(report, BRIEF).repeated.is_empty());
}
