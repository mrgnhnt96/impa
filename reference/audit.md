# Audit

Every rule a command can decide or flag, and how. This is the build list for `impa audit`. Nothing
here has been run on a real repo yet, so the False alarms column is an estimate. Measure it on real
docs and narrow or drop any check that fires on sound text.

A clean audit proves little. These checks see dead links and examples that won't compile, and the worst
problems (a wrong sentence, a missing topic) mostly need a person. See `principles.md`.

## How the audit behaves

| Part | Behaviour |
| --- | --- |
| Yes rule | The command decides. The Level column says whether it blocks or warns. |
| Flag rule | The command lists candidates and never blocks. A person decides. |
| No rule | Not in this file. A reviewer checks it. |
| Parsing | Read Markdown as a syntax tree, not with regular expressions over the text. |
| Prose checks | Skip code blocks, inline code, link targets, front matter and quoted text. |
| Page type | Read from front matter (T2). A page with no type gets only the checks that need no type. |
| Project choices | Read from the project's config. A check with no configured choice compares pages with each other, and stays quiet on a one-page project. |
| Network | Off by default. Checks that need it run only on request. |
| Running code | Off by default. Checks that run the project's own program (T9, A11) run only on request. |
| Certainty | A finding says how sure it is. A missing file is certain, a vanished name is a candidate. |
| Unknown | If a check can't tell, it stays silent. |
| Word lists | Short and exact. A project can add to them, and turn any check off with a reason. |

**Rules for the audit itself:**

| Rule | What it means |
| --- | --- |
| W13 | Never compute or gate on a readability score. |
| L11 | Report size if useful, and never fail a page on word count. The only size checks that can block are the limits on always-loaded files in G14. |

**Needs** in the tables below: Type = declared page type. Config = a project choice. Repo = reads
outside the docs, such as code, CI files or git history. Net = network. A dash means only the page.

**False alarms** is Low, Medium or High, and is an estimate.

## Doc types

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| T2 | Page has a `type` in front matter from the allowed list | - | block | Low |
| T4 | No empty section, and no `TODO`, `TBD`, "coming soon" or lorem ipsum in prose | - | block | Low |
| T5 | README has text before the first H2, and a heading matching start, install or usage | Repo | warn | Medium |
| T6 | A `LICENSE` file exists | Repo, Config (public) | block | Low |
| T7 | A page of type quickstart or tutorial exists, or README has an install heading | Repo | warn | Low |
| T8 | Each public item from the language's doc tool has an entry. Run that tool where one exists | Repo, Config | block | Medium |
| T9 | Run `--help` on each command and report any failure, or help with no usage or example line | Repo, Config | warn | Medium |
| T10 | Each changelog heading has a version and an ISO date, and group names come from the project's list | Repo, Config | block | Low |
| T11 | Changelog entries that look like commit subjects: `fix:`, hashes, `Merge pull request` | Repo | warn | Medium |
| T12 | A breaking entry in the changelog with no link to a migration page or section | Repo | warn | Medium |
| T13 | `CONTRIBUTING` exists and holds a code span or block with a test command | Repo | warn | Medium |
| T14 | `SECURITY.md` exists, or a security heading in README or contributing | Repo, Config (public) | block | Low |
| T15 | A decision record has title, context, decision, status and consequences | Type | block | Low |
| T16 | A deleted decision record in git history, or a superseded status with no link to the replacement | Type, Repo | block | Low |
| T17 | Each alert name in the project's alert config has a runbook entry | Repo, Config | block | Medium |
| T18 | Each page is linked from another page or the navigation file | Repo | block | Low |

## Structure

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| S1 | First block after the H1 is a paragraph with one or two sentences | - | warn | Medium |
| S2 | First sentence doesn't start "This page", "This topic", "This guide", "In this guide" and the like | - | block | Low |
| S3 | Nothing but a paragraph comes first after the H1: no notice, image or badge | - | block | Low |
| S6 | A prerequisites heading, if present, comes before the first numbered list or steps heading | Type | block | Low |
| S7 | Last step or section of a tutorial or how-to holds an output block or words such as "you should see" | Type | warn | High |
| S8 | A next-steps heading is the last one and has five links or fewer | - | block | Low |
| S9 | Heading sequence on reference pages matches the project's template, or the most common sequence | Type, Config | block | Medium |
| S10 | Page has more than eight H2s and no table of contents | Config (site generates one) | warn | Low |
| S12 | One H1, no skipped level, no empty heading | - | block | Low |
| S13 | Headings match the configured case, allowing for code spans and names in the term list | Config | block | Medium |
| S14 | Heading text isn't one of the banned generic words | - | block | Low |
| S15 | Heading has no end period and no link. Code span only on reference pages | Type | block | Low |
| S16 | No heading directly follows another | - | block | Low |
| S17 | Three or more sibling headings share their first two words | - | warn | Low |
| S18 | Task headings on how-to pages match the configured form | Type, Config | warn | Medium |
| S19 | Title over 65 characters, or a heading outside 3 to 11 words on non-reference pages | Type | warn | Medium |
| S20 | No list with one item | - | block | Low |
| S22 | List has no paragraph before it, outside Prerequisites and Steps headings | - | warn | Medium |
| S23 | List has more than seven items | - | warn | Medium |
| S24 | List mixes items with and without end punctuation, or first-word parts of speech differ | - | warn | High |
| S25 | Table has two or more columns, a header row and no blank cell | - | block | Low |
| S26 | No two notices in a row, and at most one in a section | Config (notice syntax) | block | Low |
| S27 | A step contains "and then", "then" or two imperative clauses | Type | warn | High |
| S28 | A step's first word isn't an imperative verb, "If", "In", "On" or "Optional:" | Type | warn | Medium |
| S29 | A step ends with an "if" clause | Type | warn | Medium |
| S30 | A step with "optional" or "optionally" in it starts with `Optional:` | Type | block | Low |
| S31 | A code block under a step holds more than one command | Type | warn | Medium |
| S32 | Link text isn't on the banned list, and no bare URL in prose | - | block | Low |
| S33 | One link text with two targets on a page | - | block | Low |
| S34 | Every image has alt text | - | block | Low |
| S35 | An image is the only thing under a step or a heading | - | warn | Medium |
| S36 | Each entry under an error page has the code or message, a cause heading or label, and a fix one | Type, Config | block | Medium |
| S37 | Troubleshooting heading starts with a fix word or names no symptom: "Fixing", "How to", "Configure" | Type | warn | Medium |
| S38 | Each error code found by the project's pattern in the source has an entry | Repo, Config | warn | Medium |
| S39 | Error strings in source that have a doc entry but no link | Repo, Config | warn | High |

## Wording

Every wording check warns. `wording.md` says why: Vue and Django ban minimising words and still ship
them. A project can raise a list to block by choice.

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| W2 | Spellings that differ only by case, hyphen or space ("sign-in", "signin"), and a term the config marks as replaced | Config | warn | Medium |
| W3 | An all-capitals word of two to six letters used with no expansion or link, outside a known list | Config | warn | Medium |
| W5 | "fast", "faster", "easy", "best", "better" in a sentence with no number | - | warn | High |
| W6 | Passive verb forms with no "by" | - | warn | High |
| W7 | Prose on task pages with "the user" or "the developer" | Type | warn | Medium |
| W8 | "will" followed by a verb | - | warn | High |
| W9 | "will support", "planned", "coming", "roadmap" | - | warn | Low |
| W10 | Sentence over 25 words, outside lists and tables | - | warn | Medium |
| W11 | Paragraph over five sentences or 150 words | - | warn | Medium |
| W14 | "please" in prose | - | warn | Low |
| W15 | "e.g.", "i.e.", "etc." | - | warn | Low |
| W16 | "utilize", "in order to", "allows you to", "via" | - | warn | Low |
| W17 | "!" in prose | - | warn | Low |
| W18 | The older terms in the inclusive list | - | warn | Low |
| W19 | Minimising words on task pages | Type | warn | Medium |
| W20 | "currently", "latest", "soon", "recently". Never "now" or "new" | - | warn | Medium |
| W21 | "you can" | - | warn | High |
| W22 | Marketing words on task pages | Type | warn | Medium |
| W23 | A short list of common idioms | Config | warn | Medium |
| W24 | "above", "below" as pointers: "see above", "shown below", "the following" is allowed | - | warn | Medium |
| W25 | A word in quote marks that matches a code span elsewhere on the page, or a name with `_`, `()` or a file extension outside code font | - | warn | Medium |
| W26 | Pages that use contractions against pages that don't, or against the configured choice | Config | warn | Low |
| W27 | "we" and "our" against the configured choice | Config | warn | Low |

## Examples

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| E1 | How-to, tutorial and quickstart pages have a code block. Reference entries for callables have one | Type | block | Low |
| E2 | A code block that uses a name with no import or definition, for languages with a parser | Config | warn | High |
| E4 | `foo`, `bar`, `baz`, `example123` inside code blocks | - | block | Low |
| E5 | Placeholders in code match the configured format, and each is mentioned in the text | Config | block | Medium |
| E6 | A line of only `...` or `…` in a code block, for languages where that isn't valid syntax | - | block | Medium |
| E7 | Secret patterns and real-looking email addresses in code. Use a secret scanner's rules | - | warn | Medium |
| E8 | Words such as "don't", "wrong", "avoid" in the sentence before a block with no label on the block | Config | warn | High |
| E9 | Each fenced block has a language tag | - | block | Low |
| E10 | A paragraph or list item comes directly before each code block | - | block | Low |
| E12 | A step's command block with no output block or "you should see" nearby | Type | warn | High |
| E13 | A block with a `$` line followed by lines that have no prompt | - | warn | Medium |
| E14 | Command blocks mix `$` prompts and no prompts across the project | Config | block | Low |
| E15 | A block over 25 lines or 85 columns | - | warn | Medium |
| E16 | How-to page with no error-word heading, and no error text in a block | Type | warn | High |
| E17 | The project's doc-test tool is present and CI runs it | Repo, Config | block | Medium |
| E18 | Reads the project's recorded rung for install, quickstart and tutorial pages, and lists any below rung 3 | Type, Config | warn | Low |

## Leave out

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| L2 | A reference description whose words are all in the item's name and type | Type | warn | Medium |
| L3 | Opening text with "welcome", "thanks for", "history", or "was created" | - | warn | Medium |
| L4 | "previously", "used to", "in older versions" outside changelog and migration pages | Type | warn | Medium |
| L5 | A paragraph of 30 words or more that repeats on another page, ignoring notices | - | warn | Medium |
| L6 | "deprecated", "removed", "legacy" near a code span with no replacement link or name | - | warn | High |
| L10 | A heading matching FAQ or "Frequently asked questions" | - | warn | Medium |

## Accuracy

These compare docs with the repo. They prove a name exists, not that a sentence is true. Drift is
rare per reference, about 1 to 4%, so even a good check produces many false alarms. Only A10 has a
measured rate: 64% of raw reports were genuine, before the gate described here.

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| A2 | Run the generator's check mode, or compare the marked block with fresh output | Repo, Config | block | Low |
| A3 | Run the injection tool's verify mode, or compare the block with its source | Repo, Config | block | Low |
| A4 | Each include directive's target exists | Repo | block | Low |
| A5 | Include directives with a line range: `#L20-L30`, `:lines:` | Repo | warn | Low |
| A6 | Each relative link's target exists | Repo | block | Low |
| A7 | Each anchor matches a heading under the configured renderer's slug rule. Off until a renderer is set | Config | warn | Medium |
| A8 | Code spans shaped like a path, absent at HEAD. Skip placeholders and paths marked as examples | Repo | warn | Medium |
| A9 | The project's name beside a version, compared with `Cargo.toml`, `package.json` or `pubspec.yaml`. Skip changelog and migration pages | Repo, Type | warn | Medium |
| A10 | Code spans with `_`, `.`, `/`, `::` or camelCase that are absent at HEAD and were present at the commit that last changed the line. Skip changelogs, generated files and text sources | Repo | warn | Medium |
| A11 | Flags and subcommands of the project's own program, compared with its `--help`. On request | Repo, Config | warn | Low |
| A12 | Capitalised names with `_` in code font, absent from `git grep`. Skip a list of platform variables | Repo, Config | warn | Medium |
| A13 | Fenced `json`, `yaml` and `toml` blocks parse | - | warn | Low |
| A14 | CI runs the language's doc tool with broken references as errors. Don't reimplement the tool | Repo, Config | block | Low |
| A15 | External links respond. Treat 403, 405 and 500 as unknown | Net | warn | Medium |
| A19 | Files a page names that changed after the page last did. Most code changes need no doc change | Repo | warn | High |
| A21 | Names dropped from the public list between two tags, absent from changelog and migration pages | Repo, Config | warn | High |
| A22 | CODEOWNERS covers each docs path | Repo, Config | warn | Low |
| A24 | Front matter `reviewed` date older than the configured limit | Config | warn | Low |

## Agents

| Rule | Check | Needs | Level | False alarms |
| --- | --- | --- | --- | --- |
| G2 | An instruction file names a build, test and lint command in a code span or block | Repo | block | Low |
| G3 | Each path and script an instruction file names exists, and each command's program is known | Repo | block | Medium |
| G4 | A directory tree block in an instruction file | Repo | warn | Low |
| G5 | Style words such as "indent", "semicolon", "quotes" when a linter or formatter config exists | Repo | warn | Medium |
| G6 | Capitalised `IMPORTANT`, `MUST`, `NEVER`, `ALWAYS` in an instruction file | Repo | warn | Medium |
| G13 | Dates, version numbers and W20's words in an instruction file | Repo | warn | Medium |
| G14 | Lines and bytes against the limit table, for tools the project names. The Codex 32 KiB limit counts the whole chain | Repo, Config | block | Low |
| G15 | Both files exist and `CLAUDE.md` has no import of `AGENTS.md` | Repo | block | Low |
| G17 | `git log` shows the file has only grown, across five or more changes | Repo | warn | Medium |
| G39 | A doc file named `agents.md`, `claude.md` or `gemini.md`, matching an instruction file with only the case changed | Repo | block | Low |
| G18 | `name` is 64 characters or fewer, `description` 1,024 or fewer | Repo | block | Low |
| G19 | Description starts with "I", "You" or "Use this" with no third-person verb, or has no "when" or "use" clause | Repo | warn | Medium |
| G20 | Links from a reference file to a further reference file. Any reference over 100 lines with no table of contents in its first lines | Repo | block | Low |
| G26 | Fetch with no script execution, and find the page's first heading in the response | Net | block | Low |
| G27 | Request the `.md` URL, or send `Accept: text/markdown`, and check the content type | Net | warn | Low |
| G28 | Offset of the first heading in what's served | Net | warn | Medium |
| G29 | Length of the Markdown form, or of the source file | - | warn | Low |
| G30 | Content type isn't HTML, first bytes are `# `, H2 lists parse, size under 100,000 characters, links resolve | Net | block | Low |
| G31 | Pages hold a line naming the index path | Net | warn | Medium |
| G32 | `llms-full.txt` exists and is over 100,000 characters | Net | warn | Low |
| G33 | A made-up URL returns 404. Old paths from git history or the redirect file redirect on the same host | Net, Repo | block | Medium |
| G34 | Tab components in the source, and pages with more than one variant and no heading for each | Config | warn | Medium |
| G36 | A heading or block labelled "For agents", "For AI", "For LLMs" | - | warn | Medium |
| G37 | Text starting "Agent:", "AI agents must", "If you are an agent" | - | warn | Low |
| G38 | Zero-width and bidirectional characters, HTML hidden text, long base64 runs. Scan docs and instruction files | - | block | Low |

## Order to build

Most value for least work first. This order is judgment.

1. Page-only structure checks: S2, S3, S12, S14 to S16, S20, S25, S32 to S34, E9, E10.
2. Instruction-file checks: G2, G3, G14, G15, G18, G20, G38, G39.
3. Type-based checks once T2 is in place: E1, S6, S9, S15, S30, T15.
4. Accuracy checks that compare docs with the repo: A4, A6, A8, A10, A13, then the rest.
5. Word lists, all at warn: W14 to W22.
6. Network checks, on request only.

Markdown syntax-tree checks are the gap worth filling. Vale already does word lists well, so a
project using it can leave those to Vale.

A10 is the one check no mature tool has. Treude and Baltes did the whole search with `git grep`, so
it needs no language parser. A Markdown parser that gives source offsets, a git binary and TOML,
JSON and YAML parsers cover everything else offline. Leave these to the tools that own them: doc
tests, doc-comment references, OpenAPI rules and grammar-aware prose rules.
