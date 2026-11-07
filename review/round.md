# Review round

Follow this when you wrote or changed the docs in this session. It runs without being asked, at the
end of every round. Don't read `method.md`, `checklist.md` or anything in `reference/` for this: the
reviewer loads those itself, in its own session, so yours stays small.

**Contents:** 1 scope, 2 write the brief, 3 run the review, 4 act on the report, 5 report.

Use one folder per round, `"${TMPDIR:-/tmp}"/impa/<project>/round-<n>/`, with `<n>` one above the
last round there. Use absolute paths in the brief, because the review runs from that folder.

## Before you edit

If the project already has docs, copy them before the first edit of the round. That gives you a
diff even when the docs are new to git or were never impa's:

```sh
mkdir -p <round>/before
cd <project> && tar -cf - <doc paths> | tar -xf - -C <round>/before
```

## 1. Scope

- **The whole doc set** the first time impa writes or rewrites a project's docs.
- **The diff and everything it touches** on every later round. "Everything it touches" is the
  reviewer's job to follow: the pages that link to the changed pages, the pages that name the same
  command, flag or term, and the code the changed lines describe.
- **The whole doc set** again if a round changes something every page rests on: the index or nav,
  the docs plan, a term renamed across pages, or install and quickstart steps.

Copy the docs again before each review pass, into `reviewed-1` and then `reviewed-2`. The first
diff is between `before` and `reviewed-1`, and a whole-doc-set round doesn't need it. `diff` exits 1
when it finds changes, which isn't a failure. `git diff` won't do, because it misses new files:

```sh
mkdir -p <round>/reviewed-1
cd <project> && tar -cf - <doc paths> | tar -xf - -C <round>/reviewed-1
diff -ruN <round>/before <round>/reviewed-1 > <round>/changes-1.diff
```

If the round started from a code change, save that code's diff too and name it under `Code changes`.

## 2. Write the brief

Write `<round>/brief-1.md`. The brief holds facts and never your view. It's the only thing the
reviewer sees from this session, so anything you add to it is what the reviewer will believe. Never
say why you made a choice, what you changed in your own words, what you verified, what to look at
"in particular", or how good you think it is.

```text
Review technical docs by the impa review method. Don't edit any files.

Project: <absolute path>
Docs: <absolute paths of the doc set, or "find them">
Other sources: <absolute paths of dependencies or sibling repos the docs describe, each also passed with --add-dir, or "none">
Captured output: <absolute path to a folder of raw command output, or "none">
Docs plan: <absolute path to .impa/plan.md, or "none">
This round's request, in the user's words: "<prompt>"
Scope: <"the whole doc set" | "the changes in <absolute path to the diff> and everything they touch">
Code changes: <absolute path to a diff of the code these docs follow, or "none">
Still open from the last round: <one line each, "ID · severity · title", with the user's words if kept on purpose, or "none">
Handed to a person: <"none", or each item on its own line below this one, "ID · severity · title · who: what they must do">

Return the method's report. Name any problem outside the scope that you happen to see, but don't go looking for one.
```

The reviewer can't run the project's program, so give it the output instead. Save the raw output of
`--help` and any other safe command the docs describe into `<round>/captured/`, one file per
command, with the command line on the first line and nothing else added. Name that folder under
`Captured output`. Output is a fact, so it's allowed in a brief. A summary of it isn't.

Keep a blank line after the `Handed to a person` block. The CLI reads that block up to the first
blank line, and a line it can't read as an item is lost.

## 3. Run the review

Start it in the background, then carry on with anything else the round needs:

```sh
impa review --brief <round>/brief-1.md --add-dir <project>
```

Add `--add-dir <path>` for each entry under `Other sources`. Each one must be readable, or the
reviewer can't open it. The output lands beside the brief: `review-1.md` and `review-1.json`.

The command prints one line, ending in the verdict, turns, seconds and cost. Then:

- **There's no `review-1.json`, or `error` starts `Error: spawn`:** the review couldn't start. Read
  what the command printed, then use the fallback below.
- **It exits 1, or `ok` is false in `review-1.json`:** the review didn't finish, and any
  `review-1.md` is partial. `error` may say only that `claude` exited, even for a timeout. Try once
  more with a larger `--timeout` (minutes, 20 by default) or a smaller scope. Don't report a review
  that didn't finish.
- **`verdict` is empty:** there's no verdict line. Read what's there and tell the user it can't be
  trusted.
- **`repeated` lists IDs:** the reviewer reported items you handed to a person. Ignore those and say
  so.

If `impa` isn't on the PATH or can't start, start a general-purpose agent, not a fork, with the
same brief. Begin its prompt with:

> Read method.md and checklist.md in ~/.claude/skills/impa/review/ and every file in
> ~/.claude/skills/impa/reference/ except sources.md, then follow them. Don't load the impa-review
> skill, which only starts another review. Don't edit any files.

Save its reply as `<round>/review-1.md`. There's no `review-1.json`, so the cost is unknown. This
path isn't locked: say the review ran through the fallback, and point the user to `INSTALL.md`.

If that can't start either, stop and tell the user the docs haven't been reviewed. Never review in
this session: the rules would load here, and you'd be marking your own work.

## 4. Act on the report

Read only `review-1.md`. The reviewer is a first read, and the verdict is triage, so check each
finding against the code before you touch anything. In the studies behind these rules, four to six
findings in ten held up. A finding that doesn't hold up stays unfixed and goes in the report with
the line of code that shows why.

For a finding that does hold up, go by who can act:

- **Caller:** fix every Blocker and Major. Fix Minors too, unless one would undo what the user asked
  for, then say so. Notes are your choice.
- **Person:** never work around it. Copy each one unchanged into the next brief's `Handed to a
  person` block and into *Waiting on a person* in `.impa/plan.md`, so it isn't reported twice.

Where the doc and the code disagree, the code may be the bug. Never change code to match a doc
unless the user asked. If the evidence doesn't show which side is right, change neither and tell the
user.

Never reverse what the user asked for to satisfy a finding. Report the conflict, and put it under
`Still open` with their words.

Then, by verdict:

- **`Ready to ship`:** fix any Minors you're going to, then go to step 5. No re-check needed.
- **`Fix first`:** make the fixes, copy the docs into `reviewed-2`, write `changes-2.diff` between
  `reviewed-1` and `reviewed-2`, then run a re-check.
- **`Can't ship until a person acts`:** nothing you can do changes the answer. Fix what's left that
  you can, don't start another review, and report it at the top.

A re-check is a new brief, `brief-2.md`, the same as the first one but with this scope, and it
produces `review-2.md`:

```text
Scope: check whether findings <IDs> in <absolute path to review-1.md> are resolved, using the changes in <absolute path to the fix diff>, and everything those changes touch.
```

Stop after two review passes in a round. A finding still open then goes to the user, with what you
tried. Don't start a third.

## 5. Report

Tell the user the verdict, the scope, the claims checked and how many disagree, the findings by
severity, what you fixed, what stays open, who can act on it and the cost. Say which findings you
checked and left because they didn't hold up. Lead with *Can't ship until a person acts* if that's
the verdict.
