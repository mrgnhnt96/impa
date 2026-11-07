# Structure

How to lay out a page so a reader can find the part they came for. Evidence that readers scan and
look at the top of a page is strong, and most of it comes from general web studies, not developer
docs. Almost every numeric limit here is one guide's choice with no study behind it, so numbers are
flags for a reviewer, never failures on their own.

## How readers move through a page

- Developers read opportunistically: they jump in, try code and read only when stuck. Some read
  systematically first. Serve both by putting the answer first and keeping the detail further down.
- About 79% of web users scan, and at most about 28% of a page's words get read. Most viewing lands
  in the first two screens.
- People judge a link or heading on roughly its first 11 characters.
- 78% of 252 developer web sessions in one log study had the traits of a reminder, not of learning.
  Those readers want the fact and the line of code, fast.
- Guide pages are worth pointing to. Visitors to them had 3.92 times the odds of making API calls in
  one large study of cloud services. That's a correlation, not proof of cause.

Findability is its own problem. Half of the usability issues mined by Aghajani were about finding
things, and 34% of developers in Postman's survey couldn't find existing APIs.

## Page

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S1 | The first text after the title says what the page does, in one sentence. | Flag | Doc |
| S2 | That first sentence isn't a self-reference: no "This page describes", "This topic explains", "In this guide". | Yes | Doc |
| S3 | No notice, badge wall or image comes before that first sentence. | Yes | Doc |
| S4 | The page names who it's for or what they should already know, when that isn't obvious from its type. | No | Measured |
| S5 | The most important information comes first: the answer before the background, the common case before the edge case. | No | Measured |
| S6 | Prerequisites come before the first step on tutorial, quickstart and how-to pages. | Yes | Doc |
| S7 | A tutorial or how-to ends by saying how to tell it worked. | Flag | Doc |
| S8 | Next steps, when present, sit at the end and hold at most five links. | Yes | Doc |
| S9 | Pages of the same kind in a project keep the same section order. Reference pages always do. | Yes | Doc |
| S10 | A page with more than about eight sections has a table of contents or the site generates one. | Flag | Doc |
| S11 | A reader landing from search can act: the page links the ideas it assumes and repeats what would cause an error if missed. | No | Measured |

## Headings

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S12 | One H1, no skipped levels, no empty headings. | Yes | Doc |
| S13 | Headings use the project's one case style. Sentence case is the default. | Yes | Doc |
| S14 | A heading says what the section covers when read alone. No "Introduction", "Overview", "Details", "Misc", "Other", "General", "Background". | Yes | Doc |
| S15 | A heading has no end period and no link. Code in a heading is only for reference pages that name the item. | Yes | Doc |
| S16 | No heading follows another with no text between them. | Yes | Doc |
| S17 | Sibling headings don't start with the same words ("How to…" four times). Put the distinct word first. | Flag | Measured |
| S18 | Task headings use the project's one form, bare verb by default ("Create a key"). | Flag | Doc |
| S19 | The page title is 65 characters or fewer, and headings are 3 to 11 words, outside reference pages. | Flag | Doc |

## Lists, tables and notices

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S20 | No list with one item. | Yes | Doc |
| S21 | Numbered lists are for sequences, bullets for everything else. | No | Doc |
| S22 | A list has a lead-in sentence, except under a "Prerequisites" or "Steps" heading. | Flag | Doc |
| S23 | Lists over seven items are worth a second look: group them or use a table. | Flag | Doc |
| S24 | List items are parallel: all fragments or all sentences, all starting the same way. | Flag | Doc |
| S25 | A table has at least two columns, a header row and no blank cells. | Yes | Doc |
| S26 | No two notices in a row, and at most one in any section. | Yes | Doc |

Sources disagree on what a table's lead-in ends with, on capitalising list items and on single-step
procedures. Pick one and be consistent. None of it has evidence.

## Steps

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S27 | One action per step. Steps joined by "and then" get split. | Flag | Doc |
| S28 | A step starts with an imperative verb, or with its condition or location first. | Flag | Doc |
| S29 | A condition comes before the instruction: "If it fails, run…", not "Run… if it fails". | Flag | Doc |
| S30 | An optional step starts with `Optional:`. | Yes | Doc |
| S31 | A step has at most one command per code block. | Flag | Doc |

## Links and images

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S32 | Link text makes sense alone. No "click here", "here", "this", "read more", "more", or a raw URL in prose. | Yes | Doc |
| S33 | One link text never points to two different targets on a page. | Yes | Doc |
| S34 | Every image has alt text. | Yes | Doc |
| S35 | No text that matters exists only in an image: commands, error messages and code are text. | Flag | Doc |

## Errors and troubleshooting

Error entries are the closest thing to a universal pattern among the praised docs: Twilio, Rust,
Stripe and MDN all do them. PostgreSQL's plain list of codes is the counter-example. An exact error
string is also what both a person and an agent will search for.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| S36 | Each error entry gives the exact code or message, the cause and the fix. | Yes | Observed |
| S37 | A troubleshooting heading is the symptom the reader sees, not the cause. | Flag | Doc |
| S38 | Every error code the program can return has an entry. Treat those codes as part of the public contract. | Flag | Observed |
| S39 | Error messages link to their doc entry where the product can do that. | Flag | Doc |

## Where the sources disagree

- **Question headings:** GOV.UK bans them. An older US plain-language study favoured them over noun
  headings. Microsoft allows them. Leave them out of the checks.
- **Task heading form:** Google uses a bare verb, Microsoft "To create", Red Hat's guides split
  between imperative and gerund. S18 asks for one form, not a winner.
- **F-shaped reading:** Microsoft treats it as a layout goal, NN/g as a sign the page failed. The
  safe reading for both is to put the words that matter first in the line.
- **Repeating information:** GOV.UK says not to repeat the summary, and Meng found developers miss
  what isn't repeated at the point of use. Repeat what would cause an error if missed, link the
  rest.
- **Stand-alone pages:** "every page is page one" is opinion with no study. S11 keeps the useful
  part: a reader from search has to be able to act.
