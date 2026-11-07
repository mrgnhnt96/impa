# Wording

How to write the sentences. Style guides agree on a lot here and have measured almost none of it.
No study isolates page length, contractions, "please", "we", heading case or list length in
developer docs, and none tests how wording rules affect agents. Treat these rules as a house style
that makes a project consistent. Wording checks warn, they never fail a build: Vue and Django ban
minimising words and still ship them.

## What "clear" means

In Uddin and Robillard's study of 323 IBM professionals, ambiguity was the top priority to fix, and
the paper defines it as missing information. Writers tend to treat clarity as a sentence problem. Readers report it as a content
problem: the default, the unit, the limit or the error isn't there.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| W1 | A reader can act on each sentence without guessing. Defaults, units, limits, ranges, side effects and what happens on failure are stated where they apply. | No | Measured |
| W2 | One term for one thing across the project. Terms that the code uses are spelled as the code spells them. | Flag | Doc |
| W3 | An acronym or project term is defined on first use on a page, or linked. | Flag | Doc |
| W4 | Say plainly what a thing can't do. Limits stay in the docs. | No | Observed |
| W5 | Claims about speed, ease or quality are backed by a number or removed. | Flag | Doc |

## Who does what

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| W6 | Name the actor when the reader has to act or has to know who acts: "the server retries", not "it is retried". | Flag | Doc |
| W7 | Address the reader as "you" and write instructions as commands. API facts may be third person. | Flag | Doc |
| W8 | Present tense for how things behave. | Flag | Doc |
| W9 | Don't promise what isn't shipped: no "will support", "planned", "coming". | Flag | Doc |

Active voice is a default here, not a law. Two tests found no comprehension cost from the passive
(Millar and Budgell, 161 readers; Rhodes). W6 keeps what matters: the reader knows who does what.

## Length of sentences and paragraphs

The numbers vary and none has a primary study: 8 to 12 words at Cloudflare, under 26 at Google,
25 at GOV.UK and 18F, 30 and 32 in the Microsoft and Red Hat linters. An often-repeated "14 words"
comprehension figure traces back to a dead link. Use a number to find sentences worth rereading.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| W10 | A sentence over 25 words gets a second look. Check the long tail, not the average. | Flag | Doc |
| W11 | A paragraph over five sentences or about 150 words gets a second look. | Flag | Doc |
| W12 | Don't cut words a reader needs to parse the sentence. Keep "that", "who" and articles. | No | Doc |
| W13 | Never gate on a readability score. It's weak at predicting comprehension and punishes the correct technical terms. | Yes | Measured |

W13 is a rule for the audit: it must not compute one.

## Words to avoid

Keep these lists short and exact. Notes in Vale's Google rules record that adding "now" and "new" to
the time-word list took a 950-file corpus from 14 hits to 117, and that "never", "always" and
"ensure" made up 125 of 142 hits on another rule. Both were noise.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| W14 | No "please" in instructions. | Yes | Doc |
| W15 | No "e.g.", "i.e." or "etc." | Yes | Doc |
| W16 | Replace "utilize", "in order to", "allows you to", "via". | Yes | Doc |
| W17 | No exclamation marks in prose. | Yes | Doc |
| W18 | Use "allowlist", "denylist", "primary", "replica" and the like, and avoid the older terms. | Yes | Doc |
| W19 | No "simple", "simply", "easy", "easily", "just", "quick", "quickly", "obviously" in instructions. It's a claim the reader's experience can contradict. | Flag | Doc |
| W20 | "Currently", "latest", "soon" and "recently" date a page. Leave "now" and "new" alone. | Flag | Vendor |
| W21 | Cut "you can" where the sentence still works. | Flag | Doc |
| W22 | No marketing words in task pages: "powerful", "seamless", "robust", "best-in-class", "cutting-edge", "blazing". | Flag | Doc |
| W23 | No idioms or slang. Non-native readers ask for this, and it helps translation. | Flag | Measured |
| W24 | Don't point with "above" or "below". Link or name the section. | Flag | Doc |
| W25 | Code terms go in code font, with no quote marks around them. | Flag | Doc |

## Pick and hold

These have no winner. A project picks one, writes it down, and the audit checks that the pages
agree.

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| W26 | Contractions: use or don't, the same way on every page. | Flag | Opinion |
| W27 | "We" for the project or company: use or don't. Google and Microsoft disagree. | Flag | Opinion |
| W28 | Oxford comma, capitals after a colon, spacing of dashes: one choice each. | No | Opinion |
| W29 | Tone sits at one level for the project. API reference can be plainer than a tutorial. | No | Opinion |

## Where the sources disagree

- **Contractions:** Google, Microsoft, 18F and Mailchimp use them. Red Hat mostly doesn't. GOV.UK
  allows positive ones only. Google says negative ones help readers spot the negative, GOV.UK the
  opposite. No study.
- **How conversational:** Microsoft says write like you speak, Google a knowledgeable friend, Red
  Hat stays formal. Google also warns that a conversational tone translates badly.
- **Plain language and precision:** advocates call lost precision a myth, and critics say plain
  wording can't fix a hard subject. Kimble's own survey has 43 marks for lost detail. Don't drop a
  needed qualifier to shorten a sentence.
- **Brevity against function words:** "omit needless words" and "keep your articles" pull against
  each other. W12 settles it for the reader who is translating or skimming.
- **Plain language for experts:** NN/g recommends it, with qualitative support only.
