---
name: impa-review
description: Reviews technical docs against the code they describe and reports where they disagree, and whether a developer or an agent could act on them. Returns findings only and never fixes anything. Use whenever the user asks to review, audit, check or verify a README, docs site, docs folder, API or CLI reference, guide, changelog or agent instruction file, asks whether docs are accurate, up to date or match the code, or asks what has drifted. The impa skill also starts this automatically after every round. Do not use for marketing pages (use beedle-review), for artifact pages or a claude.ai link (use mudora), for app UI (use ultrahand:review) or for code review.
---

# Impa review

Starts a technical docs review in its own locked session and returns the report. It works on any
docs, with no writer involved. Findings only: it never edits the docs or the code, even if asked
for "review and fix".

Don't read `method.md`, `checklist.md` or anything in `reference/`. The reviewer loads them itself,
and keeping them out of this session is the point.

If you wrote or changed these docs in this session, follow [round.md](round.md) instead.

## Steps

1. Write the brief to `"${TMPDIR:-/tmp}"/impa/<project>/direct-<n>/brief-1.md`, with `<n>` one above
   the last folder there. It holds facts and never your view, so don't read the docs first:

   ```text
   Review technical docs by the impa review method. Don't edit any files.

   Project: <absolute path>
   Docs: <absolute paths the user named, or "find them">
   Other sources: <absolute paths of dependencies or sibling repos the user named, or "none">
   Docs plan: <absolute path to .impa/plan.md if the project has one, or "none">
   This round's request, in the user's words: "<prompt>"
   Scope: <what the user named, or "the whole doc set">
   Still open from the last round: none
   Handed to a person: none

   Return the method's report. Name any problem outside the scope that you happen to see, but don't go looking for one.
   ```

2. Start it in the background. Add `--add-dir` for each path under `Other sources`:

   ```sh
   impa review --brief <brief> --add-dir <project>
   ```

   A large doc set may need `--timeout <minutes>`, which is 20 by default.

3. If the command exits 1, the review didn't finish, and any `review-1.md` is partial. With no
   `review-1.json`, or an `error` that starts `Error: spawn`, it couldn't start: read what the
   command printed, then use the fallback. Otherwise run it once more with a larger `--timeout`. If
   that fails too, tell the user no review finished, with the `error` from `review-1.json`.

4. Otherwise read only `review-1.md`, beside the brief. Return it as written, with the cost from the
   line the command printed, and stop. Don't act on the findings unless the user asks: fixing is a
   separate step, after this one. If `verdict` is empty in `review-1.json`, there's no verdict line,
   so say the report can't be trusted.

If `impa` isn't on the PATH or can't start, use the fallback in section 3 of
[round.md](round.md). If that can't start either, tell the user no review ran. Never review in
this session.
