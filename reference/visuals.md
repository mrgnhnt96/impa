# Visuals

When a screenshot, diagram or video earns its place, and what to do about walls of text. Text that
is complete comes first, for three separate reasons: screen readers need it, agents mostly get
nothing else, and it outlasts a UI change. Then a visual goes where it does a job text does badly:
finding or checking something on a screen, showing structure on a hard task, or showing change
over time.

The test for any visual: what would a reader or agent lose if it vanished? If nothing, it's upkeep.
If a fact, the text has a hole.

Most of this evidence is old, from students or small, and almost none of it comes from developer
docs. The Inferred column says Yes where a rule stretches its backing past what was tested, usually
from paper manuals or general learning material to developer docs. Those rules are defaults a
reviewer can override with a stated reason. The accessibility rules are the firm ones.

## What the evidence shows for each form

| Form | People | Agents |
| --- | --- | --- |
| Screenshots | Split, from benefit to harm. Gains cluster on finding and checking, and on speed more than accuracy | Alt text and a link, not the picture |
| Diagrams | No general benefit. Correctness rose on hard tasks, and readers untrained in the notation gained less | Source survives as text. A rendered diagram loses its arrows or vanishes |
| Video and animation | No consistent winner against text. Text won for looking things up | A bare link or nothing |
| Text structure | Cutting words and signalling structure have the broadest support, on general material | Plain text survives. Tables, tabs and callouts weren't in the test |

The Agents column comes from this rule set's own test: one page, one run, three converters. No
published study of what fetch tools keep was found.

## Every visual

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V1 | Each step, fact and relationship shown in a visual is also in the text nearby. | No | Doc | No |
| V2 | Each visual has a job a reviewer can name: finding, checking, structure or change. Decoration goes. | No | Doc | No |
| V3 | Reader preference alone doesn't justify a visual. | No | Measured | Yes |
| V4 | Colour is never the only signal in an image or diagram. | No | Doc | No |

S34 covers alt text. S35 covers code, commands and errors, which are always text.

V3 is there because preference and performance came apart: HelpViz's users preferred images and
felt faster, and weren't. Novakovic saw preference for animation fall from 79.2% to 45.8% in a week.

## Screenshots

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V5 | A UI procedure gets a screenshot where the reader must find something hard to locate or confirm what's on screen. | No | Measured | Yes |
| V6 | No screenshot on a step whose text is already clear. | No | Doc | No |
| V7 | Crop to the relevant UI, and keep enough context to place it. | No | Doc | Yes |
| V8 | A screenshot marks its target, and the mark has at least 3:1 contrast. | No | Measured | Yes |
| V9 | Review a screenshot against the running app, not by its age. | No | Measured | Yes |

Complex UI steps are the usual place V5 applies, since that's where "go here, then do this" gets
hard to follow in words. That is plausible, not shown: no study varied how complex the UI was.
What was tested is what the image is for. Captures made to locate something or check state produced
the gains, and the same capture on every step did not. Gellevij drew that across studies, not head
to head.

In one study of 44 people, careful wording matched images on finding things accurately but not on
speed. V8's backing points the same way: a marked target was found about three times faster, with
no more correct solutions. Other studies did cut errors, so speed is the steadiest gain, not the
only one.

## Diagrams

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V10 | A sentence introduces each diagram and says what it shows, including a block of diagram source. | Flag | Doc | No |
| V11 | Each diagram has a title, and its arrows and symbols are labelled or have a legend. | No | Doc | No |
| V12 | Specialist notation has a legend, or plain boxes and arrows replace it for a mixed audience. | No | Measured | Yes |
| V13 | A crowded diagram is split into an overview and smaller views. No element count is enforced. | No | Measured | Yes |
| V14 | Values a reader looks up go in a table, not a diagram. | No | Measured | Yes |
| V15 | A published diagram names an owner or what triggers its update. Being text doesn't pass this. | No | Doc | Yes |
| V16 | Diagram source in a repo parses in CI, and passing isn't taken as proof it's accurate. | Yes | Doc | No |
| V17 | Flowchart logic is available as diagram source or text, not only as an image. | Flag | Measured | Yes |
| V18 | A Mermaid diagram has `accTitle` and `accDescr`, plus a nearby list of its steps or connections. | Flag | Doc | No |

Scanniello pooled 12 UML experiments and found no overall effect on comprehension, against code with
its comments removed, so none of it compares a diagram with good prose. Dzidek's 20 professionals
gained 54% in correctness, from the abstract only. In Arisholm the gain came on the most complex
tasks, and time savings vanished once diagram updates were counted.

Diagrams-as-code staying accurate is a mechanism, not a finding. Nobody has measured it, and V16's
checks are nearly all syntax only.

## Video and animation

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V19 | No fact, command or code exists only in a video. | No | Measured | Yes |
| V20 | Narrated video has captions. A silent recording has a text alternative or is labelled as one for text on the page. | No | Doc | No |
| V21 | Motion that starts on its own and runs over 5 seconds beside other content can be paused. | Flag | Doc | No |
| V22 | Animation appears only where change over time is the point. | No | Measured | Yes |
| V23 | A demo has chapters or per-step clips. | No | Measured | Yes |
| V24 | Emphasis doesn't blink or flash. | No | Measured | Yes |
| V25 | An embedded video has a plain link and a text summary or transcript in the page. | Flag | Opinion | Yes |
| V26 | Video covers flows that stay stable between releases. | No | Opinion | No |

Käfer's 42 software engineering students did equally well from video and text, and looked things up
far more in text. They often started with the video and then moved to the text. A mix may suit a
first run best, though nothing tests that with professional developers.

## Text structure

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V27 | Sentences that add no step, fact or decision are cut before any formatting change. | No | Measured | Yes |
| V28 | Items compared on the same attributes go in a table. | No | Measured | Yes |
| V29 | Headings cover a page's topics evenly, with no main topic left out. | No | Measured | Yes |
| V30 | Bullets hold real list items, and reasoning stays in sentences. | No | Measured | Yes |
| V31 | A required fact isn't only inside a collapsed panel or tab. | Flag | Opinion | Yes |

The first fix for a wall of text is fewer words. In the one direct test, cutting words gained more
than formatting did, on one tourism site with programmers excluded. Signalling structure is the
best-pooled result (retention g=0.53, transfer g=0.33), but it adds reading time, and partial
signalling lowers recall of what's left unsignalled. Tables beat prose in one large trial, on
medical data. Bullets helped recall of list items and hurt recall of the text around them.

S21 and S26 already cover numbered lists and callouts. Neither has a measured basis: no study was
found that tests numbered steps head to head, or callouts, tabs or progressive disclosure in docs.

## Agent readers

| ID | Rule | Command | Basis | Inferred |
| --- | --- | --- | --- | --- |
| V32 | No fact is carried only by inline SVG or `aria-label`. | Flag | Opinion | Yes |
| V33 | A rendered diagram has its source or a text equivalent in the page text. | Flag | Opinion | Yes |
| V34 | Where a Markdown export exists, it keeps diagram source and alt text. | Flag | Opinion | Yes |
| V35 | Generated exports are read for text a vendor added for agents. | Flag | Opinion | No |

For flowcharts, the one direct study favours source: Claude 3.5 Sonnet scored 92.64 on Mermaid, 90.59
on Graphviz and 74.64 on PlantUML, against 70.00 on the image. GPT-4o scored 86.09, 87.12 and 70.35
against 72.00. Those are 2024 models, and the charts were generated from Mermaid, which may favour
it. No study compares diagram source with prose stating the same facts, and none adds a docs diagram
to a coding agent's context and measures the result. On images in issues, the gain was small or
mixed, and images hurt where they weren't needed.

## Where the sources disagree

- **Alt text:** Google and GitLab say it describes context, GitHub says content. Pick one per doc
  set.
- **Diagram format:** Google prefers SVG, GitHub Docs requires PNG. V33 holds either way.
- **Screenshots in general:** Red Hat says avoid them, GitHub allows them where a UI element is hard
  to find, and most guides are conditional. None cites a measurement in the pages read.
- **Full or cropped:** the studies conflict. V7 follows GitHub and GitLab.
- **Annotation and accuracy:** Meng found more tasks correct, Bsdurrek and Meng found no difference.
  Both agree it directs attention.
- **Captions:** Mayer's redundancy principle says not to repeat narration, WCAG requires captions.
  Closed captions a viewer can turn on may satisfy both, which is inference.
- **Image against text for models:** TextFlow favours source, and a knowledge-graph study points the
  other way with different model variants and flat triples rather than diagram source.

## What the evidence couldn't settle

- Whether screenshots help more on complex UI. No study varied it.
- Screenshots against text on cloud consoles or with professional developers.
- Whether readers mistake an old image for their own screen.
- A docs diagram against equivalent prose, for professional developers.
- Whether diagrams go stale faster than text, or Mermaid faster than exported images.
- Comprehension of C4, Mermaid, PlantUML, Graphviz or D2 output.
- Real use of video docs, GIFs against long video, and the cost of re-recording.
- Any text-structure technique tested in developer docs.
- Mermaid source against prose, for agents.
- Results on late-2026 models for software diagrams.
