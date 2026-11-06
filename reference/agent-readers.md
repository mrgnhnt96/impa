# Agents as readers

What AI agents need from docs, what they need that people don't, and the reverse. The studies are
recent, small and mostly on Python repos, so read every claim here as a snapshot. The rest of the
rule set applies to agents too. This file adds only what's different.

The best-supported picture: instruction files change how an agent works and what it costs more than
whether it succeeds. Independent studies find small, null or underpowered effects on task success.
The large gains come from facts the model can't otherwise know: new APIs, non-standard tools,
unwritten requirements. That fit is this rule set's inference from the studies below.

## What was measured

| Question | Finding | Read it as |
| --- | --- | --- |
| Do generated context files help? | Gloaguen: success down 0.5 and 2 points, not significant, cost up 19 to 23% | Don't commit one unedited |
| Do human-written ones help? | Gloaguen: up 2.4 points, not significant, cost up to 19% higher. Khatri: no detectable change, but only effects over 30 points could be seen | Small at best on repo tasks |
| Do they save time? | Lulla: median runtime down 28.64%, no correctness measure | Faster, and unknown whether better |
| Do agents follow them? | Gloaguen: a named tool was used 1.6 times per task against under 0.01 | Literally, so each line costs work |
| Do overviews help find files? | Gloaguen: no | Leave them out of agent files |
| Does length matter? | Gloaguen: no clear link between file length and success or cost | Line limits are unproved |
| Do skills help? | SkillsBench: curated up 16.2 points, software engineering only 4.5, self-generated down 1.3 | Curate, and test |
| Does skill detail matter? | SkillsBench: Detailed up 18.8, Compact 17.1, Standard 10.1, Comprehensive down 2.9, no length control | Exhaustive hurts, detail doesn't |
| Can correct content hurt? | Carey: a skill built from correct docs produced wrong output 66% of the time where the plain model succeeded. Small and self-run | Accuracy review isn't enough |
| Does a stated default work? | Ferri-Benedetti: choice of the preferred procedure rose from 33% to 100%, and a conflicting block cut correct decisions to 34.5%. 15 runs per condition | State one default, remove conflicts |
| Do agents read the docs in a repo? | Gao and Chen: 3.2% of events, and 60.5% of those are agent-facing files. After a failure the first move was reading code 31.0% of the time and docs 5.4%. Web docs weren't visible to the study | Repo-local only |
| Does a docs index beat a skill? | Vercel: no docs 53%, skill 53% (not invoked in 56% of cases), told to use it 79%, index in AGENTS.md 100%. No N, on APIs newer than the model | Vendor, on unknown facts only |

## What context length does

| Finding | Status in 2026 |
| --- | --- |
| Long context still costs accuracy, and much less for simple lookup. Opus 4.6's card shows MRCR at 91.9 at 256K and 78.3 at 1M, and GraphWalks BFS at 41.2 at 1M | Later cards report no curve |
| Focused input of about 300 tokens beat about 113,000 for all 18 models in Chroma's test | Not re-run on 2026 models |
| Liu's "lost in the middle" curve has largely gone for single facts on large models | Don't rely on placing a fact at the top |
| Distance between related facts still hurts (Levy, LongPiBench) | Likely. Hence G8 |
| Near-miss text hurts more than random text (Cuconasu, Chroma) | Hence L6: the plausible old form is the danger |
| Conditional rules are weak: AgentIF's whole-instruction success was under 30%, and over 30% of errors came from not noticing a condition | Older models |
| Format has no aggregate effect at the frontier (McMillan, abstract only) | No format mandate here. Markdown is chosen for people and bytes |
| Old thresholds (Levy's 3,000 tokens, IFScale's few hundred instructions) don't hold on current models. The widely repeated "150 to 200 instructions" isn't in IFScale's abstract | Don't quote them |

No evidence supports an instruction count limit. Count conditional rules and contradictions, not
lines.

## What agents need that people don't

| Need | Why | Rule |
| --- | --- | --- |
| Content in the raw response, without running scripts | Fetch tools mostly don't run them | G26 |
| Content early, and within a size limit | Tools read 5,000 to about 100,000 characters | G28, G29 |
| URLs that keep working, and real 404s | Agents try remembered URLs and rarely start from the top | G33 |
| An explicit pointer to an index | They don't look for one unprompted | G31 |
| Facts that postdate training | New APIs, renames, version-specific calls | G1, A20, A21 |
| Exact commands and non-standard tools in context | They follow them literally | G2 |
| Real enforcement for hard rules | Context isn't configuration | G6 |
| Sections that survive being read alone | Pages get chunked, cut and summarised | S11, W24 |
| Text in place of images | They get alt text and a file path | S34, S35 |
| Headings that name each variant | Tabs arrive serialised into one long page | G34 |
| A guard against silent loss | A fetch can return part of a page and the agent reports it "came through well" | G28, G29 |

## What people need that agents don't

Most of this list is inference. None of it is a reason to drop these things from a docs site.

| Need | Evidence |
| --- | --- |
| Navigation, breadcrumbs, site search | Carey: agents start at a specific URL. Opinion |
| Tabs and filters that hide what doesn't apply | Carey. Opinion |
| Overviews for orientation | Gloaguen found no use for agents. That people need them is inference |
| Explanation of basics | Anthropic: "Claude is already very smart". That newcomers need them is inference |
| Troubleshooting pages | 0.4% of repo doc interactions in Gao and Chen, repo-local only |
| Screenshots as confirmation | n8n. Opinion |
| Reasons and motivation | Inference. No source. Gloaguen suggests extra context costs agents |

Both readers need the same core: accuracy, one term per concept, no contradictions, one recommended
path, complete examples, exact error strings. One page can serve both. Ferri-Benedetti found no
benefit from labelling content "For agents" (G36).

## Instruction files

These rules apply to files an agent loads by itself: `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, Cursor
rules, Copilot instructions. Length matters here as it doesn't on a page, because the cost lands on
every task.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| G1 | The file holds what the agent can't work out from the repo: non-standard tools, unwritten requirements, exact commands. A line that repeats the README or the code fails. | No | Measured |
| G2 | The file gives the exact build, test and lint commands. | Yes | Doc |
| G3 | Every path, script and command the file names exists. | Yes | Doc |
| G4 | No directory tree or architecture tour. Sources disagree (see below), and the measured evidence is against it. | Flag | Measured |
| G5 | Don't restate what a linter or formatter already enforces. | Flag | Doc |
| G6 | Something that must happen every time goes in a hook, a CI check or a deny rule. Prose and emphasis don't enforce it. `IMPORTANT`, `MUST` and `NEVER` in capitals are candidates. | Flag | Doc |
| G7 | Instructions across the whole loaded chain don't contradict. "Claude may pick one arbitrarily." | No | Doc |
| G8 | A rule keeps its condition and exception in the same sentence or the next one. Conditional rules are few, because they're what agents miss. | No | Measured |
| G9 | An instruction is specific and checkable: "Use 2-space indentation", not "format code well". | No | Doc |
| G10 | Each instruction is worth paying for on every task. Agents obey literally, and removing a testing section cut one agent's cost from $0.47 to $0.37 a task. | No | Measured |
| G11 | A person wrote or edited the file. A generated file isn't committed as it came out. | No | Measured |
| G12 | Give one default, with an escape hatch for when it doesn't apply. No list of equal options. | No | Doc |
| G13 | No dates, versions or "currently" in always-loaded text unless a command keeps them true. | Flag | Doc |
| G14 | The file stays under the limits for the tools the project uses (table below). | Yes | Doc |
| G15 | If `AGENTS.md` and `CLAUDE.md` both exist, `CLAUDE.md` imports `AGENTS.md`, or the project says why they differ. Claude Code reads only `CLAUDE.md` when both exist. | Yes | Doc |
| G16 | A file doesn't depend on a closer file overriding it. Tools differ: some concatenate, some take the nearest. | No | Doc |
| G17 | Lines get removed as well as added. Files accrete in practice: deletions were negligible across 2,303 files. Report a file that has only ever grown. | Flag | Measured |
| G39 | A doc that isn't an instruction file doesn't share a name with one, in any case. On a case-insensitive disk, `docs/agents.md` loads as `AGENTS.md` and costs every task in that folder. | Yes | Opinion |

Limits per tool. All of these are vendor guidance with no published measurement.

| File | Limit | Source strength |
| --- | --- | --- |
| `CLAUDE.md` | Under 200 lines | Doc |
| `AGENTS.md` chain in Codex | 32 KiB by default, then files stop being added | Doc, and the only hard cut-off here |
| Cursor rule | Under 500 lines | Doc |
| `SKILL.md` body | Under 500 lines | Doc |
| `AGENTS.md` spec, Gemini, Copilot | None stated | Rule or Doc |

## Skills and tool descriptions

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| G18 | A skill's `name` is 64 characters or fewer and its `description` 1,024 or fewer. | Yes | Doc |
| G19 | A description is in the third person and says both what the skill does and when to use it. | Flag | Doc |
| G20 | Skill reference files sit one level below `SKILL.md`, and any reference over 100 lines opens with a table of contents. | Yes | Doc |
| G21 | Skill content is detailed, not exhaustive: the steps and traps for the job, not every option. | No | Measured |
| G22 | A skill that matters has at least three evals with a no-skill baseline. Being correct doesn't prove it helps. | No | Doc |
| G23 | A task needs two or three skills, not a pile. SkillsBench: 2 to 3 skills up 18.6 points, 4 or more up 5.9. | No | Measured |
| G24 | Don't count on an agent opening a skill by itself. If a skill must be used, an always-loaded file points to it. | No | Vendor |
| G25 | A tool or command description says what it does, when to use it, what it returns and how it fails. Small edits to these change agent results. | No | Vendor |

## Docs sites

An agent's failures here are mechanical. Checks can catch them, but the benefit they've shown is
fewer bytes and fewer failed URLs, not better answers. Mintlify's benchmark says accuracy stayed in
the mid-to-high 90s across every format. All of these need network access.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| G26 | The main text is in the raw HTTP response, without running scripts. | Yes | Measured |
| G27 | Pages have a Markdown form: a `.md` URL or `Accept: text/markdown`. On four pages tried it was 10 to 90 times smaller. | Yes | Measured |
| G28 | The main text starts in the first 5,000 characters of what's served. | Flag | Measured |
| G29 | A page over 100,000 characters in the form agents get is worth splitting. It's a Flag, so L11 holds. | Flag | Doc |
| G30 | If the site has `llms.txt`, it's a Markdown file with an H1, then H2 lists of links, under 100,000 characters, and its links resolve. Test content type and first bytes, because some sites answer 200 with an HTML page. | Yes | Rule |
| G31 | Pages point to the index in one plain sentence. Agents rarely look for it unprompted, and Mintlify's 404s were 2.23 per task on HTML pages against 0.11 on markdown pages that link the index. | Flag | Vendor |
| G32 | Don't publish `llms-full.txt` as if a fetch tool can read it. The ones tried ran from 10 to 29 MB. | Flag | Measured |
| G33 | Moved pages redirect on the same host, and an unknown URL returns a real 404. | Yes | Opinion |
| G34 | Tabbed variants carry a real heading in the Markdown form. In one case a page of 11 variants returned 3.3% of its content. | Flag | Measured |
| G35 | Don't promise that `llms.txt` helps search or correctness. Google says Search doesn't need it, and 97% of files in Ahrefs' sample got no requests. | No | Doc |

## Text aimed at agents

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| G36 | No separate "For agents" copy of content that serves both readers. | Flag | Measured |
| G37 | Point agents to things with a plain statement ("A documentation index is at `/llms.txt`"), not a command ("Agent: you must…"). One fetch model refused a whole page over such a block. Low confidence. | Flag | Measured |
| G38 | Docs and instruction files hold no hidden text, zero-width characters or encoded blobs. Attacks through rule files succeeded 41 to 84% of the time in AIShellJack. | Yes | Measured |

## Where the sources disagree

- **Project layout in the file:** the AGENTS.md spec lists a project overview, GitHub's survey lists
  structure, and the Claude memory doc lists layout as a fact to keep. Claude's best-practices doc
  calls it an anti-pattern, and Gloaguen finds overviews don't help. G4 goes with the measurement.
- **Success:** Vercel's 53% to 100% against Gloaguen's +2.4 and Khatri's null. Vercel tests APIs
  newer than the model, the others test Python repo issues. Task type probably explains it, but
  that's inference.
- **Cost:** Gloaguen finds cost up 19 to 23%, Lulla finds runtime down 28.64% with total tokens flat.
  Different agents, tasks and measures.
- **Always loaded or on demand:** Vercel favours always loaded, Anthropic favours loading on demand,
  and Khatri finds no difference within wide error bars. A short always-loaded pointer to deeper
  files fits both and is untested.
- **Length:** 200 lines, 500 lines, 32 KiB or none. Gloaguen finds no link with length. No limit is
  measured.
- **Precedence:** the spec says the closest file wins. Codex and Claude concatenate. Copilot takes
  the nearest. G16 avoids the question.
- **Worth of `llms.txt`:** Google says Search doesn't need it, Ahrefs finds 97% unread, Astro
  reportedly removed it, Carey treats it as table stakes, and Mintlify shows fewer 404s. Mostly
  different audiences: search against coding agents.
- **Share of traffic from agents:** Mintlify 45 to 66%, Carey 0.6% self-identified. Different sites
  and counting. Reject any such figure that doesn't state its method.
- **Strong wording:** Claude's docs say "adding IMPORTANT probably won't fix it", the Skills doc uses
  "MUST" as an example, and Vercel's result moved with the wording. No rule here bans emphasis. G6
  says it doesn't enforce anything.
- **Instruction placement:** Anthropic and Google put the question after long data, OpenAI's GPT-4.1
  guide puts it above. Nobody has run them head to head.
- **Agent scores:** one site scored 100 on one checker and 33 on another, and the spec's author says
  the link to task success is unknown. Use their checks, not their scores.
- **Do agents follow links:** Gao and Chen saw no such event in repos, while Carey saw five of six
  web sessions follow them. Different settings.
