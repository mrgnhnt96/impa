# What to leave out

What to cut, and what to keep however long the page gets. The evidence says the danger runs both
ways. Readers complain about superfluous content (55% of practitioners in Aghajani's survey) and 43
to 51% of reference units carried little value in Maalej and Robillard's sample. But none of the six
blockers in the IBM study was bloat. All six were content problems, and the bloat that study
describes is low signal, not high word count. A cut is a risk to take on purpose, not a free
improvement.

## Keep, however long the page gets

These are what readers report missing, so they don't go for the sake of length.

- Prerequisites and versions.
- Defaults, units, limits and side effects.
- What failure looks like and how to recover. Lazonder found error information improves learning.
- A working example.
- The reason for anything surprising.
- What the thing can't do.

## Cut

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| L1 | Each section earns its place: a reviewer can name the reader and the task it serves. | No | Doc |
| L2 | Reference text adds something the name and type don't. "`name`: the name" fails. | Flag | Measured |
| L3 | No preamble before the first useful sentence: no welcome, no history of the project, no "thanks for choosing". | Flag | Measured |
| L4 | History leaves the main text: "previously", "used to", "in older versions" move to the changelog, unless the reader needs them to migrate. | Flag | Observed |
| L5 | A fact or procedure lives in one place and other pages link it. Copied paragraphs fail, except a warning repeated at its point of use. | Flag | Doc |
| L6 | A removed or old form doesn't sit beside the current one unlabelled. Label it removed, give the replacement, or delete it. | Flag | Measured |
| L7 | Don't teach what the reader's tools already teach: what JSON is, how to install Git. Link it. | No | Doc |
| L8 | Task pages don't cover every edge case. They cover the common path and link to reference for the rest. | No | Measured |
| L9 | No meeting notes, sign-offs, author bios, or who approved what. | No | Opinion |
| L10 | No FAQ made of questions nobody asked. Put each answer on the page where it's needed. | Flag | Opinion |

## How to review for length

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| L11 | Never mark a page down for word count alone. No study supports a length limit for developer docs. | Yes | Measured |
| L12 | Before cutting, check the text against the keep list. A cut that removes one of those items is a defect. | No | Measured |
| L13 | Judge each line by whether it changes what the reader does. | No | Doc |

L11 is a rule for the audit: it may report a page's size but must never fail on it. The one
exception is a file an agent always loads (`agent-readers.md`), where length is a cost on every
task.

## Page sizes seen in praised docs

For calibration only. These come from 14 hand-picked doc sets, so they show what good docs happen to
be, not what makes them good. Medians, in words, rounded:

| Page type | Median words | Note |
| --- | --- | --- |
| Quickstart | 1,370 | The tightest type |
| Concept | 4,000 | |
| Reference | 4,800 | Ranged from 530 to 42,668, set by how the site splits pages |
| Guide | 7,100 | |

A quickstart far above that is worth a look, because a first run should be quick. Nothing here
justifies a limit on reference.

## Where the sources disagree

- **Minimalism:** van der Meij's minimal manual beat a conventional one, and its authors say the
  method is "more subtle than weeding out the excess". The test bundled several changes, so it
  doesn't show that cutting did the work.
- **Rewrites that cut:** NN/g rewrote two Sun whitepapers at about 54% of their length and measured
  a 159% gain overall with 21 technical users. Structure changed along with length, so length can't
  be credited alone.
- **Completeness:** readers ask for it most, yet Robillard found that documenting every member can
  backfire by burying what matters. L2 and L8 draw the line.
- **Repetition:** see `structure.md`.
- **FAQs:** Write the Docs warns against "overusing Frequently Asked Questions (FAQs) as
  documentation". Google's position wasn't confirmed. L10 is opinion.
