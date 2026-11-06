# Accuracy

Keeping docs true to the code, and how to check that they are. The numbers on how often docs drift
are solid. What prevents drift rests on surveys, interviews and documented practice, because no
study has measured drift before and after any single practice. A wrong page costing more than a
missing one is a reasonable reading of the evidence, and nobody has tested it on current agents.

The main idea: a command can tell that a named thing exists. It can't tell that a sentence about the
thing is true. So move as many facts as possible to where a command can decide them, and spend
review on the rest.

## What was measured

| Question | Finding | Read it as |
| --- | --- | --- |
| How common is drift in READMEs and wikis? | Tan: 28.9% of top GitHub projects, 19.2% of docs and 3.9% of references were outdated. The Google sample showed 5.4%, 9.7% and 2.7% | Common per project, rare per reference |
| In agent instruction files? | Treude and Baltes: 23.0% of repos held a stale reference, 1.27% of references | The same shape, on the files agents read most |
| How long does it last? | Tan: outdated references were a mean of 4.7 and 4.2 years old | Nobody notices by chance |
| Which side was wrong? | Tan: of resolved cases, 47.6% were fixed in the source, 39.1% in the doc, and 13.3% by deleting the doc | A mismatch doesn't say which side is wrong |
| Do comments keep up with code? | Wen: 13 to 20% of code changes triggered a comment update, across 1,500 Java systems. The authors say the rest aren't shown to be inconsistent | Most changes need no doc change, so "code changed, doc didn't" is a weak signal |
| Do links last? | Hata: 9.1% of unique links in code comments returned 404, and 8.6% returned 403, 405 or 500, which don't clearly mean dead | Check on a schedule, and don't block on them |
| Do install docs work? | Milliken: an agent installed 21 of 40 Python repos at least once, 28.8% on average. Only 36.5% of successes needed no repair | Install steps need a real run |
| Is wrong the biggest problem? | Aghajani counted completeness at 268 issues, up-to-dateness 190, correctness 72. Uddin: incorrectness is rare, and severe when seen | Missing is more common, wrong costs more each time |
| Does wrong text mislead? | ClashEval: models took wrong context over a correct prior more than 60% of the time. Kabir: 12 users missed errors in fluent answers 39% of the time | Plausible and wrong is the danger |
| What helps with changed APIs? | Ashik: docs raised update adoption to 92.87%, and modified APIs were hardest. A version string alone added about 0.5 points in LibEvolutionEval and about 3 in VersiCode, while the right imports added 21 | Show old and new code, not just a version |
| Is a raw drift check right? | Treude and Baltes: of 50 sampled stale references, 32 genuine, 12 false, 6 ambiguous. One annotator | Reproducible isn't precise |
| Can a model find wrong docs? | Kiecker: the best method fell from .88 precision on a balanced set to .39 at 10% inconsistent, and single prompts fell to .10 to .20. DocPrism: .62 in the wild | Triage, never a verdict |

All the model numbers above are on function-level comments, using small or older models. No study
tests a model reviewing a README or a guide against a repo.

## One source for each fact

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A1 | Reference that can come from the code does: command flags, API signatures, config defaults and schemas are generated, not typed. | No | Doc |
| A2 | A generated block matches a fresh generation. CI runs the generator's check mode, and a difference fails. | Yes | Doc |
| A3 | A snippet injected from source matches that source. | Yes | Doc |
| A4 | Every include target exists, and a missing one fails the build. Some tools drop it silently or only warn. | Yes | Doc |
| A5 | Injected snippets use a named region or symbol, not line numbers, which quote the wrong lines after an edit. | Yes | Opinion |

## Names the docs use

These are the only prose-to-code checks that work without a language parser. A clean result means
the names exist, not that the sentences around them are true.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A6 | Every relative link points to a file that exists. | Yes | Doc |
| A7 | Every anchor points to a heading that exists. | Yes | Doc |
| A8 | A path in code font exists in the repo, unless the text marks it as an example or as something the reader creates. | Flag | Measured |
| A9 | A version the docs give for this project matches its manifest. Use a template variable where the tool has one. | Flag | Opinion |
| A10 | A code name that was in the repo when the line was written and isn't now gets looked at. | Flag | Measured |
| A11 | A flag or subcommand of the project's own program appears in that program's help. | Flag | Opinion |
| A12 | An environment variable the docs name appears in the code. | Flag | Opinion |
| A13 | JSON, YAML and TOML blocks parse. | Yes | Opinion |
| A14 | The language's doc tool fails on a broken reference in a doc comment, and CI runs it. | Yes | Doc |
| A15 | External links are checked on a schedule, not as a merge gate. | Flag | Measured |

## Claims only a person or a run can settle

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A16 | Each default, limit, unit, ordering and side effect in the text has been checked against the source or by running it. | No | Measured |
| A17 | A number about speed or scale says how it was measured or links to the benchmark. If not, it goes. | No | Opinion |
| A18 | Before a release, someone who didn't write the page follows each first-run page from a clean state and logs where they got stuck. | No | Opinion |

What a command can't decide, and what stands in. This split is this rule set's synthesis, and no
source validates it.

| Claim | Why a command can't | What can stand in |
| --- | --- | --- |
| Behaviour: "retries three times" | The name exists either way | Run it, or read the source |
| A default stated in prose | Prose has no link to the code | Generate it from the schema (A1) |
| Step order | Each step can be valid alone | Run the steps from a clean state (A18) |
| Performance | Depends on the machine and load | A benchmark the page cites (A17) |
| Completeness | A mention isn't an explanation | A reader trying the task |
| Whether the page does its job | No proxy exists | A reader trying the task |

## When behaviour changes

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A19 | A change to behaviour updates the docs in the same pull request. | Flag | Doc |
| A20 | A changed API or command is documented with old and new together, as code, and the old form is labelled. A version number alone isn't enough. | No | Measured |
| A21 | A removed or renamed public name stays findable: a migration page or changelog entry gives the old name beside its replacement. | Flag | Measured |

A20 and L6 pull against each other on purpose. Near-miss text is what misleads, so an old form
appears only where it's labelled, on a page about the change.

## Upkeep

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A22 | Each docs path has a named owner. `docs/*` in CODEOWNERS isn't recursive, and `/docs/` is. | Yes | Doc |
| A23 | The project writes down who updates docs, when, and how a page is retired. | No | Measured |
| A24 | A `reviewed` date older than the project's limit is reported. The date is a reminder, not proof that the page is right. | Flag | Doc |
| A25 | A page nobody can keep true is deleted, not left to rot. | No | Doc |

## How to review for accuracy

For a reviewer, person or model. On real code, only about four to six in ten model findings held up
when people checked them, so the order and the limits matter as much as the checking.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| A26 | Run the audit first, and spend review only on what it can't decide. | No | Opinion |
| A27 | Take each checkable claim and ask "is this sentence false?", not "is this doc good?". The broad question drifts to style. | No | Measured |
| A28 | A finding quotes the doc line and the code line or command output. No evidence, no finding. | No | Opinion |
| A29 | Prefer running to reading, when it's safe. Read the source for defaults and behaviour. | No | Opinion |
| A30 | Give the reviewer the source or docs of the project's dependencies. Missing API knowledge caused most false alarms in Python in DocPrism. | No | Measured |
| A31 | Saying less than the code isn't an inconsistency. Omission belongs to a completeness review. | No | Measured |
| A32 | Report "the doc and the code disagree", not "the doc is wrong". The code may be the bug. | No | Measured |
| A33 | Model findings are triage. They never block a merge on their own. | No | Measured |

## Where the sources disagree

- **Wrong or missing:** Write the Docs says "Consider incorrect documentation to be worse than
  missing documentation". Aghajani counts missing far more often, and Uddin finds wrong rare but
  severe. Treude leaves accuracy out of its quality dimensions on purpose, since it can only be
  checked against other artifacts. A rule set that polices only accuracy would miss the larger
  problem, and no study compares wrong docs with no docs on current agents.
- **How good model detectors are:** CCISolver reports GPT-4o at about 85% precision on a near-balanced
  benchmark, DocPrism .62 in the wild, and Kiecker .10 to .20 for single prompts at 10% inconsistent.
  They differ in base rate, unit and label quality. These rules go with the last two for real use.
  CCISolver also found 45.67% of sampled positives in an older benchmark were mislabelled, so older
  precision figures are soft.
- **Deterministic or precise:** Treude and Baltes recommend deterministic detection for precision,
  yet their own sample was 64% genuine. Deterministic means the same answer each time. A10 stays a
  Flag.
- **Outdated doc or buggy code:** tools say "outdated", but about half of Tan's resolved cases were
  fixed in the source, and iComment found 33 bugs against 27 bad comments. A32 follows from this.
- **Comments or far-off docs:** Fluri (through Wen) has 90 to 97% of source-driven comment changes
  landing in the same revision, while Tan finds README staleness lasting years. Docs close to the
  code seem to keep up better, which is inference, since the two percentages count different things.
- **When to update:** Google says in the same change, which is advice. Dagenais found most projects
  required docs by release, which is what happened. A19 goes with Google, with no measured benefit.
- **Docs as tests:** docs-as-tests advocates want every claim executable. Python's doctest page warns
  that "filling your docstrings with obscure test cases makes for bad documentation". Test the
  examples written for readers. Don't turn pages into tests.
- **Does verifiable text help agents:** the advice assumes so. Gao and Chen saw no doc-based
  validation in 557 agent sessions, and docs were the first recovery move in 5.4% of failures. That's
  one corpus, and it doesn't show verifying is useless.
- **Does writing docs improve design:** Dagenais reports contributors finding design flaws while
  documenting, and two contributors said review and tests catch the same things.
