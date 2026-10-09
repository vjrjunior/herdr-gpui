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
| Tab strip height | The fork's extra 6px was dropped. Upstream's tab row is the title bar now and is at least 34px tall, more than the fork's strip was. Open for the user: whether they want it taller still. | 2026-10-05 |

## Where the fork's code lives

Fork code sits in files the fork owns wherever possible. Upstream does not edit
these, so they cannot conflict:

- `config/theme_overrides.rs`: the `[theme_overrides]` types, re-exported from
  the config module.
- `palette/daemon_binding.rs`: running a daemon command by its binding label.
- `popup_chrome.rs`: the popup panel, with `PopupLook` as the one-line entry
  point the window render calls.
- `sidebar/layouts/orbita.rs` and `sidebar/orbita_tests.rs`. The layout file
  also holds the Orbita preset and the look methods the shared row and the
  sidebar render call: `mark_nested`, `mark_blocked`, `tree_color`,
  `tree_padding`, `header_actions`.
- Topic test files: `config/tests/{theme_overrides,sidebar_worktrees,daemon_keybindings}.rs`,
  `keymap/tests/daemon_bindings.rs`, `titlebar/orbita_git_button_tests.rs`.

What still has to live in upstream's files: the `Orbita` and `SidebarWorktrees`
variants, the `orbita` flag on `SidebarMetrics` with its arm in
`SidebarMetrics::for_mode`, `corners::ROW`, the `marked` line in
`SidebarLook::highlight`, the one-line hooks in the shared sidebar `row`, the
`Theme` fields and accessors, the `Keymap` daemon bindings, and one-line hooks
in the window render.

## Breaks without a conflict

After a stop, these fail to compile, to pass tests, or to pass a gate although
git merged them cleanly. The type-check finds the first four; only the tests and
`just ci` find the rest.

- **New callers of functions the fork changed.** `titlebar::render` takes a
  `&Theme` in the fork, and the shared sidebar `row` takes the fork's `tokens`
  list. Every window or layout upstream adds calls the old shape.
  `SidebarLook::highlight` and `SidebarLook::mark` keep upstream's signatures:
  the Orbita highlight is nested by `mark_nested`, so do not add parameters
  to them again.
- **New constructions of structs the fork extended.** `RowContext` has a
  `worktree_font`, and `Theme` has `accent` and `chrome`. Upstream's new tests,
  previews, and row builders (`append_agent_rows`) build them without those.
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
  not list them.
- **Upstream replacing an API the fork calls.** A fork function that used
  `status_style` broke when upstream moved status colors into `Indicators`;
  the fork's `FontConfig::apply` became redundant when upstream added
  `FontSettings::apply`. Follow upstream's replacement and drop the fork's copy.
- **Visibility after a move.** When upstream moves a type into a submodule, a
  fork method added with `pub(super)` stops being reachable. `RowBadge` methods
  need `pub(in crate::sidebar)`.
- **Upstream tests over `LayoutMode::ALL`.** They assert a contract for every
  layout, Orbita included. Make the Orbita layout honor the contract.
- **The 1,000-line limit.** `just check-file-size` is part of `just ci`. The
  window render sits a few lines under it with the fork's hooks, so every hook
  there must stay one line; move anything longer into a fork-owned module.
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
- Fork tests that asserted an unknown config key is rejected fail now: upstream
  ignores unknown keys and reports them in `unknown_keys`.
- When upstream splits a file the fork also edits, or moves its test module
  out, git reports the whole old block as the conflict. Take upstream's side
  and replay the fork's hunks with `port_hunks.py`.
- The Spaces section of `render_sidebar`: the fork hangs `header_actions` on
  the header and wraps the footer in `when(!header_actions)`, so an upstream
  edit to what sits between them conflicts. Keep upstream's body, such as the
  wrapper that clips the pinned host header, between the fork's two hooks.
- The test module list in `config/tests.rs` conflicts whenever upstream adds a
  topic file next to the fork's. Keep both lines, in alphabetical order.
- A fork commit that conflicts in the sidebar look file while only its own
  layout changed: take the file as the rebase has it
  (`git checkout HEAD -- <file>`) and put the commit's change in the Orbita
  layout's file. `pick.py FILE b` keeps both sides of a block, so when both
  rewrote the same line, delete the stale one by hand.
- After the last stop, type-check every commit, not only the tip:
  `git rebase -x '<fmt check> && <cargo check>' main`. A commit that applied
  cleanly between two stops can still be the one that does not compile or is
  not formatted.
