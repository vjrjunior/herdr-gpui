# Fork notes for the next sync

What earlier syncs decided and where they got hurt. Upstream moves code between
files often, so entries name symbols, not paths: find them with a search.

## Standing decisions

| Area | Decision | Since |
| --- | --- | --- |
| Ahead and behind counts | Upstream draws them (`Upstream` in the sidebar row). The fork's own counts were dropped; the Orbita layout passes `row.upstream()` like Herdr's layout. | 2026-10-03 |
| `[daemon_keybindings]` | Kept. The user's `config-gpui.local.toml` binds `"prefix+m" = "cmd-shift-m"`. Upstream runs `[[keys.command]]` chords itself, so the table only adds a native shortcut beside the chord. Its bindings come from the local GUI config even while a device uses its server's keybindings. A keystroke `[pane_keys]` holds is refused, like one a native shortcut holds. | 2026-10-05 |
| Plugin token badges | Open. The Orbita layout follows rows configured in the daemon's `[ui.sidebar.spaces]` like every layout, and draws its own badge line only on native rows (no configured rows, or `[usage] inline = false`). Whether the badge line stays is the user's call. | 2026-10-03 |
| Uncommitted mark on rows | Orbita rows draw none: the Orbita layout takes the mark off the badge it hands the shared row (`RowBadge::without_dirty`). Upstream's note mark stays on the badge. The title bar marks the focused checkout with a dot on its branch icon. | 2026-10-08 |
| Orbita style | Upstream replaced the style traits with one `SidebarMetrics` value. The Orbita look is the rounded comfortable preset run through `SidebarMetrics::orbita`, which sets one `orbita` flag; everything the flag means is a `SidebarLook` method in the Orbita layout's file. Its child indent carries the guide's tick, so upstream code that reads `child_indent` lines up with Orbita rows. | 2026-10-08 |
| Rows under a host header | Upstream steps rows in under their host (`RowContext::nest`). An Orbita highlight starts at the row's whole indent, so with several hosts it steps in with the row; Herdr's layouts keep full-width highlights. Open for the user if they list several hosts. | 2026-10-08 |
| Fold chevron icon | Upstream ships `icons/chevron-right.svg` and registers it. The fork's copy and its registry entry were dropped. | 2026-10-08 |
| Bundle signing | `scripts/fork/update.sh` signs ad hoc, without the hardened runtime, so upstream's `Herdr.entitlements` is not applied: the `NSMicrophoneUsageDescription` key in upstream's `Info.plist` is what lets a pane ask for the microphone. If the script ever signs with `--options runtime`, pass the entitlements file too. | 2026-10-07 |
| Sidebar search field | Upstream made the field the Spaces heading (`search_heading`, on with `show_sidebar_search`). The Orbita layout keeps it and hangs its plus and hamburger at the heading's right, beside the field; with search off they sit on the plain header. The fork's header test measures the buttons' right margin against the field's left inset. Open for the user: whether Orbita should hide the field. | 2026-10-14 |
| Daemon binding errors | The `[daemon_keybindings]` and unbound-command errors are a fork-owned `DaemonBindingError`, which the crate `Error` wraps in one transparent `DaemonBinding` variant, as upstream does for `coder` and `code_server`. New fork errors go in that enum. | 2026-10-10 |
| Window backdrop | Upstream's `[backdrop]` paints every panel with its own color at the backdrop's opacity. The fork's `chrome` override replaces `theme.surface`, so it gets the same treatment with no code of its own. `titlebar::render` keeps upstream's signature; the tinted or flat choice is the `style` argument of `titlebar::bar`. | 2026-10-11 |
| Shortcut editor | Upstream's Herdr > Keyboard Shortcuts writes `[keybindings]`. It validates a draft through `Keymap::with_edited_overrides`, which carries the loaded `[daemon_keybindings]`, so it refuses a key the table holds instead of saving a config that no longer loads. Any new upstream surface that writes keybindings needs the same call. | 2026-10-11 |
| "Herdr UI" in messages | Upstream names the client "Herdr UI" in errors, notes, and dialogs. The fork's build-set name covers the window title, process name, application menu, and About box only, so Orbita's messages say "Herdr UI". Open for the user. | 2026-10-11 |
| Terminal-only font keys | Upstream's `[terminal]`-only keys (`line_height`, `weight`) are refused for `[sidebar_worktrees]` as for `[sidebar]`, `[tabs]`, and `[ui]`: the fork's face is in the list `Config` runs through `reject_terminal_keys`, so the key is reported in `unknown_keys` and ignored. A terminal-only key upstream adds to that function needs no fork code. | 2026-10-13 |
| Repository and device icons | Upstream's `show_repository_icons` and `show_device_icons` apply to the Orbita layout with no fork code: the layout hands the shared row the icon its context carries, and `LayoutMode::device_icons` leaves out only Orca and Minimal. | 2026-10-14 |
| Device themes | `[theme_overrides]` apply to the app theme only: `Config::theme_with_directories` runs upstream's shared `resolve` and then the overrides, for every theme source, Follow Herdr included. A device's `theme` (`theme_named`) paints only the terminal and takes none. | 2026-10-14 |
| Blocked bar and Edge rail | Upstream's `status_icons = "rail"` draws a bar on the leading edge of working, done, and blocked rows in every layout (`Indicators::rail`, added once in `Cell::row`). The Orbita layout's own blocked bar was dropped for it: the user's `config-gpui.local.toml` sets `status_icons = "rail"`. | 2026-10-15 |
| Tab strip height | The fork's extra 6px was dropped. Upstream's tab row is the title bar now and is at least 34px tall, more than the fork's strip was. Open for the user: whether they want it taller still. | 2026-10-05 |
| Trunk under a note | Upstream carries a worktree child's tree guide on past its note and ports lines (`RowLayout::trunk_below`), and a test asks it of every layout that draws a tree. The Orbita layout returns upstream's `row::trunk_below`, which reads `tree_padding` and `tree_color` from the look like the row's own guide, so the trunk matches the Orbita guide's width and color. | 2026-10-16 |
| Worktree font in Settings | Upstream's rebuilt Settings window lists four faces (`FACES`), and the fork's `SidebarWorktrees` is not one of them, as it was not before: `[sidebar_worktrees]` is set in the config file only. The Fonts page's `size_control` and `specimen_title` still match on every `FontFace`, so they carry arms for it. | 2026-10-16 |

## Where the fork's code lives

Fork code sits in files the fork owns wherever possible. Upstream does not edit
these, so they cannot conflict:

- `config/theme_overrides.rs`: the `[theme_overrides]` types, re-exported from
  the config module.
- `palette/daemon_binding.rs`: running a daemon command by its binding label.
- `keymap/daemon_bindings.rs`: `DaemonBindingError`, the keystroke parser the
  `Keymap` calls for `[daemon_keybindings]`, and `with_edited_overrides`.
- `popup_chrome.rs`: the popup panel, with `PopupLook` as the one-line entry
  point the window render calls.
- `sidebar/layouts/orbita.rs` and `sidebar/orbita_tests.rs`. The layout file
  also holds the Orbita preset and the look methods the shared row and the
  sidebar render call: `mark_nested`, `tree_color`,
  `tree_padding`, `header_actions`, `primary_chip`, `counts_x`.
- Topic test files: `config/tests/{theme_overrides,sidebar_worktrees,daemon_keybindings}.rs`,
  `keymap/tests/daemon_bindings.rs`, `titlebar/orbita_git_button_tests.rs`,
  `menu/shortcut_editor/tests/daemon_bindings.rs`.

What still has to live in upstream's files: the `Orbita` and `SidebarWorktrees`
variants, the `orbita` flag on `SidebarMetrics` with its arm in
`SidebarMetrics::for_mode`, `corners::ROW`, the `marked` line in
`SidebarLook::highlight`, the one-line hooks in the shared sidebar `row` and
`row::trunk_below`, the
`Theme` fields and accessors, the `Keymap` daemon bindings, the validation
call in the shortcut editor, the `sidebar_worktrees` entry in the list of
faces that take no terminal-only keys, `menu.svg` in the sorted
`embedded_icons!` table with the `branch` and `branch_dot` helpers after it,
the `style` argument of `titlebar::bar`, and one-line hooks in the window
render.

## Breaks without a conflict

After a stop, these fail to compile, to pass tests, or to pass a gate although
git merged them cleanly. The type-check finds the first four; only the tests and
`just ci` find the rest.

- **New callers of functions the fork changed.** `titlebar::bar` takes the
  title bar style in the fork, and the shared sidebar `row` takes the fork's
  `tokens` list. Every window or layout upstream adds calls the old shape.
  `SidebarLook::highlight` and `SidebarLook::mark` keep upstream's signatures:
  the Orbita highlight is nested by `mark_nested`, so do not add parameters
  to them again.
- **New constructions of structs the fork extended.** `RowContext` has a
  `worktree_font`, and `Theme` has `accent` and `chrome`. Upstream's new tests,
  previews, and row builders (`append_agent_rows`, `append_device_agents`)
  build them without those.
- **New fields on structs the fork builds.** The Orbita layout rebuilds a
  `RowContext` for worktree rows and `RowBadge` through `RowBadge::new`, so a
  field upstream adds (`nest`, `mark`, `noted`) has to be forwarded there.
- **The same entry on both sides.** When upstream registers something the
  fork also registers, such as an icon path, git keeps both match arms without
  a conflict. Drop the fork's.
- **Upstream changing a type the fork's tests build.** `pull_request::Input`'s
  `repo_key` became an `Option`, and the fork's fixtures stopped compiling in
  test targets only. One fixture can be extended by several fork commits, so
  `git blame` each failing line after the rebase and fix up each owner.
- **New exhaustive matches.** `LayoutMode::Orbita` and
  `FontFace::SidebarWorktrees` are fork variants; upstream's new `match`es do
  not list them. The Settings pages are the usual source: the Fonts page's
  control ids and specimen titles broke on 2026-10-16.
- **Positional patterns over an array the fork extended.** `resolve_fonts`
  lists the font faces in an array and upstream picks the terminal face from
  it with a pattern such as `[_, _, terminal, _]`. The fork's
  `sidebar_worktrees` entry stays last in that array, so upstream's positions
  keep naming its own faces and a new pattern only needs one more `_` at the
  end. In the middle, a pattern that still compiled would pick another face.
- **Upstream replacing an API the fork calls.** A fork function that used
  `status_style` broke when upstream moved status colors into `Indicators`;
  the fork's `FontConfig::apply` became redundant when upstream added
  `FontSettings::apply`. Follow upstream's replacement and drop the fork's copy.
  `CommandInfo` lost its `shortcuts` field to per-platform profiles
  (`default_shortcuts()`), which the daemon binding check reads; a fork test
  that names a native default wraps it in `platform_default`, since Linux
  binds other keys.
- **Upstream adding a parameter to a function the fork calls.**
  `HerdrWindow::reconnect` took a `cx`, which broke the fork's window tests
  (`orbita_tests`, the daemon binding test) in test targets only, and
  `Target::capture` took the pane's selection, which the daemon binding runner
  fills with `plugin_selection()` like upstream's `invoke_custom_command`.
- **Visibility after a move.** When upstream moves a type into a submodule, a
  fork method added with `pub(super)` stops being reachable. `RowBadge` methods
  need `pub(in crate::sidebar)`.
- **Upstream tests over `LayoutMode::ALL`.** They assert a contract for every
  layout, Orbita included. Make the Orbita layout honor the contract.
- **The 1,000-line limit.** `just check-file-size` is part of `just ci`, and
  neither the rebase nor the type-check runs it. Upstream keeps some files a
  few lines under the limit, and its own growth can push one over with lines
  the fork added long ago: the crate `Error` file did on 2026-10-10. Run
  `bash scripts/check-file-size.sh` right after the last stop, and move the
  fork's lines into a fork-owned module in the commit that added them.
  The crate `Error` file stood at 990 lines after the 2026-10-16 sync, two of
  them the fork's transparent `DaemonBinding` variant.
- **Gates upstream adds.** `just lint` also runs clippy with
  `--no-default-features`, and `just ci` runs `test-mockup-wait`, which needs
  `python3`; the usage tests need `jq`. `just check-sorted` refuses a
  `keep-sorted` region out of order, such as the `embedded_icons!` table: a
  fork entry in one goes in its sorted place, not at the end.
- **Upstream tests that read this machine's config.** The device theme
  picker tests list themes from the real config root, and
  `~/.config/ghostty/themes` here holds themes CI does not have, so two of
  them fail on this machine and pass with an empty `XDG_CONFIG_HOME`. Run the
  gates as CI does: `XDG_CONFIG_HOME=<empty dir> scripts/fork/update.sh --no-sync`.
- **Upstream's process-timing tests.** On this machine
  `timeout_kills_shell_and_descendant_group` (`login_env`) fails in the first
  runs after a rebuild and passes alone: it gives a fresh shell script one
  second to start, which is lost while `syspolicyd` assesses the new
  executables. On 2026-10-15 it failed in a warm-up `cargo test` and the next
  gate, the brew `relaunch_retries_then_alerts_when_open_keeps_failing` test
  timed out beside it in the gate after that, and the third gate passed. The
  fork touches neither file. A warm-up run does not reliably help; rerun the
  gate until `syspolicyd` settles, and report the failures as upstream's
  instead of editing them.
- **Test placement.** Test bodies live in `tests.rs` or `tests/` topic files,
  never inline in a production file. A fork test written inline has to move.
- **Fixtures other fork commits set up.** Dropping a fork commit can drop a line
  a later fork test relied on, and the test then passes for the wrong reason or
  fails. Check what the dropped commit added to shared test helpers.

## Recurring conflicts

- The Orbita paragraph of the crate README is edited by most fork commits, so
  it conflicts at nearly every stop. Keep the sentence about configured rows
  next to the badge description it qualifies.
- `Keymap` and the config `Settings` struct gain fields on both sides. Keep
  both; `with_overrides` must stay available outside tests because upstream
  builds server keymaps with it, and it forwards to `with_daemon_bindings`.
- Sorted shared structs. Since 2026-10-16 `Config`, its `Default`, the
  private `Settings`, and the crate `Error` enum sit in `keep-sorted` regions,
  so every fork field conflicts as a block that also holds upstream's
  reordered lines. Take upstream's side and put the fork's lines back in
  their sorted place: `daemon_keybindings` before `daytona`,
  `sidebar_worktrees` after `sidebar_style`, `theme_overrides` after
  `theme`, and `DaemonBinding` before `DaemonCancelled`. Then run
  `python3 scripts/check-sorted.py`.
- Fork tests that asserted an unknown config key is rejected fail now: upstream
  ignores unknown keys and reports them in `unknown_keys`.
- When upstream splits a file the fork also edits, or moves its test module
  out, git reports the whole old block as the conflict. Take upstream's side
  and replay the fork's hunks with `port_hunks.py`.
- The Spaces section of `render_sidebar`: the fork hangs `header_actions` on
  whatever upstream's heading chain returns (the search field or the plain
  header) and wraps the footer in `when(!header_actions)`, so an upstream
  edit to what sits between them conflicts. Keep upstream's body, such as the
  wrapper that clips the pinned host header, between the fork's two hooks.
- The test module list in `config/tests.rs` and the `mod` lists in `main.rs`
  and the palette file conflict whenever upstream adds an entry next to the
  fork's. Keep both lines, in alphabetical order.
- The `pane_keys` field of `Keymap`. The fork's `daemon` field sits right
  under it, so an upstream change to that field or its doc comment conflicts.
  Take upstream's lines and keep `daemon` below them.
- The tail of `LayoutMode`. Upstream adds variants where `Orbita` sits, so the
  enum, its name list, `ALL`, and every `match` arm conflict. Keep both with
  `Orbita` last, add `Orbita` to the arms it shares (`density`, `style`), and
  raise the length of `ALL` by hand: that line is outside
  the conflict blocks. `cargo fmt` spreads the `Classic` variant over four
  lines once `Orbita` is in the enum; leave it so.
- A fork commit that conflicts in the sidebar look file while only its own
  layout changed: take the file as the rebase has it
  (`git checkout HEAD -- <file>`) and put the commit's change in the Orbita
  layout's file. `pick.py FILE b` keeps both sides of a block, so when both
  rewrote the same line, delete the stale one by hand.
- A `fixup!` for an early fork commit that edits the Orbita README paragraph
  stops at every later fork commit that rewords or rewraps that paragraph.
  Resolve each stop as the later commit's text plus the fixup's words, then
  check that `git diff <fixup commit> vjr` is empty: the autosquash must end
  on the tree the fixup was written against.
- Autosquash matches a `fixup!` by subject, and every notes commit is
  `chore: update fork sync notes`, so a fixup for this sync's notes commit
  folds into the oldest one instead. When it is the tip, fold it by hand:
  commit the fix, `git reset --soft HEAD~2`, and `git commit -C <notes sha>`.
- `rerere` replays a resolution it recorded earlier whenever the same
  conflict returns, even when a dropped commit made it wrong: it once
  restored a `mod` line for a deleted test file. When a second rebase drops
  or rewrites a commit resolved earlier, run it with
  `git -c rerere.enabled=false rebase ...`, and read every file git reports
  as resolved before continuing.
- After the last stop, type-check every commit, not only the tip:
  `git rebase -x '<fmt check> && <cargo check>' main`. A commit that applied
  cleanly between two stops can still be the one that does not compile or is
  not formatted.
