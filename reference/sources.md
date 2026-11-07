# Sources

Researched 2026-11-06. Re-check agent, platform and model claims before relying on them for long:
they change quickly.

**Strength:** Rule = binding policy for those it covers. Doc = documented by a style guide, spec or
vendor's own guidance. Measured = a study. Vendor = vendor data, usually correlational. Observed =
seen in the praised docs, which were hand-picked, so correlation only. Opinion = expert or
practitioner judgment, including this rule set's own where a row says so.

The Rules column names the rules each row supports. Every rule ID in the topic files appears here.

## What readers do and what hurts them

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| W1, L12 | All six blocker problems were content problems. Ambiguity was top priority (54 of 104) and is defined as missing information | Measured, self-report, 323 IBM staff, 2015 | https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf |
| T18, L12 | Completeness 55% of content issues, up-to-dateness 39%, findability half of usability issues | Measured, 878 artifacts, 2019 | https://emadpres.github.io/pdfs/icse2019.pdf |
| T7, E18 | Clarity important to 88%. Faulty tutorial 65%, code and docs disagreeing 59%, superfluous content 55%, lengthy files 35% | Measured, self-report, 146, 2020 | https://emadpres.github.io/pdfs/icse2020.pdf |
| T7, E18 | Install instructions the most frequent issue. Read as chapter 6 of the dissertation | Measured, self-report, 146, 2020 | https://folia.unifr.ch/documents/319169/files/2020INFO014.pdf |
| E1, E2 | Three cURL examples raised success (p=.0003). Time not significant (p=.051) | Measured, randomised, 26, 2017 | https://www.cs.mcgill.ca/~martin/papers/vlhcc2017.pdf |
| S5, S11, E5 | Opportunistic and systematic readers. Placeholders blocked pasting. Repeat critical information | Measured, 11, 2019 | https://mangold-international.com/resources/meng-et-al-how-developers-use-api-documentation-2019.pdf |
| S4 | Added content raised accuracy 74% to 86%, ratings unchanged | Measured, 22, 2020 | https://mangold-international.com/resources/meng-et-al-optimizing-api-documentation-2020.pdf |
| E2 | 78% of 252 sessions had reminder traits. About a third of project code copied from the web | Measured, 2009 | https://pg.ucsd.edu/publications/opportunistic-programming-two-studies_CHI-2009.pdf |
| S4, S11 | Guide visitors had 3.92 times the odds of later API calls. Name the audience, repeat key information | Measured, correlational, 100,000+ users, 2024 | https://arxiv.org/pdf/2310.10817 |
| E3 | Examples the largest resource obstacle. Examples that miss the reader's goal hinder | Measured, self-report, 80 answers, 2009 | https://www.cs.mcgill.ca/~martin/papers/software2009a.pdf |
| E3, E11, L2, L3, L8 | Boilerplate wastes the reader's "implicit query". Trivial one-call examples disliked. Member-level completeness can backfire. Continuous presentation preferred | Measured, self-report, 440+, 2011 | https://www.cs.mcgill.ca/~martin/papers/ese2011.pdf |
| L2 | 43% of JDK and 51% of .NET member units carry little or no value | Measured, 5,574 units, 2013 | https://www.cs.mcgill.ca/~martin/papers/tse2013a.pdf |
| L8, L11 | Minimal manual: more tasks done, less time on errors, training time not significantly lower. Bundled changes | Measured, randomised, 25, 1993 | https://ris.utwente.nl/ws/files/6853553/Meij93assessment.pdf |
| L11 | Minimalist users learned faster and better | Measured, 64, 1993, abstract only | https://research.utwente.nl/en/publications/f811580e-953a-4312-a1ee-9916b3f98e05 |
| E16 | Error information in manuals improves learning | Measured, 1995, abstract only | https://research.utwente.nl/en/publications/7e445104-88ad-481f-87cf-0539f55ae327 |
| T5 | READMEs: What 97%, How 88.5%, Contribution 27.8%, Why 25.7% | Measured, 393 READMEs, 2019 | https://arxiv.org/pdf/1802.06997 |
| | People missed errors in polished AI answers 39% of the time | Measured, 12 users, 2024 | https://arxiv.org/pdf/2308.02312 |
| | Docs quality tied to 2.4 times better performance in 2021, negative in 2022, no effect on delivery in 2023 | Vendor, correlational | https://dora.dev/capabilities/documentation-quality/ |
| S5 | Most people scan: 79%. At most 28% of words read | Measured, 1997 and 2008 | https://www.nngroup.com/articles/how-little-do-users-read/ |
| S5 | 57% of viewing on the first screen, 74% on the first two | Measured, 120, 2018 | https://www.nngroup.com/articles/scrolling-and-attention/ |
| S17 | Link labels judged on about the first 11 characters | Measured, 2009 | https://www.nngroup.com/articles/first-2-words-a-signal-for-scanning/ |
| L11 | Sun whitepapers rewritten at 54% length, +159%. Structure changed too | Measured, 21, 1998 | https://www.nngroup.com/articles/applying-writing-guidelines-web-pages/ |
| T18 | 34% can't find existing APIs. Docs scattered, not absent | Vendor, sells docs tooling, 5,700+, 2025 | https://www.postman.com/state-of-api/2025/ |
| | Official docs 30.5% for work questions, behind coworkers and code. Prior decisions named by 44% as agent context | Measured, survey, 2026 | https://survey.stackoverflow.co/2026/ |

## Doc types

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| T1 | Four types, each answering one need | Opinion, no measured outcome | https://diataxis.fr/ |
| T1 | Each section answers one type's need (the compass) | Opinion | https://diataxis.fr/compass/ |
| T4 | Don't build empty structure first | Opinion | https://diataxis.fr/how-to-use-diataxis/ |
| T2, S9, S36 | Named content types with required parts per type, declared on the page | Doc, Cloudflare | https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/ |
| T1 | About 25 templates, seven core | Doc | https://www.thegooddocsproject.dev/template |
| T1 | Doc types and not mixing them | Doc, Google | https://abseil.io/resources/swe-book/html/ch10.html |
| T3 | Strict splitting caused "too much fragmentation" at pigweed.dev | Opinion, forum post | https://news.ycombinator.com/item?id=42340740 |
| T3 | Diátaxis works better as a checklist than a blueprint | Opinion | https://idratherbewriting.com/blog/what-is-diataxis-documentation-framework |
| T5, T7 | README says what, why, how to start | Doc, GitHub | https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes |
| T6, T11, T13, T14 | Project badge criteria: licence, release notes not raw logs, contribution process, vulnerability reporting | Rule, for badge seekers | https://www.bestpractices.dev/en/criteria/0 |
| T8 | A versioned project declares a public API | Rule, for SemVer users | https://semver.org/ |
| T8 | Every public item documented | Doc, Rust | https://rust-lang.github.io/api-guidelines/documentation.html |
| T9 | Help text leads with examples | Opinion | https://clig.dev/ |
| T10, T11 | Changelog format, dates, group names, for humans | Doc | https://keepachangelog.com/en/1.1.0/ |
| T12 | A breaking change needs steps, not only a changelog line | Opinion | This rule set's own judgment |
| T15, T16 | Decision record parts. Superseded records stay | Doc | https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions |
| T17 | Playbook entry for each alert | Doc | https://sre.google/workbook/on-call/ |

The needs matrix in `doc-types.md` is inference. No source gives a minimum set.

## Structure and page layout

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| S1 | Short description: 50 to 300 characters, says what the page is for | Doc, Red Hat | https://github.com/redhat-documentation/supplementary-style-guide/blob/HEAD/supplementary_style_guide/style_guidelines/structure.adoc |
| S2 | Key message first, no "This page describes" | Doc, Cloudflare and Red Hat | https://developers.cloudflare.com/style-guide/style-and-grammar/voice-and-tone/ |
| S3 | Don't open a page with a notice | Doc, Red Hat | https://github.com/redhat-documentation/supplementary-style-guide/blob/HEAD/supplementary_style_guide/style_guidelines/structure.adoc |
| S5 | Most important first | Doc and Measured attention | https://www.nngroup.com/articles/inverted-pyramid/ |
| S6, S7, S8 | Prerequisites before steps, at most five next steps | Doc, Kubernetes | https://kubernetes.io/docs/contribute/style/page-content-types/ |
| S6 | Prerequisites before procedure | Doc, Red Hat | https://redhat-documentation.github.io/modular-docs/ |
| S9 | Fixed section order on reference pages | Doc, PostgreSQL | https://www.postgresql.org/docs/current/docguide-style.html |
| S10 | Long pages need navigation | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/scannable-content/ |
| S11 | Every page is page one | Opinion, no study | https://mbakeranalecta.github.io/spfe-open-toolkit/spfe-docs-essays/every-page-is-page-one.html |
| S12, S13, S15, S18 | One H1, no skips, sentence case, no links or code, bare verb | Doc, Google | https://developers.google.com/style/headings |
| S14 | No generic headings | Doc, GOV.UK | https://guidance.publishing.service.gov.uk/writing-to-gov-uk-standards/writing-guidelines/clear-structure/ |
| S15 | No end period | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/top-10-tips-style-voice |
| S16, S24 | No stacked headings, parallel structure | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/scannable-content/headings |
| S17 | Repeating "How to" at the start of headings hurts finding | Doc reporting a study | https://github.com/GSA/plainlanguage.gov/blob/HEAD/_pages/guidelines/organize/effective-headings.md |
| S19 | Titles 65 characters or fewer | Doc, GOV.UK | https://guidance.publishing.service.gov.uk/writing-to-gov-uk-standards/writing-guidelines/clear-titles/ |
| S19, S31 | Headings 3 to 11 words, one command per block | Doc, Red Hat | https://redhat-documentation.github.io/supplementary-style-guide/ |
| S20, S21, S22 | No single-item lists, numbers for sequence, lead-in sentence | Doc, Google | https://developers.google.com/style/lists |
| S23 | Lists of two to seven items | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/scannable-content/lists |
| S25 | Tables have two or more columns | Doc, Google | https://developers.google.com/style/tables |
| S25 | No blank cells | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/scannable-content/tables |
| S26 | No adjacent notices, few per page | Doc, Google | https://developers.google.com/style/notices |
| S27, S29, S30 | One action a step, condition and location first, `Optional:` | Doc, Google | https://developers.google.com/style/procedures |
| S28 | Steps start with an imperative | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/procedures-instructions/writing-step-by-step-instructions |
| S32, S33 | Link text makes sense alone. Same text, same target | Doc, Google | https://developers.google.com/style/link-text |
| S34, S35 | Alt text. Don't put text only in images | Doc, Google | https://developers.google.com/style/accessibility |
| S36 | Error entries with code, cause and fix | Observed, Twilio | https://www.twilio.com/docs/api/errors/30007 |
| S36 | Same, Rust | Observed | https://doc.rust-lang.org/error_codes/E0382.html |
| S36, S38, S39 | Error codes documented, `doc_url` links errors to their entry | Observed and Doc, Stripe | https://docs.stripe.com/error-codes |
| S37 | Troubleshooting pages by symptom | Doc, Cloudflare | https://developers.cloudflare.com/style-guide/documentation-content-strategy/content-types/ |

## Wording

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| W2 | One term for one concept | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/global-communications/writing-tips |
| W2 | Consistent terms in skills | Doc, Anthropic | https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices |
| W3 | Define acronyms on first use | Doc, Red Hat | https://github.com/redhat-documentation/vale-at-red-hat/blob/HEAD/.vale/styles/RedHat/Definitions.yml |
| W4 | Limits stated plainly | Observed, SQLite | https://www.sqlite.org/whentouse.html |
| W5, W22 | No marketing language in task pages | Doc, Cloudflare | https://developers.cloudflare.com/style-guide/style-and-grammar/voice-and-tone/ |
| W6 | Active voice by default | Doc, Google | https://developers.google.com/style/voice |
| W6 | Passive voice caused no comprehension cost | Measured, 161, 2019 | https://pmc.ncbi.nlm.nih.gov/articles/PMC6417867 |
| W7 | Second person | Doc, Google | https://developers.google.com/style/person |
| W8 | Present tense | Doc, Google | https://developers.google.com/style/tense |
| W9 | No future features | Doc, Google | https://developers.google.com/style/future |
| W10 | Sentences under 26 words | Doc, Google | https://developers.google.com/style/accessibility |
| W10 | Sentences of 8 to 12 words | Doc, Cloudflare | https://developers.cloudflare.com/style-guide/style-and-grammar/voice-and-tone/ |
| W11 | Paragraph limits | Doc, Digital.gov | https://digital.gov/guides/plain-language/writing/clear-short |
| W11 | Under six sentences | Doc, Kubernetes | https://kubernetes.io/docs/contribute/style/style-guide/ |
| W12, W23 | Keep "that" and articles. No idioms | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/global-communications/writing-tips |
| W13 | Formulas predict adult comprehension worse than NLP models | Measured, 2017 | https://asu.elsevierpure.com/en/publications/predicting-text-comprehension-processing-and-familiarity-in-adult |
| W14, W15, W16, W19 | Word list: "please", Latin abbreviations, wordy forms, minimisers | Doc, Google | https://developers.google.com/style/word-list |
| W17 | No exclamation points | Doc, Google | https://developers.google.com/style/tone |
| W18 | Inclusive terms | Doc, Red Hat | https://github.com/redhat-documentation/vale-at-red-hat/blob/HEAD/.vale/styles/RedHat/ConsciousLanguage.yml |
| W19 | Minimiser words banned and still shipped | Doc and Observed, Vue | https://github.com/vuejs/docs/blob/main/.github/contributing/writing-guide.md |
| W20 | Narrow time-word list. "now" and "new" took 14 hits to 117 | Vendor, 950-file corpus | https://github.com/errata-ai/Google/blob/HEAD/Google/Timeless.yml |
| W20 | "never", "always", "ensure" were 125 of 142 hits | Vendor, 950-file corpus | https://github.com/errata-ai/Google/blob/HEAD/Google/ExcessiveClaims.yml |
| W21 | Remove "you can" | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/grammar/person |
| W23 | Non-native learners want simpler English and no slang | Measured, survey, 840, 2018 | https://pg.ucsd.edu/publications/non-native-english-speakers-learning-programming_CHI-2018.txt |
| W24 | Avoid "above" and "below" | Doc, MDN | https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Writing_style_guide |
| W25 | Code font for code terms | Doc, Google | https://developers.google.com/style/code-in-text |
| W26, W27, W28, W29 | House choices: guides disagree and nothing is measured | Opinion | https://developers.google.com/style/contractions |
| W26 | GOV.UK's position | Opinion | https://guidance.publishing.service.gov.uk/writing-to-gov-uk-standards/style-guides/a-to-z-style-guide/ |
| W29 | Plain language for experts | Doc, qualitative | https://www.nngroup.com/articles/plain-language-experts/ |

## Examples

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| E4 | No foo, bar, baz | Doc, Kubernetes | https://kubernetes.io/docs/contribute/style/style-guide/ |
| E5, E13 | One placeholder format, commands and output apart | Doc, Kubernetes | https://kubernetes.io/docs/contribute/style/style-guide/ |
| E6, E10, E15 | No `...`, a sentence before each block, line width | Doc, Google | https://developers.google.com/style/code-samples |
| E7, E12 | No hard-coded passwords, show expected output, list requirements | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/developer-content/code-examples |
| E8 | Bad code is labelled | Doc, Microsoft | https://learn.microsoft.com/en-us/contribute/content/code-in-docs |
| E9 | Language tags help agents and tools | Opinion | https://docsbydesign.com/2026/02/17/what-makes-api-documentation-ai-ready-code-examples/ |
| E12 | Expected output after each step | Observed, Rust Book | https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html |
| E14 | Prompt style is a house choice | Opinion | https://kubernetes.io/docs/contribute/style/style-guide/ |
| E15 | Code example length | Doc, MDN | https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Writing_style_guide/Code_style_guide |
| E17 | Doc examples run as tests | Doc, Rust | https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html |
| E17 | Same, Go | Doc | https://go.dev/blog/examples |
| E17 | Same, Python | Doc | https://docs.python.org/3/library/doctest.html |
| E17 | Same, AWS | Doc | https://github.com/awsdocs/aws-doc-sdk-examples |

The proof ladder in `examples.md` is inference from those practices.

## Leave out

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| L1, L13 | Include only what the reader can't work out. Each line costs attention | Doc, Anthropic | https://code.claude.com/docs/en/best-practices |
| L4 | History moved to a dedicated place, not removed | Observed, Django | https://docs.djangoproject.com/en/dev/internals/contributing/writing-documentation/ |
| L5 | "Accept (some) Repetition", and "Eliminate content overlap between separate sources" | Doc, Write the Docs | https://www.writethedocs.org/guide/writing/docs-principles/ |
| L6, L8 | Near-miss passages hurt more than random text | Measured, 2024, abstract only | https://arxiv.org/abs/2401.14887 |
| L7 | "Claude is already very smart": leave out what it knows | Doc, Anthropic | https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices |
| L8 | Comprehensive skills -2.9, Detailed +18.8, Compact +17.1 | Measured, no length control, 2026 | https://arxiv.org/html/2602.12670v1 |
| L9 | Leave out who approved what | Opinion | This rule set's own judgment |
| L10 | "beware of overusing Frequently Asked Questions (FAQs) as documentation" | Opinion, Write the Docs | https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/ |

## Accuracy

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| A8, A10 | 28.9% of top projects, 19.2% of docs, 3.9% of references outdated. Mean age 4.7 and 4.2 years. False alarm types: common words, the project name, changelogs, source that is itself text | Measured, 991 projects, 2022 preprint | https://ar5iv.labs.arxiv.org/html/2212.01479 |
| A25, A32 | Resolved by a source change 47.6%, a doc update 39.1%, deleting the doc 13.3% | Measured, 2022 preprint | https://ar5iv.labs.arxiv.org/html/2212.01479 |
| A8, A10 | 23.0% of repos and 1.27% of references stale in agent files. Of 50 sampled stale references, 32 genuine, 12 false, 6 ambiguous. Search done with `git grep` | Measured, 356 repos, one annotator, 2026 preprint | https://arxiv.org/pdf/2606.09090 |
| A10 | Report only when broken now and resolved when the line was written. "Unknown" is never reported. Author's own figures | Vendor, alpha | https://github.com/dager23/docrot |
| A16 | Doc checkers can't detect inaccurate functional descriptions or undocumented items | Doc | https://arxiv.org/html/2307.04291v1 |
| A16 | Up-to-dateness 39% of content issues. A false claim in one project's docs could cause permanent data loss | Measured, 878 artifacts, 2019 | https://emadpres.github.io/pdfs/icse2019.pdf |
| | Incorrectness rare (4 of 61 examples) and judged severe | Measured, self-report, 323 IBM staff, 2015 | https://www.cs.mcgill.ca/~martin/papers/ieeesw2015.pdf |
| | Accuracy left out of quality dimensions because it needs other artifacts to check | Measured, 2020 | https://ar5iv.labs.arxiv.org/html/2007.10744 |
| A1 | Man pages rendered from the command definition | Doc | https://docs.rs/clap_mangen/latest/clap_mangen/ |
| A1 | Module docs generated and placed between markers | Doc | https://terraform-docs.io/ |
| A2 | `cargo rdme --check` exits 3 when the README is out of date | Doc | https://github.com/orium/cargo-rdme |
| A3 | `--verify` compares embedded code with its source | Doc | https://github.com/zakhenry/embedme |
| A3 | `--frozen` fails in CI when output would change | Doc | https://github.com/zimbatm/mdsh |
| A4, A5 | A missing snippet file is silently dropped by default. Named sections exist | Doc | https://facelessuser.github.io/pymdown-extensions/extensions/snippets/ |
| A4, A5 | A missing `literalinclude` file only warns. Offers `:lines:` and `:pyobject:` | Doc | https://www.sphinx-doc.org/en/master/usage/restructuredtext/directives.html |
| A5 | That line ranges rot after an edit is this rule set's own judgment | Opinion | This rule set's own judgment |
| A6, A7 | Offline checks of local files and headings, "less prone to false positives" | Doc | https://github.com/remarkjs/remark-validate-links |
| A6, A7 | `--offline` and `--include-fragments`. Anchors made by scripts are missed | Doc | https://lychee.cli.rs/guides/cli/ |
| A9 | Version templates such as `@VERSION@`. Comparing with a manifest is this rule set's own idea | Opinion | https://scalameta.org/mdoc/ |
| A11, A12 | No tool or study read does exactly this. Their precision is unknown | Opinion | This rule set's own judgment |
| A13 | Rung 2 of the proof ladder. Data only, because fragments rarely parse: 3.89% of Java answers did | Opinion | https://ar5iv.labs.arxiv.org/html/1605.04464 |
| A14 | `broken_intra_doc_links` warns by default | Doc | https://doc.rust-lang.org/rustdoc/lints.html |
| A14 | `errors: [unresolved-doc-reference]` in `dartdoc_options.yaml` | Doc | https://raw.githubusercontent.com/dart-lang/dartdoc/main/README.md |
| A15 | 9.1% of unique links in comments returned 404, and 8.6% returned 403, 405 or 500 | Measured, 382,650 links, 2019 | https://arxiv.org/pdf/1901.07440 |
| A17 | A quoted 91% from a docs-testing case study has no method, which is the failure A17 names | Opinion | https://www.docsastests.com/kong-case-study |
| A18 | Friction log: a scenario, then each step marked good, unclear or blocked | Opinion | https://developerrelations.com/guides/an-introduction-to-friction-logging |
| A18 | Agent installed 21 of 40 repos, 28.8% average, 36.5% of successes with no repair | Measured, 40 repos, 2025 | https://coinse.github.io/publications/pdfs/Milliken2025aa.pdf |
| A19, A22, A24 | Docs change with the code. "Documents without owners become stale." Freshness metadata drives reminders. About 90% of one internal wiki's docs had no views or updates in its last months | Doc | https://abseil.io/resources/swe-book/html/ch10.html |
| A19 | "You can block merging of new features if they don't include documentation" | Doc | https://www.writethedocs.org/guide/docs-as-code/ |
| A19 | Researchers suggest an alert when a file wasn't updated alongside code | Opinion | https://arxiv.org/pdf/2606.09090 |
| A19 | 13 to 20% of code changes triggered a comment update. Read in part | Measured, 1,500 Java systems, 2019 | https://web.archive.org/web/2022/https://www.inf.usi.ch/lanza/Downloads/Wen2019a.pdf |
| A19 | Five update strategies. Most projects required docs by release | Measured, 22 interviews, 2010 | https://www.cs.mcgill.ca/~martin/papers/fse2010.pdf |
| A20, A21 | Docs raised update adoption to 92.87%. Modified APIs hardest. No frontier models | Measured, 270 updates, 11 models, 2026 | https://arxiv.org/html/2604.09515 |
| A20 | Version string alone about +0.5. Version-aware docs retrieval much more | Measured, 8 libraries | https://arxiv.org/html/2412.04478v1 |
| A20 | Version 52.80 against 49.72, imports 65.97 against 44.54 | Measured, 300+ libraries | https://arxiv.org/html/2406.07411v2 |
| A20 | Docs retrieval gained up to about 10 points on breaking changes | Measured, 328 problems | https://arxiv.org/html/2507.12367v2 |
| A20, A21 | Outdated comments pull completions toward older code. Listing only, no numbers | Measured, 2026 | https://conf.researchr.org/details/ase-2026/ase-2026-research-track/29/When-Comments-Mislead-An-Empirical-Study-of-Outdated-Comments-in-LLM-Based-Code-Comp |
| | Prepending docs didn't help older small models | Measured, abstract only | https://arxiv.org/abs/2407.06249 |
| | A docs tool was called in 6 of 89 tasks despite an instruction, and improved none | Measured, single run | https://marginlab.ai/blog/do-mcps-improve-coding-agents-part-1/ |
| A22 | `docs/*` isn't recursive, `/docs/` is. Last match wins | Rule, for GitHub | https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners |
| A23 | Owners, guidelines for updating and removing docs, and docs in the process go with quality | Measured, survey correlation, 2021 | https://dora.dev/publications/pdf/state-of-devops-2021.pdf |
| A25 | "Consider incorrect documentation to be worse than missing documentation" | Doc | https://www.writethedocs.org/guide/writing/docs-principles/ |
| | Wrong context adopted over a correct prior more than 60% of the time | Measured, 1,200+ questions, six models, abstract only | https://arxiv.org/abs/2404.10198 |
| | Models accept coherent evidence that conflicts with their memory | Measured, 2024, abstract only | https://arxiv.org/abs/2305.13300 |
| | 52% of answers wrong, errors missed 39% of the time | Measured, 517 questions, 12 users, 2024 | https://arxiv.org/pdf/2308.02312 |
| A26 | `/doctor prompt-audit` has a model look for missing files, which a command can decide. Running commands first is this rule set's own judgment | Opinion | https://code.claude.com/docs/en/memory |
| A27, A33 | Precision .88 balanced, .39 at 10% inconsistent. Voting .61 to .17. Single prompts .53-.59 to .10-.20. "Is there an inconsistency?" was stricter than "Are they consistent?". Small models | Measured, 71 inconsistent pairs, 2026 | https://arxiv.org/abs/2604.19400 |
| A28 | Findings need a specific wrong word or line. Requiring quotes for both sides is this rule set's own judgment | Opinion | https://arxiv.org/abs/2604.19400 |
| A29 | Docs as falsifiable assertions. No study compares running with reading | Opinion | https://docsastests.com/ |
| A30, A31, A33 | Precision .62 at a 15% flag rate. Most Python false alarms came from missing API knowledge. Under-promising is treated as normal | Measured, 2025 preprint | https://arxiv.org/html/2511.00215v1 |
| A32 | 60 inconsistencies: 33 bugs, 27 bad comments. Abstract plus keyword search | Measured, 2007 | https://www.cs.purdue.edu/homes/lintan/publications/icomment_sosp07.pdf |
| | GPT-4o at about 85% precision near balance. 45.67% of sampled positives in an older benchmark mislabelled | Measured, 600 sampled, 2025 preprint | https://arxiv.org/html/2506.20558v1 |
| | No doc-based validation in 557 sessions | Measured, 2026 | https://arxiv.org/html/2608.20195v1 |
| | "filling your docstrings with obscure test cases makes for bad documentation" | Doc | https://docs.python.org/3/library/doctest.html |
| | Source ranges available from a Rust Markdown parser, for line numbers in audit findings | Doc | https://docs.rs/pulldown-cmark/latest/pulldown_cmark/ |

## Agents

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| G1, G4, G10, G11 | Generated files -0.5 and -2 points, developer files +2.4 (p=.21), cost up 19 to 23%, overviews don't speed file finding, removing a testing section cut cost, no length link | Measured, 300 and 138 tasks, 4 agents, Python, 2026 | https://arxiv.org/html/2602.11988 |
| | AGENTS.md cut median runtime 28.64%, no correctness measure | Measured, 124 PRs, 2026 | https://arxiv.org/html/2601.20404 |
| | No detectable pass-rate change, minimum detectable effect over 30 points | Measured, 288 runs, 2026 | https://arxiv.org/html/2607.27250v1 |
| G17 | 2,303 context files: they accrete, deletions negligible | Measured, descriptive, 2025 | https://arxiv.org/html/2511.12884v1 |
| G21, G23 | Curated skills +16.2 (Fig. 2 says +12.66), SE +4.5, self-generated -1.3, length bands, skill counts | Measured, 7,308 runs, no length control, 2026 | https://arxiv.org/html/2602.12670v1 |
| G22 | Skill built from correct docs gave wrong output 66% of the time | Measured, small, self-run, 2026 | https://dacharycarey.com/2026/05/07/can-agent-skills-make-output-worse/index.md |
| G12, G36 | Explicit default 33% to 100%. Conflicts cut correct decisions to 34.5%. Labels no effect at ceiling | Measured, hobby, 15 runs a condition, 2026 | https://passo.uno/if-you-are-an-agent-read-this/ |
| | 3.2% of events are doc interactions, 60.5% of those agent-facing. Recovery: code 31.0%, docs 5.4%. Repo-local only | Measured, 557 sessions, 2026 | https://arxiv.org/html/2608.20195v1 |
| G24 | No docs 53%, skill 53%, explicit 79%, AGENTS.md index 100%. Skill unused in 56% | Vendor, no N, 2026 | https://vercel.com/blog/agents-md-outperforms-skills-in-our-agent-evals |
| G2, G3, G7, G9, G15, G16 | Concrete instructions, contradictions, concatenation, audit for missing files, under 200 lines, AGENTS.md read only without CLAUDE.md | Doc, Anthropic | https://code.claude.com/docs/en/memory |
| G1, G4, G6 | Each line costs attention, don't describe layout, "adding IMPORTANT probably won't fix it", use hooks, test command is the most useful line | Doc, Anthropic | https://code.claude.com/docs/en/best-practices |
| G12, G13, G18, G19, G20, G22 | Skill limits, third-person descriptions, one level deep, table of contents, no time-sensitive text, one default, evals with a baseline | Doc, Anthropic | https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices |
| G14, G16 | Codex 32 KiB limit, concatenation order, "Leave formatting and lint checks to CI" | Doc, OpenAI | https://learn.chatgpt.com/docs/agent-configuration/agents-md |
| G5, G14 | Rules under 500 lines, don't paste style guides | Doc, Cursor | https://cursor.com/docs/context/rules |
| G16 | Nearest AGENTS.md takes precedence | Doc, GitHub | https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions |
| G14, G16 | No required fields, no length rule, "closest AGENTS.md wins" | Rule, a convention | https://agents.md/ |
| G14 | No length limit stated | Doc, Gemini CLI | https://geminicli.com/docs/cli/gemini-md/ |
| G2 | Commands early, code over prose, boundaries. Survey of 2,500+ files, no outcome measured | Opinion, vendor-authored | https://github.blog/ai-and-ml/github-copilot/how-to-write-a-great-agents-md-lessons-from-over-2500-repositories/ |
| G25 | Tool description edits have large effects | Vendor, 2025 | https://www.anthropic.com/engineering/writing-tools-for-agents |
| G26, G37 | Canary tests of what fetch tools deliver. Agent-directed blockquote refused as injection | Measured, small, 2026 | https://dacharycarey.com/2026/04/06/designing-agent-reading-test/index.md |
| G27, G29 | WebFetch reads up to 100,000 characters, is lossy, prefers Markdown, returns cross-host redirects as text | Doc, Anthropic | https://code.claude.com/docs/en/tools-reference |
| G27 | Accept negotiation on every Claude Code fetch | Measured, 12 sites, 2026 | https://dacharycarey.com/2026/09/23/agent-data-seeking-patterns/index.md |
| G27, G32 | Markdown 10 to 90 times smaller on four pages. `llms-full.txt` 10 to 29 MB | Measured, this research's own requests, 2026-11-06 | Stripe, Cloudflare, Vercel and Claude docs, one page each |
| G28 | Fetch tool limits from 5,000 characters | Measured, small, 2026 | https://agentdocsspec.com/platforms/ |
| G30 | H1 required, blockquote, H2 link lists | Rule, a proposal | https://llmstxt.org/ |
| G31, G35 | 404s 2.23 per task on HTML pages, 0.11 on markdown pages that link the index, accuracy flat | Vendor, 2,400 runs, 2026 | https://www.mintlify.com/blog/llms-txt-agent-benchmark |
| G31 | Agents use `llms.txt` when pointed to it. "this evidence reduces to anecdata" | Measured, six sessions, 2026 | https://dacharycarey.com/2026/09/23/agent-data-seeking-patterns/index.md |
| G35 | 97% of files got zero requests | Measured, tool vendor, 137,210 domains, 2026 | https://ahrefs.com/blog/llmstxt-study/ |
| G35 | Google Search needs no AI files | Doc, Google | https://developers.google.com/search/docs/fundamentals/ai-optimization-guide |
| G33 | Agents guess URLs, soft 404s mislead, same-host redirects work | Opinion, observed, 2026 | https://dacharycarey.com/2026/02/18/agent-friendly-docs/index.md |
| G34 | Tabbed page of 258K characters returned about 3.3%. HTML page's content began 87% of the way in | Measured, one page each, 2026 | https://dacharycarey.com/2026/02/19/agent-web-fetch-spelunking/index.md |
| S11, S35 | Sections stand alone, text not images, one-sentence answer after the H1 | Opinion, still under test | https://docs.n8n.io/contribute/contribution-guide-for-n8n-docs/agent-friendly-docs.md |
| S11 | Terms absent from a chunk mean the chunk isn't retrieved | Vendor, no data | https://docs.kapa.ai/improving/writing-best-practices |
| G38 | Rule-file attacks succeeded 41.1 to 84.1% | Measured, 314 payloads, 2025 | https://arxiv.org/html/2509.22040v2 |
| | Agent scores: 100 and 33 for one site, no link shown to task success | Opinion, author's own statement | https://dacharycarey.com/2026/04/18/what-agent-score-can-tell-you/index.md |
| G39 | `reference/agents.md` loaded as a folder's `AGENTS.md` on macOS while this rule set was written | Opinion, seen once, 2026-11-06 | This rule set's own experience |

## Context length

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| | MRCR 91.9 at 256K to 78.3 at 1M, GraphWalks BFS 41.2 at 1M | Vendor, Opus 4.6 card, 2026 | https://www-cdn.anthropic.com/6a5fa276ac68b9aeb0c8b6af5fa36326e0e166dd.pdf |
| | Focused 300 tokens beat 113,000 for every model tested. The report covers 18, not all in each test | Measured, 2025 | https://www.trychroma.com/research/context-rot |
| | "Lost in the middle" on older models | Measured, 2023 | https://ar5iv.labs.arxiv.org/html/2307.03172 |
| G8 | Accuracy falls with padding. Threshold no longer holds | Measured, 2024 | https://ar5iv.labs.arxiv.org/html/2402.14848 |
| G8 | Spacing between related facts hurts | Measured, 2024 and 2025 | https://arxiv.org/html/2410.14641 |
| G8 | Conditional constraints: whole-instruction success under 30% | Measured, 2025 | https://arxiv.org/html/2505.16944v1 |
| | No aggregate format effect at the frontier | Measured, 11 models, 2026, abstract only | https://arxiv.org/abs/2602.05447 |
| | 500 keyword instructions, best 68%. The task is keyword inclusion, not coding | Measured, 2025 | https://arxiv.org/abs/2507.11538 |
| | Keyword ceiling near 2,000+ on 2026 models | Measured, blog, 2026 | https://arize.com/blog/llm-instruction-following-benchmark-2026/ |
| | Instruction budget of 150 to 200 is claimed, and the source's abstract doesn't say so | Opinion | https://www.humanlayer.dev/blog/writing-a-good-claude-md |

## Visuals

All read during a second pass on 2026-11-06. Many papers were reachable only as abstracts or
excerpts, and the last column of `visuals.md` says where a rule stretches past what was tested.

| Rules | Claim | Strength | Source |
| --- | --- | --- | --- |
| V1 | Reader gets the whole story from either picture or text | Doc, Microsoft | https://learn.microsoft.com/en-us/style-guide/accessibility/alternative-text |
| V1, V5, V6, V7 | Screenshot when a UI element is hard to find, none where text is clear, just enough context | Doc, GitHub | https://docs.github.com/en/contributing/writing-for-github-docs/creating-screenshots |
| V2, V10, V13 | Useful visual explanations, introduce each figure, big picture first | Doc, Google | https://developers.google.com/style/images |
| V3 | 33 s with images against 30 s without, not significant, users preferred images | Measured, 10, 2021 | https://arxiv.org/pdf/2108.03356 |
| V3, V22 | Preference for animation fell from 79.2% to 45.8%. Faster right after, slower a week later | Measured, 72, 2013 | https://www.ijee.ie/articles/Vol29-4/23_ijee2783ns.pdf |
| V4 | Use of colour | Doc, WCAG 1.4.1 | https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html |
| V5 | Errors per step 1.83 to 0.95, time 13.80 s to 9.58 s, all tasks done 37.9% to 66.1%. Learning, help then removed | Measured, randomised, 176, 1995 | https://www.chi1995.chistatic.hosting.acm.org/proceedings/papers/smh_bdy.htm |
| V5, V6, V7 | Captures for locating and checking gained (n=44, n=53). No gain in n=73 and n=42. Wording matched images on accuracy, not speed | Measured, 2002 thesis, paper manuals | https://ris.utwente.nl/ws/portalfiles/portal/6073421/t0000018.pdf |
| V6, V7 | Full-screens manual slowest and least helpful, small button images best. Sample size not found | Measured, 1993, abstract only | https://katalog.lib.cas.cz/KNAV/EdsRecord/edsair,edsair.doi...........3f1110ebba9c033f8488ccf2f7bbf34b |
| V7 | Images in docs | Doc, GitLab | https://docs.gitlab.com/development/documentation/styleguide/images_and_diagrams/ |
| V8 | Red frames: finding about 3 times faster, no difference in correct solutions | Measured, 49, practitioner article | https://technischekommunikation.info/fachartikel/online-information/ist-weniger-immer-mehr-1341/ |
| V8 | Signalled screenshots: longer on relevant areas, more tasks correct | Measured, 2019, excerpts only | https://www.ingentaconnect.com/content/stc/tc/2019/00000066/00000004/art00007 |
| V8 | Contrast of non-text content. Doesn't name annotations | Doc, WCAG 1.4.11 | https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html |
| V9 | 297-day age flagged 70.3% of screenshots, 8 of 154 sampled truly outdated. VS Code only | Measured, 2025 | https://www.cs.sjtu.edu.cn/~zhonghao/paper/DOSUD2025.pdf |
| S35 | Text shown as an image: OCR that lost indentation took HumanEval to 0.00 for three models | Measured, 2026 | https://arxiv.org/html/2603.09095v1 |
| V11 | Title, legend, labelled arrows | Doc, C4, untested | https://c4model.com/diagrams/notation |
| V12 | Everyone improved from text, only trained readers from BPMN | Measured, 196 students, 2012 | https://hreijers.win.tue.nl/H.A.%20Reijers%20Bestanden/JSS.pdf |
| V12 | Same accuracy, more time on graphical models | Measured, 28, 2013 | https://doi.org/10.1109/icpc.2013.6613831 |
| V13 | Good layout +7% correct, experience about twice the effect | Measured, 77 students, 2011 | https://backend.orbit.dtu.dk/ws/files/6282242/PID1910517.pdf |
| V13 | Performance falls with size. 20 to 60 elements isn't a tested limit | Measured, 2014 | https://backend.orbit.dtu.dk/ws/files/120654649/VL_4d_Size.pdf |
| V14 | Tables beat graphs on risk models | Measured, 69 and 83 students | https://iris.unitn.it/bitstream/11572/169302/1/SSRN-id2906745.pdf |
| V15 | Creator maintains the diagram | Doc, GitHub | https://docs.github.com/en/contributing/writing-for-github-docs/creating-diagrams-for-github-docs |
| V15 | Text formats didn't change the tendency to neglect docs. Maintenance measure unverified | Measured, about 13,000 projects, 2025, abstract only | https://conf.researchr.org/details/icse-2025/icse-2025-research-track/110/UML-is-Back-Or-is-it-Investigating-the-Past-Present-and-Future-of-UML-in-Open-Sou |
| V16 | Syntax checks | Doc, PlantUML | https://plantuml.com/command-line |
| V16 | Validate a workspace | Doc, Structurizr | https://docs.structurizr.com/validate |
| V16 | Component diagrams checked against code, dependency rules only | Doc, ArchUnit | https://www.archunit.org/userguide/html/000_Index.html |
| V17, V33 | With source, Claude 3.5 Sonnet 92.64 Mermaid, 74.64 PlantUML, 70.00 image. 2024 models, flowcharts only, set size unconfirmed | Measured, 2024 | https://arxiv.org/pdf/2412.16420 |
| V18 | `accTitle` and `accDescr` | Doc, Mermaid | https://mermaid.js.org/config/accessibility.html |
| V18 | Screen reader announces only that the graphic starts and stops. Open since 2021 | Opinion, bug report | https://github.com/mermaid-js/mermaid/issues/2395 |
| V19 | Same performance from video and text, far more lookups in text | Measured, randomised, 42 students, 2017 | https://ar5iv.labs.arxiv.org/html/1704.00074 |
| V20 | Captions | Doc, WCAG 1.2.2 | https://www.w3.org/WAI/WCAG22/Understanding/captions-prerecorded |
| V20 | Silent recordings | Doc, WCAG 1.2.1 | https://www.w3.org/WAI/WCAG22/Understanding/audio-only-and-video-only-prerecorded |
| V21 | Pause, stop, hide | Doc, WCAG 2.2.2 | https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide |
| V22 | Animation over static g=0.226, 59.3% of 140 comparisons no difference, computing 0.108 not significant | Measured, N=7,036, 2016 | https://archive-ouverte.unige.ch/unige:92234 |
| V22 | Animation best for motions, pictures for arrangements | Measured, 2021 | https://doi.org/10.1007/s11251-021-09541-w |
| V23 | Segmenting helps, with longer learning time. Exact values not retrieved | Measured, 2019 | https://doi.org/10.1007/s10648-018-9456-4 |
| V24, V29 | Signalling retention g=0.53, transfer g=0.33, longer learning time. Blinking g=-0.56. Read through a review | Measured, N=12,201, 2018 | https://www.clearinghouse.edu.tum.de/wp-content/uploads/2018/12/CHU_KR-22_-Schneider_2017_Lernmaterial_Hervorhebungen.pdf |
| V25, V32, V33, V34 | What three converters kept from one test page: alt text and links, diagram source, not edges or embeds | Opinion, this rule set's own test, one run, 2026-11-06 | This rule set's own experience |
| V26 | Videos are harder to edit than text | Doc, Cloudflare | https://developers.cloudflare.com/style-guide/how-we-docs/how-we-video/maintenance/ |
| V27 | Concise +58%, scannable +47%, all three +124%. One site, about 10 per condition, programmers excluded | Measured, 1997 | https://www.nngroup.com/articles/concise-scannable-and-objective-how-to-write-for-the-web/ |
| V28 | Table beat text, d=0.39. Medical data | Measured, preregistered, 2,305, 2020 | https://doi.org/10.1098/rsos.190876 |
| V29 | Unsignalled content recalled worse in a half-signalled text | Measured, 1996 | https://doi.org/10.1037/0022-0663.88.1.38 |
| V30 | Bullets raised recall of items, lowered it for the text around them. Snippet only | Measured | https://benjamins.com/catalog/idj.21.2.06jan |
| V31 | Accordions, no measured data | Opinion, NN/g | https://www.nngroup.com/articles/accordions-on-desktop/ |
| V34 | Mermaid fences kept in `.md` exports, one page each | Opinion, this rule set's own check, 2026-11-06 | https://www.mintlify.com/docs/components/mermaid-diagrams.md |
| V35 | A vendor's `.md` export opened with text asking agents to POST feedback | Opinion, seen on one page, 2026-11-06 | https://www.mintlify.com/docs/components/mermaid-diagrams.md |
| | UML: no overall effect on comprehension, MD 0.01, CI -0.06 to 0.08. Baseline was code with comments removed. Prose and Table 4 disagree on time | Measured, 12 experiments, 333 observations, 2018 | https://alarcos.esi.uclm.es/ALARNET2/FILES/Articulos/2018-Empir%20Software%20Eng-Scanniello.pdf |
| | UML: +54% functional correctness, +14% time not significant | Measured, 20 professionals, 2008, abstract only | https://doi.org/10.1109/tse.2008.15 |
| | UML: correctness rose on the most complex tasks, no saving once updates counted | Measured, 2006, technical report | https://cms.simula.no/sites/default/files/publications/Arisholm.2005.2.pdf |
| | Best model 70.18% on software diagrams, 33 to 40% on awkward arrows. Text and tables disagree on some figures | Measured, 154 diagrams, 2026 | https://arxiv.org/html/2604.04009v1 |
| | Images in issues: small gains, and they hurt where not needed | Measured, about 100 tasks, 2024 | https://arxiv.org/html/2410.03859 |
| | Developers using video to learn: 54.2%, documentation 83.9%. Self-report | Measured, survey, 2024 | https://survey.stackoverflow.co/2024/developer-profile/ |

## Not verified

**The biggest gaps.** No study holds content fixed while varying length. No study holds content
fixed while varying wording. No study compares wrong docs with no docs on current agents. No study
varies UI complexity for screenshots. No study compares diagram source with prose for agents.

**Standards and books not read:**

- ISO 24495-1 and ISO/IEC/IEEE 26514, by their text.
- ASD-STE100, by its text.
- IBM Style. The Apple style guide is mostly unread and the Red Hat guide partly read.
- *Docs for Developers* and *Docs Like Code*: tables of contents at most.

**Read second-hand or in part:**

- Many web pages came through a tool that returns a model-written summary, including the Google and
  GOV.UK style guide quotes and Meng. Recheck exact wording and flag spellings before encoding them.
- Fluri's papers: numbers come through Wen. Zhou 2017: through Wen and a snippet.
- Gloaguen's last 3,000 characters and Gao and Chen's sections 4.3 to 5 are unread.
- Read as abstract only: METAMON, ClashEval, Xie, CodeUpdateArena and several long-context papers.
- Snippet only: Beyond the Prompt, SE Ranking, the Mintlify and Profound work, Mueller, ContextCrush,
  Peec, DOCREF, DocTer.
- Astro's removal of `llms.txt` is known only through Carey.
- Google's position on FAQs and GOV.UK's on the Oxford comma weren't confirmed.
- Vance and colleagues gave no measured evidence on overusing callouts.
- `terraform-docs --output-check` wasn't on the pages read. Anchor slug rules by renderer are from
  memory, so A7 needs them confirmed before it's built.

**Weak or unexplained figures:**

- Anthropic's 200 lines, 500 lines and "up to 30 percent" have no published method.
- Surge's results don't name the models. ReadMe's claims returned 404.
- HumanLayer's "150 to 200 instructions" wasn't found in IFScale. The count of 60,000 `AGENTS.md`
  files is unexplained.
- About 41% real-project precision for Kiecker's method is this rule set's arithmetic from a table
  whose column spacing was lost. Its people filtered false alarms by hand.
- docrot's benchmark is its author's own. Kong's 91% has no method.
- Aghajani's counts for completeness, up-to-dateness and correctness sum to more than the 485 issues
  they're drawn from, so an issue may fall in more than one group.
- Du, SkillsBench, Gloaguen, Ashik and LongPiBench each give inconsistent figures internally, and so
  does Gao, Treude and Zahedi. Check any figure against the source's tables before leaning on it.

**Scope limits:**

- Nearly all coding-agent studies are on Python repos.
- All model-reviewer numbers are on function-level comments, using small or older models.
- No peer-reviewed rate exists here for broken links in READMEs or failing README commands.
- Any claim about a specific agent is a snapshot.
- The praised docs were hand-picked, so what they do is correlation only.

**Visuals:**

- "Meng 2019" is two papers: the GIMP signalling study and the API documentation study.
- The study counts for Schneider 2018 differ between the two secondary accounts read, with the same
  N=12,201.
- Meng 2023 was seen only through a citing paper, and a "HackerNoon 2026" video figure couldn't be
  traced. Neither is used. Benchmark scores from aggregator sites are unverified and left out.
- The converter test was one run of one simplified page. WebFetch and Firecrawl weren't run on it.

**Not cited on purpose,** because they couldn't be traced to a primary source: "40% of time reading
docs" attributed to Lethbridge, Stripe's claimed sample testing and its "documentation is part of
feature done", the "25 percent" attributed to Lazonder through 18F, and secondary long-context
figures for Opus 4.7 and GPT-5.5.
