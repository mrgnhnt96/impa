# Impa

Claude Code skills for technical docs that developers and AI agents can use: a writer that keeps docs short, direct and true to the code, and a reviewer that checks them in its own headless session.

The name comes from Impa in The Legend of Zelda, the keeper of lore who tells you plainly what to do next. This is an unofficial fan reference, with no link to Nintendo.

## What it does

- **Writes the docs a project needs.** It reads the project, picks the doc types that kind of project requires and stops there. One page does one job.
- **Follows researched rules.** The rules in `reference/` come with their sources, and say which ones rest on evidence and which on opinion.
- **Checks each fact against the code.** Every command, flag, path and default is compared with the code that defines it, and run where that's safe.
- **Never invents a fact.** A value it can't find is a sentence it doesn't write, and what only a person can supply is listed as waiting on them.
- **Reviews every round.** After each change, a reviewer in its own session compares the docs with the code without being asked: the whole set the first time, and the diff and what it touches after that.
- **Reviews any docs on its own.** The reviewer needs no writer. It returns findings only, says who can act on each one, and never fixes anything.

## Install

Follow [INSTALL.md](INSTALL.md), or ask Claude Code to. The skills call the `impa` binary, so it has to be on your PATH. That needs Rust's `cargo`. From the repo:

```sh
cargo install --path cli
```

## Review docs without the writer

In Claude Code, ask for a review: "check whether the README matches the code". The `impa-review` skill starts it and returns the report.

From a terminal, write a brief and run the command. One line is enough:

```sh
mkdir -p /tmp/impa-review
echo "Review the docs in $PWD." > /tmp/impa-review/brief-1.md
impa review --brief /tmp/impa-review/brief-1.md --add-dir "$PWD"
```

It prints one line with the verdict, turns, seconds and cost, and writes `review-1.md` and `review-1.json` beside the brief. The verdict is `Ready to ship`, `Fix first` or `Can't ship until a person acts`. It exits 1 if the review fails.

| Flag | What it does | Default |
| --- | --- | --- |
| `--brief <file>` | The brief, sent as the reviewer's prompt. The review runs in its folder | Required |
| `--add-dir <dir>` | A folder the reviewer may read. Repeat it for each one | Required |
| `--out <base>` | Where to write the report, without an extension | Beside the brief |
| `--model <model>` | The model that reviews | `opus` |
| `--effort <level>` | The reasoning effort | The model's own |
| `--timeout <minutes>` | How long the review may run | `20` |
| `--skill-dir <dir>` | Where the rule files are | `~/.claude/skills/impa` |

The reviewer can read only the brief's folder, the skill and the `--add-dir` folders. It can't edit files, and its shell can run only `impa audit`. That subcommand isn't in the binary yet, so the report lists the audit under *Not checked*, with anything else it couldn't check.

## Layout

- `SKILL.md`: the `impa` skill, which writes and updates docs
- `review/SKILL.md`: the `impa-review` skill, which starts a review and returns its findings
- `review/method.md`, `review/checklist.md`: how a reviewer works and every check. Only reviewers load these
- `review/round.md`: how each round is reviewed
- `reference/`: the rules for writing and reviewing docs, with `sources.md` holding the citations
- `cli/`: the Rust binary, with `impa review` to start a locked reviewer

## Tests

```sh
cargo test --manifest-path cli/Cargo.toml
```

## License

[MIT](LICENSE)
