# Installing impa

impa is a pair of Claude Code skills. `impa` writes and updates technical docs, and `impa-review`
reviews them against the code.

## Steps for the agent

1. Put the repo in the user's Claude skills folder, so the path is exactly
   `~/.claude/skills/impa/SKILL.md`, and link the review skill next to it:
   ```sh
   mkdir -p ~/.claude/skills
   git clone https://github.com/mrgnhnt96/impa ~/.claude/skills/impa
   ln -s ~/.claude/skills/impa/review ~/.claude/skills/impa-review
   ```
   `impa review` reads its rule files from `~/.claude/skills/impa/`, so use that path. If you keep
   the clone somewhere else, symlink it to that path. The skills rely on it: `--skill-dir` works
   only when you run `impa review` yourself.
2. Put the `impa` binary on the PATH. This needs `cargo`:
   ```sh
   cargo install --path ~/.claude/skills/impa/cli
   ```
   From the repo, that's `cargo install --path cli`. It puts `impa` in `~/.cargo/bin`, so add that
   folder to the PATH if `command -v impa` finds nothing.
3. Start a new Claude Code session. Check that `impa` and `impa-review` are in the skills list.

Without the binary, the skills fall back to a general-purpose agent for the review. That agent isn't
locked to read-only tools, and its cost isn't recorded.

## What it needs

| Needs | Why |
| --- | --- |
| Claude Code | Both skills run in it |
| Rust 1.85 or later (`cargo`) | Building the `impa` CLI in `cli/`, which uses the 2024 edition |
| `git`, for `impa audit` | Listing a project's files and comparing docs with its history. Without it the audit walks the folder and skips the history check |
| A Unix system such as macOS or Linux | The CLI uses Unix process calls, so it doesn't build on Windows |
| The `claude` CLI on the PATH, with `--restricted` | `impa review` runs each review as a locked headless session. Set `IMPA_CLAUDE` to use another program |

## Updating

```sh
git -C ~/.claude/skills/impa pull
cargo install --path ~/.claude/skills/impa/cli
```
