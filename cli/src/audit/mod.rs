mod agents;
mod examples;
mod markdown;
mod names;
mod project;
mod rules;
mod structure;
#[cfg(test)]
mod tests;

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Args as ClapArgs, ValueEnum};
use serde_json::{Value, json};

use project::Project;
use rules::{A7, A10, Level, RULE_IDS, Rule};

const USAGE: &str =
    "usage: impa audit --dir <project> [--exclude <path>]... [--anchors github] [--json]";

#[derive(ClapArgs)]
#[command(args_override_self = true)]
pub struct Args {
    /// The project to check
    #[arg(long, allow_hyphen_values = true)]
    dir: Option<PathBuf>,
    /// A path to leave out, relative to the project. Repeat for more
    #[arg(long, allow_hyphen_values = true)]
    exclude: Vec<String>,
    /// How the docs turn headings into anchors. Anchors aren't checked without it
    #[arg(long, value_enum)]
    anchors: Option<Renderer>,
    /// Print the full result as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Renderer {
    Github,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    pub rule: Rule,
    pub file: String,
    pub line: usize,
    pub message: String,
}

#[derive(Default)]
pub struct Findings(Vec<Finding>);

impl Findings {
    pub fn add(&mut self, rule: Rule, file: &str, line: usize, message: impl Into<String>) {
        let finding = Finding {
            rule,
            file: file.to_string(),
            line,
            message: message.into(),
        };
        if !self.0.contains(&finding) {
            self.0.push(finding);
        }
    }
}

pub struct Report {
    pages: usize,
    findings: Vec<Finding>,
    not_run: Vec<String>,
}

impl Report {
    fn count(&self, level: Level) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.rule.level == level)
            .count()
    }

    fn to_json(&self) -> Value {
        let findings: Vec<Value> = self
            .findings
            .iter()
            .map(|finding| {
                json!({
                    "level": finding.rule.level.name(),
                    "certainty": finding.rule.sure.name(),
                    "rule": finding.rule.id,
                    "file": finding.file,
                    "line": finding.line,
                    "message": finding.message,
                })
            })
            .collect();
        json!({
            "pages": self.pages,
            "blocks": self.count(Level::Block),
            "warnings": self.count(Level::Warn),
            "findings": findings,
            "rulesRun": RULE_IDS,
            "notRun": self.not_run,
        })
    }

    fn to_text(&self) -> String {
        let mut text = String::new();
        for finding in &self.findings {
            let Finding {
                rule,
                file,
                line,
                message,
            } = finding;
            let (level, sure, id) = (rule.level.name(), rule.sure.name(), rule.id);
            text.push_str(&format!(
                "{level:<5}  {sure:<9}  {id:<3}  {file}:{line}  {message}\n"
            ));
        }
        if !text.is_empty() {
            text.push('\n');
        }
        let (pages, blocks, warnings) = (
            self.pages,
            self.count(Level::Block),
            self.count(Level::Warn),
        );
        text.push_str(&format!(
            "{pages} pages, {blocks} blocks, {warnings} warnings\n"
        ));
        text.push_str(&format!("Rules run: {}\n", RULE_IDS.join(" ")));
        for reason in &self.not_run {
            text.push_str(&format!("Not run: {reason}\n"));
        }
        text
    }
}

pub fn audit(dir: &Path, exclude: &[String], anchors: Option<Renderer>) -> Result<Report, String> {
    let project = Project::load(dir)?;
    let pages = project.read_pages(exclude)?;
    let mut out = Findings::default();
    let mut not_run = Vec::new();
    for page in &pages {
        structure::check(page, &mut out);
        examples::check(page, &mut out);
        agents::check_hidden_text(page, &mut out);
        names::check_links(&project, page, &mut out);
        names::check_includes(&project, page, &mut out);
        names::check_paths(&project, page, &mut out);
    }
    structure::check_project(&pages, &mut out);
    agents::check_project(&project, &pages, &mut out);
    match anchors {
        Some(Renderer::Github) => names::check_anchors(&pages, &mut out),
        None => not_run.push(format!(
            "{}: anchors, because no --anchors renderer is set",
            A7.id
        )),
    }
    if project.has_history() {
        names::check_vanished(&project, &pages, &mut out);
    } else {
        not_run.push(format!(
            "{}: vanished names, because there's no git history",
            A10.id
        ));
    }
    let mut findings = out.0;
    findings.sort_by(|one, other| {
        let key = |finding: &Finding| (finding.rule.level, finding.file.clone(), finding.line);
        key(one).cmp(&key(other))
    });
    Ok(Report {
        pages: pages.len(),
        findings,
        not_run,
    })
}

pub fn run(args: Args) -> u8 {
    let Some(dir) = args.dir.filter(|dir| dir.is_dir()) else {
        eprintln!("{USAGE}");
        return 2;
    };
    let report = match audit(&dir, &args.exclude, args.anchors) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return 2;
        }
    };
    let output = if args.json {
        serde_json::to_string_pretty(&report.to_json()).unwrap_or_default() + "\n"
    } else {
        report.to_text()
    };
    let _ = io::stdout().write_all(output.as_bytes());
    u8::from(report.count(Level::Block) > 0)
}
