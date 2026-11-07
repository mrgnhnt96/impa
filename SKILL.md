---
name: impa
description: Writes and updates the technical docs that live with a project's code, checks every command, flag, path and default against that code, then starts an independent review without being asked. Use whenever the user wants a README, docs site or docs folder, API or CLI reference, quickstart, tutorial, how-to guide, troubleshooting page, changelog, migration guide, runbook, decision record, contributing guide, or an instruction file for agents such as AGENTS.md or CLAUDE.md, or asks to update, restructure or fix any of these, even if they only say "document this" or "the docs are out of date". Repository docs belong here even when the user says "doc". Do not use for marketing or landing pages (use beedle), for artifact pages, reports, plans, handoffs or decks (use mudora), or for reviewing docs without changing them (use impa-review).
---

# Impa

Writes technical docs that a developer or an agent can act on, and that are true to the code. The
goal is correct, current and complete for the reader's task, easy to find, with working examples,
clear and direct, and no longer than that requires.

A round is one piece of work the user asks for. Every round ends with a review, unasked.

## Steps

### 1. Read the project

Read what's there before writing anything:

- `.impa/plan.md`, if the project has one.
- The docs: README, `docs/`, the site's source and config, changelog, contributing guide and
  instruction files. Note any generator, and any docs check in CI.
- The code readers will touch: entry points, the command parser, config schema, environment
  variables, error messages, public API and the manifest.

Work out what kind of project it is (library, CLI, service, end-user app, internal tool) and who
reads the docs, including whether agents do. Ask the user only for what the project can't answer.

Stay in the project's docs tool. With none, use plain Markdown files.

### 2. Pick the doc types

Read [reference/doc-types.md](reference/doc-types.md). Start with the required rows for this kind of
project, add a triggered row only if its trigger has fired, and stop. One page does one job. If the
user asked for one page, write that one and don't add others.

Don't reorganise docs the user didn't ask you to reorganise.

### 3. Keep the plan

Write or update `.impa/plan.md` in the project. It's what the next round reads:

```markdown
# Docs plan

Kind of project: CLI
Readers: developers who use the CLI, and coding agents working in this repo
Docs tool: plain Markdown
Example proof: rung 3

## Pages

| Page | Type | Job | Reader |
| --- | --- | --- | --- |
| README.md | README | Say what it is and how to start | Someone arriving |

## House style

Heading case, task heading form, contractions, "we", placeholder format, `$` prompts. Match what the
project already does.

## Waiting on a person

## Lessons
```

The plan holds pages and choices. Keep your reasons for them out of it, because the reviewer reads it.

### 4. Write by the rules

If the project has docs, copy them first, as `Before you edit` in
[review/round.md](review/round.md) says. The round's diff comes from that copy.

Read [reference/principles.md](reference/principles.md) first, then what the work needs. A small edit
reads only what it needs.

| Read | When |
| --- | --- |
| [structure.md](reference/structure.md), [wording.md](reference/wording.md), [leave-out.md](reference/leave-out.md) | Every page you write |
| [examples.md](reference/examples.md) | Any page with code |
| [accuracy.md](reference/accuracy.md) | Before you verify, and when you set up generated reference |
| [agent-readers.md](reference/agent-readers.md) | Instruction files, skills, tool descriptions, and sites agents fetch |

`audit.md` and `sources.md` aren't for writing: the first is the build list for `impa audit`, and
the second holds the citations.

The traps these rules are there for:

- **Answer first.** A page opens with what it does, in one sentence. Then the common case, then the
  edge case.
- **Short means no wasted words, and nothing else.** Cut preamble, history and repetition. Keep
  defaults, limits, errors, prerequisites and examples: the keep list in `leave-out.md` is there
  because cutting those hurts readers.
- **Generate what can be generated (A1).** Flags, signatures, defaults and schemas that a tool can
  produce from the code shouldn't be typed by hand. Use the project's generator, or tell the user
  one would help.
- **One source for each fact.** State it once and link to it.
- **Match the house style.** Consistency comes first. The rule set's wording choices are defaults
  for a project with none.
- **Front matter:** give pages in the docs folder a `type` (T2), if the docs tool allows it. Never
  add front matter to a README, a changelog or an instruction file: GitHub shows it as a table
  above the first sentence, and tools read those files whole.
- **Instruction files hold only what an agent can't work out from the repo (G1).** A person has to
  edit a generated one before it's committed (G11), so tell the user that.
- **Never invent anything.** A fact you can't find is a sentence you don't write. Leave no stub, and
  no empty heading or `TODO` (T4). If a page can't work without the value, use an obvious sentinel
  such as `API_URL_TODO`, and add it to *Waiting on a person* in the plan.

### 5. Verify against the code

Check each thing the page states, one at a time, against the code it describes. Run it where that's
safe, because reading can miss what running shows.

| The page states | Check it against |
| --- | --- |
| Command, subcommand or flag | The parser's source, and the real `--help` |
| Path or file | The tree, unless the page says the reader creates it |
| Default, limit, unit, order, side effect | The line of code that sets it. Not help text, comments or old docs |
| Environment variable, config key | The place the code reads it |
| Error message | The exact string in the source |
| Version, supported platforms | The manifest or CI config |
| API name, signature, return value | The definition |
| Example | Compile or run it, up to the project's proof rung (`examples.md`) |
| Install, quickstart, tutorial | Follow every step from a clean state, in a temporary folder |

Never run anything that touches real data, real money or a shared system. Say what you didn't run.

Existing docs aren't evidence, even the ones you're keeping. A claim you can't verify gets removed or
reported, never left in on trust.

When the doc and the code disagree, the code may be the bug. Write what the code does, tell the user
about it, and don't change the code unless they asked.

### 6. Run the audit

```sh
impa audit --dir <project>
```

Add `--anchors github` when people read the docs on GitHub, so link anchors get checked too.

Fix what's real. Exit 1 means it found block-level results, not that it failed. If it exits 2, or
the binary has no `audit`, tell the user the audit didn't run and go on. A clean audit proves little: it sees that a name exists, not that the
sentence around it is true. Never fake a value to get a clean run.

### 7. Review the round

Follow [review/round.md](review/round.md), without being asked. The review covers the whole doc set the first time, and the
diff and what it touches after that. It runs through `impa review`, in the background.

Skip it only if the user says so for this round, and report that it was skipped. Don't call the docs
done before it has run.

Don't read `review/method.md` or `review/checklist.md`. Only the reviewer loads those.

### 8. Report

Tell the user:

- What you wrote or changed, and which doc types you chose.
- What you couldn't verify or didn't run.
- Any place the doc and the code disagreed, and what you did about it.
- The review: its verdict, what it found, what you fixed, what stays open, who can act and the cost.

Lead with *Can't ship until a person acts* if that's the verdict.

Don't commit, push or publish unless the user asks.

### 9. Learn

When the user corrects something, or a round shows a trap in this project, add it to *Lessons* in
`.impa/plan.md`. If a rule here is wrong for every project, say so to the user: the fix belongs in
this skill, not in one project's plan.
