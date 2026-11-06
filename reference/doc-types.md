# Doc types

Which documents a project needs and what each one is for. No study has tested which doc types a
project needs, and none has measured a benefit from Diátaxis. The matrix below is judgment drawn
from what each source requires. The strong evidence is about failure: install steps and tutorials
that don't work, and docs that don't match the code, are what hurt readers most.

## What the type frameworks agree on

Diátaxis names four types: tutorial, how-to guide, reference and explanation. Its claim that there
are "four and only four" is contested. Cloudflare uses 14 types, the Good Docs Project about 25,
Aghajani's survey 13 and Google's engineering book 5, and none of them tested the others. What they
share is the useful part: a reader wants one thing at a time, and a page that mixes jobs serves
neither.

Use the four types as a check on each page, not as the site's folder layout. Ask which job this
page does, and move what doesn't belong to it.

| Type | Reader is | Page gives | Must have | Leave out |
| --- | --- | --- | --- | --- |
| Tutorial | Learning, new to it | One guided path to a working result | Prerequisites, every step, result at each step | Choices, full option lists |
| Quickstart | Evaluating, wants it running | Shortest path to a first success | Install, one working example, next steps | Concepts beyond the first run |
| How-to guide | Working, has a goal | Steps for one real task | Prerequisites, steps, how to tell it worked | Teaching, background |
| Reference | Working, needs a fact | Complete, consistent description | Every public item, types, defaults, errors | Tutorials, opinions on why |
| Explanation | Studying, wants to understand | Why it works this way, trade-offs | The idea, the alternatives, links to tasks | Steps |
| Troubleshooting | Stuck on a symptom | Cause and fix | Exact error text, cause, fix | Anything that isn't a symptom |
| Changelog | Upgrading | What changed in each version | Version, date, grouped changes | Raw commit log |
| Migration guide | Upgrading across a break | Steps from old to new | What broke, before and after, steps | Praise for the new version |
| README | Arriving | What it is and how to start | What, how to start, links | Everything the docs already hold |
| Decision record | Maintaining | Why a choice was made | Title, context, decision, status, consequences | Long surveys |
| Runbook | On call | What to do for this alert | One entry per alert, first steps | Theory |

## Which a project needs

This is inference. No source gives a minimum set by project type.

R = required. T = required once the trigger fires. O = optional. A dash = usually not needed.

| Doc | Library | CLI | Service | End-user app | Internal tool | Trigger |
| --- | --- | --- | --- | --- | --- | --- |
| README | R | R | R | R | R | |
| LICENSE | R | R | R if public | R if public | - | |
| Install or quickstart | R | R | R for consumers | R | R | |
| Tutorial | O | O | O | O | - | A real learning curve exists |
| How-to guides | T | T | T | T | T | A real recurring task exists |
| Reference | R, from code | R, as `--help` | R for its external interface | O | T | Others call it |
| Explanation | T | O | T | O | T | The model is hard to guess from the API |
| Changelog | R | R | T | R, as release notes | O | Consumers see the changes |
| Migration guide | T | T | T | - | T | A breaking change ships |
| Troubleshooting | O | O | T | T | O | The same failures keep coming up |
| Contributing guide | R if taking changes | same | same | same | O | |
| Security reporting | R if public | R if public | R if public | R if public | - | |
| Decision records | O | O | T | O | T | A costly-to-reverse decision is made |
| Runbook | - | - | R | T | T | Anything pages a person |
| Code comments | R | R | R | R | R | |

Start with the required rows and stop. An empty tutorial folder is worse than no tutorial.

Readers at work reach for coworkers, code and internal wikis before official docs. A project's
docs compete with those sources, so an internal doc has to beat asking a colleague or it won't be
opened. Prior decisions are the third most named context for agents in Stack Overflow's 2026 survey,
which is the only direct support for decision records as agent context.

## Rules

**Command:** Yes = a command decides. Flag = a command finds candidates, a person decides. No = a
person decides. The `audit.md` file lists every Yes and Flag rule with how to test it.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| T1 | Each page does one job: a reader can say which row of the table above it is. A page that teaches and also lists every option fails. | No | Opinion |
| T2 | Each page declares its type in front matter (`type: how-to`). Most checks in `audit.md` need it. | Yes | Doc |
| T3 | Split by the reader's task, not by content type alone. If one task needs more than two pages open at once, join them or link the exact section. | No | Opinion |
| T4 | No empty scaffolding: no empty section, stub page, `TODO`, `TBD`, "coming soon" or lorem ipsum. | Yes | Opinion |
| T5 | The README's first text says what the project is, and it has a section on how to start. | Flag | Measured |
| T6 | A public project has a LICENSE file. | Yes | Rule |
| T7 | A project has install or quickstart steps that reach a first working result. | Flag | Measured |
| T8 | A library, CLI or public API documents its whole public surface: every public item has an entry. | Yes | Doc |
| T9 | A CLI has `--help` on each command, and the help leads with usage and examples. | Flag | Opinion |
| T10 | Changelog entries carry a version and an ISO date, and group changes under the project's fixed group names. | Yes | Doc |
| T11 | Release notes are written for the reader, not pasted from the commit log. | Flag | Doc |
| T12 | A breaking change has migration steps: what broke, the replacement, and how to move. | Flag | Opinion |
| T13 | A project that takes outside changes has a contributing guide, and it names the test command. | Flag | Rule |
| T14 | A public project says how to report a security problem. | Yes | Rule |
| T15 | A decision record has title, context, decision, status and consequences. | Yes | Doc |
| T16 | A superseded decision record stays, marked with what replaced it. It isn't deleted. | Yes | Doc |
| T17 | Every alert in the repo has a runbook entry. | Yes | Doc |
| T18 | Every page has a place on the project's index or navigation. A page nothing links to fails. | Yes | Measured |

T6 and T14 are binding only for projects seeking the OpenSSF badge. T8's reach comes from SemVer:
a project can't version a public API it hasn't declared.

## Where the sources disagree

- **How many types:** Diátaxis says four. Cloudflare, Good Docs and others use more. Nobody measured
  it. Pick what fits the project and keep T1.
- **Separate or together:** Diátaxis wants types kept apart. Pigweed reported that strict separation
  scattered related content, and developers prefer reading in one continuous flow (Robillard). That
  is why T3 splits by task.
- **Generated or handwritten reference:** no evidence compares them. Generate the skeleton so it's
  complete and hand-write what code can't say: when to use it, and what goes wrong.
