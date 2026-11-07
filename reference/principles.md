# Principles

What good technical docs are, tested against the guess that they should be "short, direct,
intentional and clear". Read this file first. The other files turn it into rules a reviewer can
check. It leans on measured studies where they exist, and it says so where they don't.

## The verdict on the guess

| Word | Verdict | What the evidence says |
| --- | --- | --- |
| Clear | Holds, on opinion | 88% of 146 practitioners rated clarity important, which is a rating, not a measured effect. Among IBM staff, ambiguity was the top priority, but that study defines it as missing information, so readers often mean "the information is there" more than "the sentence is tidy". |
| Direct | Holds | Answer first, then detail. Web reading studies support it, agents read specific instructions better, and no source read argues the other way. |
| Intentional | Holds as checks | No source uses the word. It becomes: each page has one job, one named reader, and a first sentence that says what the page is for. |
| Short | Holds only as "no wasted words" | Style guides ask for it in sentences, leads, summaries and single examples, and quickstarts were the shortest type in the praised docs measured. It fails as a page-length rule. The research found no study that isolates length in developer docs, and all six blockers in the IBM study were content problems, not bloat. |

The guess leaves out what has the strongest evidence: correct, complete for the task, current, easy
to find, with working examples that are explained.

A better version of the guess, which is this rule set's synthesis and not a finding:

> Correct, current and complete for the reader's task. Easy to find, with working examples. Clear
> and direct. No longer than that requires.

"Intentional" drops out of the wording because no source uses the word. It lives on as checks on each page.

For agents the same shape holds. In SkillsBench, comprehensive skills scored 2.9 points below no
skill at all, while detailed ones gained 18.8 and compact ones 17.1. That is an association: standard
skills gained 10.1, below both, and the research found no study that holds content fixed
while varying length.

## What the evidence is strong on

- **Content problems outweigh style problems.** Across 878 mined doc issues, completeness and
  up-to-dateness led. In a survey of 146 practitioners, 59% rated code and docs disagreeing as
  important.
- **Examples work.** Adding them raised task success in a randomised test with 26 developers, and
  a 2009 survey named examples the largest resource obstacle.
- **Readers scan and hunt.** They rarely read a page through. The figures come from general web
  reading, and small studies of developers show the same habit.
- **Wrong text misleads.** Models adopt plausible wrong context over a correct prior more than 60% of
  the time (read as an abstract), and 12 users missed polished wrong answers 39% of the time. Being wrong costs more than
  being long.

- **Cutting words and signalling structure help readers.** Signalling has the deepest pooled support
  (retention g=0.53, transfer g=0.33), and in the one direct test cutting words beat formatting. Both
  come from general material, not developer docs.

## What the evidence is weak on

- **How long anything should be.** All the numeric limits come from style guides.
- **What failed tutorials cost.** About 65% of 146 practitioners rated faulty tutorials a relevant
  problem, which is opinion, not measured harm.
- **Most wording rules.** Consistency is worth having, but passive voice has no measured
  comprehension cost, and readability formulas predict poorly.
- **Which doc types a project needs.** No study. Diátaxis has no measured outcome.
- **Business outcomes.** DORA's results conflict from year to year. Say "associated with", and
  promise fewer errors rather than faster reading: Meng found higher accuracy with added content, and
  Sohan and Meng found no significant saving in time.
- **Visuals.** Screenshot studies run from benefit to harm, UML shows no general benefit, and video has
  no consistent lead over text. Each helps with specific jobs, and most of the evidence is old or from
  students. See `visuals.md`.
- **How wording affects agents.** The research found no study that holds content fixed and
  changes the wording.

## How to use these rules

**Basis** says how strong a rule is. Strength labels are defined in `sources.md`.

**Command** says who decides. Yes = a command decides. Flag = a command finds candidates and a person
decides. No = a person decides.

**A clean audit proves little.** A command sees a dead link or an example that won't compile, but the worst
problems (a wrong sentence, a missing topic) mostly need a person. Passing `audit.md` means the page
is tidy. Whether it's good takes a review against the No rules.

**Review in this order, which is this rule set's judgment:** correct and current, complete for the task, findable, examples, then wording. Stop and
fix at the first level that fails. Polishing the wording of a wrong page is wasted work.

**Severity follows confidence.** Block only on structure, including its exact lists of banned
headings and link text. Warn on wording, voice, tense, length and tone. A rule that fires on sound
text gets narrowed or dropped.

## Files

| File | Covers |
| --- | --- |
| `doc-types.md` | Which documents exist and which a project needs |
| `structure.md` | Page layout, headings, lists, steps, links, error entries |
| `wording.md` | Sentences, terms, words to avoid, house style |
| `examples.md` | What a code example must do and how far it's proved |
| `visuals.md` | When a screenshot, diagram or video earns its place, and how to fix walls of text |
| `accuracy.md` | Keeping docs true to the code |
| `agent-readers.md` | What agents need that people don't, and the reverse |
| `leave-out.md` | What to cut and what never to cut |
| `audit.md` | Every rule a command can decide or flag, and how |
| `sources.md` | Where each rule comes from and how strong it is |
