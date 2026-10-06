---
name: orbita-sync
description: Bring upstream herdr-gpui (penso/herdr-gpui) updates into the Orbita fork, end to end. Checks what upstream added, rebases the `vjr` branch onto it, resolves conflicts and the breakages git does not flag, runs the gates, installs Orbita.app, and finishes with a report of what came in and what changed. Use whenever the user asks to check for, bring, pull, sync, or update Orbita with herdr-gpui or upstream changes, in any phrasing or language ("veja se o herdr-gpui tem atualizações", "traz os updates pro Orbita", "confira se temos mais atualizações", "sync the fork", "rebase vjr onto upstream"), even when they only ask whether there is anything new.
---

# Syncing Orbita with upstream herdr-gpui

Orbita is a personal fork of `penso/herdr-gpui`. `main` is a fast-forward mirror
of `upstream/main`; the fork's changes live as a stack of commits on `vjr`,
rebased onto `main` at every sync. `scripts/fork/update.sh` does the mechanical
part. This skill is the judgment around it: what to do when the rebase stops,
what breaks without a conflict, and what the user needs to hear at the end.

The user runs this skill to get the whole flow done without being asked at each
step. Decide, act, and report the decisions; stop only where this page says to.

Every path on this page is relative to the repository root.

## Arguments

| Invocation | What it does |
| --- | --- |
| `/orbita-sync` | The full flow through installing Orbita.app, then the report. Stops before pushing and asks. |
| `/orbita-sync push` | The same, and pushes `main` and `vjr` to `origin` once the gates pass. |
| `/orbita-sync check` | Steps 1 and 2 only: reports what is new and what would conflict. Changes nothing. |

A request that only asks whether there is something new is `check`. A request to
bring the updates is the full flow.

## What invoking this authorizes

Running the skill is the user's go-ahead, for this run, for everything the flow
needs on `vjr` and on this machine:

- fetching, fast-forwarding `main`, and rebasing `vjr`;
- resolving conflicts, and dropping or rewording fork commits under the overlap
  rule in step 2;
- `fixup!` commits folded back in with an autosquash rebase (step 4);
- one `chore: update fork sync notes` commit for this skill's own files (step 5);
- running the gates and replacing `/Applications/Orbita.app`.

It is not a go-ahead for: pushing (unless `push` was given), editing the user's
files under `~/.config/herdr/`, restarting the running app, or touching Herdr
itself or any sibling checkout. Report those as things the user can do.

## Step 1: Look before touching anything

```sh
.claude/skills/orbita-sync/scripts/status.sh
```

It fetches and prints, read-only: how far `vjr` is behind; every upstream change
with its size and a count by type; upstream commits that touch the rules, gates,
or build; the config keys and defaults upstream changed; the files a merge would
conflict in, with the commits behind them on both sides; the files both sides
changed that merge cleanly; and a preview of which fork commits would stop the
rebase.

- **Up to date**: say so in one line and stop.
- **Working tree dirty**: stop and list what is uncommitted. `scripts/fork/update.sh`
  refuses a dirty tree, and stashing someone's work to get past that is not
  yours to do. Untracked files do not count for either script.
- Otherwise read `.claude/skills/orbita-sync/references/fork-notes.md` now. It
  holds the decisions earlier syncs already made and the places that break
  without a conflict.

## Step 2: Find the overlaps and the new rules

Conflicts are the easy part. The expensive mistakes come from upstream shipping
something the fork already does its own way, and from upstream changing the
rules the fork's code has to meet.

**Overlaps.** Subjects mislead in both directions, so do not judge by them: read
the body and the relevant diff of every upstream `feat` and `fix` that
`status.sh` lists behind a conflicting file or a file changed on both sides.
Then ask, for each fork commit: did upstream just build this, or part of it?

The standing rule, agreed with the user:

- **Upstream now covers it**: adopt upstream's implementation and drop the
  fork's. A smaller fork conflicts less at every future sync.
- **Upstream covers part of it**: keep only the part upstream lacks.
- **The user's config still depends on the fork feature**: keep it. Read
  `~/.config/herdr/config-gpui.local.toml` and `~/.config/herdr/config.toml` to
  see what is in use; do not guess. `config-gpui.toml` beside them is the
  managed defaults file the app rewrites at startup, so nothing of the user's
  lives there.
- **A real product choice between two designs**: keep both working where that is
  possible, pick the option closest to upstream's default, and put the choice in
  the report as an open decision. Do not stop the flow to ask.

**New rules.** When `status.sh` lists commits touching the rules or gates, read
what changed (`git diff <fork-base> upstream/main -- AGENTS.md justfile`). A new
gate applies to the fork's code the moment the rebase lands: a file-size limit
can fail on a file the fork only added thirty lines to.

**New defaults.** The config section of `status.sh` shows keys that arrive
switched on. Note the ones that will act on what the user already has; they go
in the report.

### The `check` report

For `check`, stop here. The full flow repeats these two steps, so nothing needs
saving. Report in the language of the user's request, with these sections
translated:

```markdown
<One or two sentences: how far behind, whether the tree and origin are in step,
and that nothing was changed.>

## What is new upstream
<N changes, counted by type. Group by what the user would notice. Name the
notable ones; do not list every commit.>

## What would change for you
<Defaults arriving switched on and behavior changes that act on their config,
each with the setting that turns it off. Omit if there are none.>

## Conflicts
<Files and hunk counts in a line or two, then the fork commits that would stop
the rebase and what else would break without a conflict.>

## Overlaps
<One entry per fork feature upstream now touches: keep, drop, or keep part,
with the reason. Mark real product choices as the user's to make.>

## Size of the job
<Stops expected, which ones are heavy, and what needs a look in the app.>
```

## Step 3: Start the sync

Leave a local way back first. The rebase rewrites `vjr`, and a tag makes
"compare with before" and "undo" one command. The name carries the time so a
second run on the same day cannot move the first one's tag:

```sh
git tag "orbita-pre-sync-$(date +%Y%m%d-%H%M)" vjr && git tag -l 'orbita-pre-sync-*' | tail -1
scripts/fork/update.sh
```

Keep the tag name it prints; step 8 needs it.

This skill's own commits sit at the top of the fork's stack, so while the rebase
replays the commits below them, `.claude/skills/orbita-sync/` is missing from the
working tree. Copy the helpers out before they disappear:

```sh
mkdir -p <scratch>/orbita-sync && cp .claude/skills/orbita-sync/scripts/*.py <scratch>/orbita-sync/
```

and run them from there during step 4.

The script fetches again, so `main` may land on a newer commit than step 1 saw.
If it did, run `status.sh --no-fetch` once more so the report describes what was
actually brought in.

With no conflicts the script carries on into the gates, the bundle, and the
install; that takes many minutes, so run it in the background and wait for it to
finish rather than polling. With conflicts it stops and prints how to continue.

## Step 4: Work through the rebase

Each stop is one fork commit that no longer applies. For each one:

1. **Understand both sides before editing.** `git show <fork-sha> -- <file>` is
   what the fork commit meant to do; `git diff <old-base> main -- <file>` is
   what upstream did to the same place. The conflict markers alone hide both.
2. **Keep upstream's structure and re-apply the fork's intent on top of it.**
   When upstream rewrote or moved a function, port the fork's few hooks into the
   new shape and the new file instead of restoring the fork's old version.
3. **Apply the overlap rule.** A commit that only served a feature upstream now
   provides: `git rebase --skip`. A commit that mixes such a feature with
   something still wanted: keep the wanted part and reword the body so the
   history does not describe code that is gone (below).
4. **Type-check before continuing.** Most breakage arrives with no conflict:

   ```sh
   cargo check --locked -p herdr-gpui --all-targets --all-features
   ```

   Fix what the commit being applied caused, in that commit. The fork notes list
   the usual suspects.
5. **Run focused tests when the stop rewrote behavior**, not at every stop:

   ```sh
   cargo test --locked -p herdr-gpui --bin herdr-gpui -- sidebar orbita
   ```

   When an upstream test fails because the fork added a layout or a variant,
   make the fork satisfy the test. Editing upstream's tests to exempt the fork
   grows the diff every future sync has to carry.
6. **Keep the docs honest in the same commit.** The Orbita paragraph of
   `crates/herdr-gpui/README.md` and `config-gpui.example.toml` describe
   behavior; when the port changes behavior, change them.
7. **Continue.** Stage everything first: `--continue` refuses unstaged changes.

   ```sh
   git add -A crates && GIT_EDITOR=true git rebase --continue
   ```

Two helpers take the mechanical part out of a stop. Run the copies made in
step 3:

- `pick.py FILE` lists a file's conflict blocks with their sizes;
  `pick.py FILE o,t,b` resolves them by side. Upstream is `o` (ours) during a
  rebase, the fork commit is `t`.
- `port_hunks.py SHA OLD_PATH NEW_PATH...` replays what fork commit `SHA` did to
  `OLD_PATH` onto the files upstream moved that code to. This is the usual case
  when upstream splits a file or moves a test module out: git reports the whole
  old block as one conflict, so take upstream's side with `pick.py` and replay
  the fork's hunks where the code now lives. Run it with `--dry` first, and
  `--skip` the hunks git already merged, since a hunk that only adds lines
  would land twice.

Put code the fork adds in files the fork owns whenever a seam allows it: a
module beside upstream's, a topic file under the module's `tests/`. Upstream
never edits those, so they cannot conflict, and they keep the fork's lines out
of files upstream holds near the size limit.

A chain of fork commits that build something up and take it down again is not
worth porting step by step through a structure upstream rewrote. Port the end
state, skip the commits whose whole effect a later fork commit removes, reword
the ones that keep only part of what their subject says, and say so in the
report.

To reword a commit while continuing, write the new message to a scratch file and
let git copy it in. The subject is a changelog entry, so keep it a conventional
subject that reads well to a user, and change it only when it became untrue:

```sh
GIT_EDITOR="cp '/absolute/path/to/message.txt'" git rebase --continue
```

A fix that belongs to a fork commit already applied goes in after the rebase
ends, folded into the commit that needed it, so no commit is left broken for
someone bisecting later. The repository's commit hook accepts `fixup!` subjects:

```sh
git commit --fixup=<rebased-sha>
GIT_EDITOR=true git rebase --autosquash main
```

To change only the message of a commit already applied, commit an empty
`amend!` whose body is the full new message, then autosquash the same way:

```sh
git commit --allow-empty --only -F /absolute/path/to/message.txt
```

where the file starts with `amend! <the commit's current subject>`, a blank
line, and then the new subject and body.

Things that went wrong before, so they do not again:

- Never `git stash` mid-rebase. It fails on unmerged paths at best and loses the
  resolution at worst.
- The fork's code answers to `AGENTS.md` as it stands after the rebase, rules
  added in this very range included, and to the user's own instructions. Do not
  rely on the rules as they were when the fork commit was written.
- If the rebase is beyond repair, `git rebase --abort` returns to the starting
  point, and the tag from step 3 is there if `vjr` already moved.

## Step 5: Record what the next sync needs

If this sync taught something the next one needs, a new place that breaks
without a conflict or a new standing decision, update
`.claude/skills/orbita-sync/references/fork-notes.md` and commit it on `vjr`.
Do it now: the gates refuse a dirty tree.

```sh
git add .claude/skills/orbita-sync && git commit -m "chore: update fork sync notes"
```

Keep that file to what still holds, and delete entries upstream made obsolete.

## Step 6: Gates and install

Skip this step if step 3 ran through without stopping: it already did all of it.

```sh
just check-commits main
scripts/fork/update.sh --no-sync
```

The second command runs `just ci`, builds the bundle, and installs it as
`/Applications/Orbita.app`. Run it in the background and wait for it to finish.
Read the result from its output: the test totals, any failure, and the
`Installed ... (<sha> on vjr)` line.

- A failure is yours to fix: fold the fix into the fork commit that caused it
  and rerun. Do not report a sync as done with a red gate.
- If the port touched a `cfg` gate, also run `just lint-linux` and
  `just lint-windows`; a macOS run cannot see those. If it did not, say in the
  report that they were not run.
- The script only notes that Orbita is running. The new build takes effect when
  the user restarts the app; say so rather than restarting it.

## Step 7: Work out what changes for this user

A changelog says what upstream did. The user needs to know what they will see
differently tomorrow, with their own configuration. Start from the new defaults
noted in step 2, read both config files again, and look for upstream behavior
that now acts on settings they already have: a switch that defaults on, a daemon
option the GUI started honoring, a moved control, a changed shortcut. For each,
say what changes and the one setting that restores the previous behavior.

Check that the GUI can still read the user's config, which is the first thing to
rule out if the app comes up with default theme and colors:

```sh
grep 'Could not load GUI config' ~/.local/state/herdr/gpui/logs/herdr-gpui.jsonl | tail -3
```

A recent record there means an invalid `config-gpui.local.toml`, not a bad
sync. Report the line and the fix; edit the file only if the user asks.

## Step 8: Push

Only with `push`, and only after green gates. First make sure the force-push
discards nothing: whatever `origin/vjr` holds must already be part of the branch
as it was before this sync.

```sh
git fetch origin
git merge-base --is-ancestor origin/vjr <tag-from-step-3> && echo safe
git push origin main
git push --force-with-lease origin vjr
```

If the check does not print `safe`, something else pushed to `origin/vjr`; stop
and say so instead of overwriting it. Once the push is done, earlier
`orbita-pre-sync-*` tags protect nothing; delete all but the newest.

Without `push`, end the report with the two push commands and ask.

## The report

For the full flow. Write it in the language of the user's request, for someone
who was not watching. Lead with the state, then what they need to act on. Use
these sections, translated, and leave out any that would be empty:

```markdown
<One or two sentences: where vjr stands, the gate result with the test count,
which commit is installed, and whether anything was pushed.>

## What came from upstream
<N changes. Group by what the user would notice: terminal, sidebar, keyboard,
settings, fixes. Name the notable ones; do not list every commit.>

## What changes for you
<Visible differences with their config, each with how to get the old behavior
back. Say "nothing visible" if that is the case.>

## What changed in the fork
<Conflicts resolved and how, in a line each. Fork commits dropped or reworded,
and why. Decisions taken under the overlap rule.>

## Open items
<What was not verified: the native UI was not looked at, lints not run. Open
product decisions. The push, if it was not done.>
```

Report what actually ran. A headless test run is not a look at the app, so say
that the UI was not checked rather than implying it was.
