use regex::Regex;
use serde_json::Value;

const VERDICT: &str =
    r"(?m)^\W*Verdict:\W*(Ready to ship|Fix first|Can['’]t ship until a person acts)";
const FINDING_HEADING: &str = r"(?m)^#+\s*\**([A-Z]\d+)\s*·\s*(?:Blocker|Major|Minor|Note)\b";
const HANDED_OVER: &str = r"(?m)^Handed to a person:([\s\S]*?)(?:\n\s*\n|\z)";
const ITEM_ID: &str = r"\b([A-Z]\d+)\s*·";

#[derive(Debug, Default)]
pub struct Summary {
    pub ok: bool,
    pub report: String,
    pub turns: u64,
    pub seconds: u64,
    pub cost_usd: f64,
    pub error: Option<String>,
}

fn pattern(text: &str) -> Regex {
    Regex::new(text).unwrap_or_else(|error| panic!("bad pattern {text}: {error}"))
}

fn assistant_text(event: &Value) -> String {
    let content = event["message"]["content"].as_array();
    content
        .into_iter()
        .flatten()
        .filter(|item| item["type"] == "text")
        .filter_map(|item| item["text"].as_str())
        .collect()
}

pub fn summarise(lines: &[String]) -> Summary {
    let mut out = Summary::default();
    let mut last_text = String::new();
    for line in lines {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if event["type"] == "assistant" {
            let text = assistant_text(&event);
            if !text.trim().is_empty() {
                last_text = text;
            }
        }
        if event["type"] == "result" {
            out.ok = event["subtype"] == "success" && event["is_error"] != true;
            out.report = match event["result"].as_str() {
                Some(result) if !result.trim().is_empty() => result.to_string(),
                _ => last_text.clone(),
            };
            out.turns = event["num_turns"].as_u64().unwrap_or(0);
            let milliseconds = event["duration_ms"].as_f64().unwrap_or(0.0);
            out.seconds = (milliseconds / 1000.0).round() as u64;
            out.cost_usd = event["total_cost_usd"].as_f64().unwrap_or(0.0);
            if !out.ok {
                let subtype = event["subtype"].as_str().unwrap_or("error");
                out.error = Some(subtype.to_string());
            }
        }
    }
    if out.report.is_empty() {
        out.report = last_text;
    }
    out
}

pub fn handed_over(brief: &str) -> Vec<String> {
    let handed = pattern(HANDED_OVER);
    let section = handed
        .captures(brief)
        .and_then(|caps| caps.get(1))
        .map_or("", |found| found.as_str());
    pattern(ITEM_ID)
        .captures_iter(section)
        .map(|caps| caps[1].to_string())
        .collect()
}

#[derive(Debug)]
pub struct ReportRead {
    pub verdict: Option<String>,
    pub repeated: Vec<String>,
}

pub fn read_report(report: &str, brief: &str) -> ReportRead {
    let handed = handed_over(brief);
    let mut repeated = Vec::new();
    for caps in pattern(FINDING_HEADING).captures_iter(report) {
        let id = caps[1].to_string();
        if handed.contains(&id) && !repeated.contains(&id) {
            repeated.push(id);
        }
    }
    let verdict = pattern(VERDICT)
        .captures(report)
        .map(|caps| caps[1].replacen('’', "'", 1));
    ReportRead { verdict, repeated }
}
