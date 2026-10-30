# Fork notes for the next sync

What earlier syncs decided and where they got hurt. Upstream moves code between
files often, so entries name symbols, not paths: find them with a search.

## Standing decisions

| Area | Decision | Since |
| --- | --- | --- |
| Ahead and behind counts | Upstream draws them (`Upstream` in the sidebar row). The fork's own counts were dropped; the Orbita layout passes `row.upstream()` like Herdr's layout and right-aligns them (`counts_x`) on the branch line. Since upstream hangs a badge's counts line beside the branch line (`detail_limit`), that line ends one gap before the pull request's counts, where the badge column used to start; the fork's test measures against `pr-{key}`'s left edge. | 2026-10-21 |
| `[daemon_keybindings]` | Kept. The user's `config-gpui.local.toml` binds `"prefix+m" = "cmd-shift-m"`. Upstream runs `[[keys.command]]` chords itself, so the table only adds a native shortcut beside the chord. Its bindings come from the local GUI config even while a device uses its server's keybindings. Upstream lets a daemon `[[keys.command]]` on a direct keystroke take over a catalog default (`Shortcut::catalog_default`), but a prefix chord still gets no native shortcut, and the daemon cannot name a `cmd` key, so the table stays. A keystroke `[pane_keys]` holds is refused, like one a native shortcut holds. | 2026-10-05 |
| Plugin token badges | Open. The Orbita layout follows rows configured in the daemon's `[ui.sidebar.spaces]` like every layout, and draws its own badge line only on native rows (no configured rows, or `[usage] inline = false`). Whether the badge line stays is the user's call. | 2026-10-03 |
| Uncommitted mark on rows | Orbita rows draw none: the Orbita layout takes the mark off the badge it hands the shared row (`RowBadge::without_dirty`). Upstream's note mark stays on the badge. The title bar marks the focused checkout with a dot on its branch icon. Upstream's `[sidebar] change_counts = "local"` draws a checkout's counts in a pencil chip that stands for the mark; `without_dirty` keeps the counts, so that chip shows on Orbita rows, and `without_pr` keeps only local counts, since pull request counts belong to the number a `pr` token replaces. The user keeps the default `"pr"`. Open for the user if they switch. | 2026-10-21 |
| Orbita style | Upstream replaced the style traits with one `SidebarMetrics` value. The Orbita look is the rounded comfortable preset run through `SidebarMetrics::orbita`, which sets one `orbita` flag; everything the flag means is a `SidebarLook` method in the Orbita layout's file. Its child indent carries the guide's tick, so upstream code that reads `child_indent` lines up with Orbita rows. | 2026-10-08 |
| Rows under a host header | Upstream steps rows in under their host (`RowContext::nest`). An Orbita highlight starts at the row's whole indent, so with several hosts it steps in with the row; Herdr's layouts keep full-width highlights. Open for the user if they list several hosts. | 2026-10-08 |
| Fold chevron icon | Upstream ships `icons/chevron-right.svg` and registers it. The fork's copy and its registry entry were dropped. | 2026-10-08 |
| Bundle signing | `scripts/fork/update.sh` signs ad hoc, without the hardened runtime, so upstream's `Herdr.entitlements` is not applied: the `NSMicrophoneUsageDescription` key in upstream's `Info.plist` is what lets a pane ask for the microphone. If the script ever signs with `--options runtime`, pass the entitlements file too. | 2026-10-07 |
| Sidebar search field | Upstream made the field the Spaces heading (`search_heading`, on with `show_sidebar_search`). The Orbita layout keeps it and hangs its plus and hamburger at the heading's right, beside the field; with search off they sit on the plain header. The fork's header test measures the buttons' right margin against the field's left inset. Open for the user: whether Orbita should hide the field. | 2026-10-14 |
| Daemon binding errors | The `[daemon_keybindings]` and unbound-command errors are `keymap::daemon_bindings::Error`, in `keymap/daemon_bindings/error.rs`, wrapped once by the root `Error::DaemonBindings` (sorted after `Daemon`). Upstream's rule since 2026-10-24 names a feature's error `Error` in its module's `error.rs`, and its refactor shrank the root `error.rs` to about 370 lines, so the old boxing into `Error::ConfigFile` was dropped. Tests match the variant; a transparent variant's `source()` is the inner error's source, so the cause test reads it from the inner error. New fork errors go in that enum. | 2026-10-26 |
| Window backdrop | Upstream's `[backdrop]` paints every panel with its own color at the backdrop's opacity. The fork's `chrome` override replaces `theme.surface`, so it gets the same treatment with no code of its own. `titlebar::render` keeps upstream's signature; the tinted or flat choice is the `style` argument of `titlebar::bar`. | 2026-10-11 |
| Shortcut editor | Upstream's Herdr > Keyboard Shortcuts writes `[keybindings]`. It validates a draft through `Keymap::with_edited_overrides`, which carries the loaded `[daemon_keybindings]`, so it refuses a key the table holds instead of saving a config that no longer loads. Any new upstream surface that writes keybindings needs the same call. | 2026-10-11 |
| "Herdr UI" in messages | Upstream names the client "Herdr UI" in errors, notes, and dialogs. The fork's build-set name covers the window title, process name, application menu, and About box only, so Orbita's messages say "Herdr UI". Open for the user. | 2026-10-11 |
| Terminal-only font keys | Upstream's `[terminal]`-only keys (`line_height`, `weight`) are refused for `[sidebar_worktrees]` as for `[sidebar]`, `[tabs]`, and `[ui]`: the fork's face is in the list `Config` runs through `reject_terminal_keys`, so the key is reported in `unknown_keys` and ignored. A terminal-only key upstream adds to that function needs no fork code. | 2026-10-13 |
| Repository and device icons | Upstream's `show_repository_icons` and `show_device_icons` apply to the Orbita layout with no fork code: the layout hands the shared row the icon its context carries, and `LayoutMode::device_icons` leaves out only Orca and Minimal. | 2026-10-14 |
| Device themes | `[theme_overrides]` apply to the app theme only: `Config::theme_with_directories` runs upstream's shared `resolve` and then the overrides, for every theme source, Follow Herdr included. A device's `theme` (`theme_named`) paints only the terminal and takes none. | 2026-10-14 |
| Blocked bar and Edge rail | Upstream's `edge_rail` (Settings > Appearance > Indicators) draws a bar on the leading edge of working, done, and blocked rows in every layout (`Indicators::rail`, added once in `Cell::row`); the retired `status_icons = "rail"` reads as Outline with `edge_rail` on. The Orbita layout's own blocked bar was dropped for it. The app no longer reads Herdr's `ui.status_indicators`; the user's `config-gpui.local.toml` sets `status_icons = "braille"` (2026-10-19). | 2026-10-15 |
| Theme accent | Upstream's `Theme::accent` holds a desktop theme's own accent (Omarchy), and `system_accent` the Windows one, which wins. The fork's `[theme_overrides] accent` writes into upstream's `accent`, and only when set, so a desktop accent survives without a pin. `Theme` keeps only the fork's `chrome` field. | 2026-10-18 |
| Tab strip height | The fork's extra 6px was dropped. Upstream's tab row is the title bar now and is at least 34px tall, more than the fork's strip was. Open for the user: whether they want it taller still. | 2026-10-05 |
| Trunk under a note | Upstream carries a worktree child's tree guide on past its note and ports lines (`RowLayout::trunk_below`), and a test asks it of every layout that draws a tree. The Orbita layout passes `row::tree_gutter(cx)` to upstream's `row::trunk_below`, as Herdr's layout does; `trunk_below` and the row's `guide` read `tree_color` from the look, so the trunk matches the Orbita guide's color. | 2026-10-22 |
| Worktree guides | Upstream draws worktree guides in every layout through the shared `row::guide`, switched on by `[worktrees] apply_to` holding `"sidebar"` (the default; `SidebarMetrics::with_guides`), and `for_mode` clears `tree_lines` for all but flat Classic and Devices. Its test `guides_run_from_the_main_checkout_to_each_childs_leading_mark` holds every layout to a tick that ends at the child's status dot. The Orbita guide now meets it: the fork's `tree_padding` was dropped, so the tick runs into the nested highlight up to the dot instead of stopping at the highlight's edge, and the Orbita preset no longer forces `tree_lines`, so `apply_to = []` drops its guides as it does Rounded's. Open for the user: stopping the tick at the highlight again needs an Orbita arm in upstream's test. | 2026-10-22 |
| Current tab fill | Upstream moved tab colors into `tab_layout/`, one file per View > Tab Layout choice. `[theme_overrides] active_tab = "solid"` is the `theme.active_tab_fill()` call in `Classic::colors` and `Compact::colors`, the two that wash the current tab in the accent; Pills, Underline, Cards, and Curved do not use the accent there and ignore the key. A new layout that washes the current tab needs the same call. | 2026-10-22 |
| App name in messages | Upstream's menu and command labels are Fluent messages (`menus-herdr`, `menus-about-herdr`, `menus-hide-herdr`, `menus-quit-herdr`, `commands-about`). The fork lists those IDs in `i18n/app_name.rs`, and `i18n::format`'s no-argument branch passes the text through `app_name::named`, which puts the build's name in place of "Herdr". `menus.rs` and the English files stay upstream's verbatim, and the `herdr-commands` build script was dropped. A test fails when a listed ID is gone; a new upstream message naming the app (not the daemon) needs its ID added, and one that gains arguments needs the call moved to the other branch. | 2026-10-29 |
| Right to left | Upstream mirrors the chrome through `direction::Logical` (`ps`, `pe`, `row`, `inset_s`, `inset_e`) and every Herdr layout's rows. The fork's own code stays physical: the Orbita layout's file, `badge_line`, and `menu.right_edge`, which the `start` match in `menu/chrome.rs` reads as a distance from the window's left. Only the `en-XB` pseudo-locale can show the difference, since English is the one language that ships. Fork lines inside upstream's mirrored chains use the logical calls. Open for the user: mirroring Orbita needs its own change with an `en-XB` test. | 2026-10-30 |
| Worktree font in Settings | Upstream's rebuilt Settings window lists four faces (`FACES`), and the fork's `SidebarWorktrees` is not one of them, as it was not before: `[sidebar_worktrees]` is set in the config file only. The Fonts page's `size_control` and `specimen_title` still match on every `FontFace`, so they carry arms for it. | 2026-10-16 |

## Where the fork's code lives

Fork code sits in files the fork owns wherever possible. Upstream does not edit
these, so they cannot conflict:

- `config/theme_overrides.rs`: the `[theme_overrides]` types, re-exported from
  the config module.
- `palette/daemon_binding.rs`: running a daemon command by its binding label.
- `keymap/daemon_bindings.rs` and its `error.rs`: the module's `Error`, the
  keystroke parser the `Keymap` calls for `[daemon_keybindings]`, and
  `with_edited_overrides`.
- `config/orbita.rs`: `FontSettings::worktree_font` and `Settings::keymap`,
  which `Config::load` calls in one line each. `Settings::keymap` borrows the
  whole `Settings`, so its call sits right after `let mut config`, before the
  load starts moving fields out.
- `popup_chrome.rs`: the popup panel, with `PopupLook` as the one-line entry
  point the window render calls.
- `sidebar/layouts/orbita.rs` and `sidebar/orbita_tests.rs`. The layout file
  also holds the Orbita preset and the look methods the shared row and the
  sidebar render call: `mark_nested`, `tree_color`, `header_actions`,
  `primary_chip`, `counts_x`, and `spaces_header`, which hangs the plus and
  hamburger on the Spaces heading.
- `i18n/app_name.rs` and its `tests.rs`: the message IDs that name the app.
- `locales/en-US/orbita.ftl`: the fork's own Fluent messages, IDs prefixed
  `orbita-`: the layout's name (`LayoutMode::message`) and the worktree
  face's Fonts labels. Text the fork adds to a migrated module goes here.
- Topic test files: `config/tests/{theme_overrides,sidebar_worktrees,daemon_keybindings}.rs`,
  `keymap/tests/daemon_bindings.rs`, `titlebar/orbita_git_button_tests.rs`,
  `menu/shortcut_editor/tests/daemon_bindings.rs`.

What still has to live in upstream's files: the `Orbita` and `SidebarWorktrees`
variants, the `orbita` flag on `SidebarMetrics` with its arm in
`SidebarMetrics::for_mode`, `corners::ROW`, the `marked` line in
`SidebarLook::paint_highlight`, the one-line hooks in the shared sidebar `row`,
`row::guide`, and `row::trunk_below`, the `active_tab_fill` calls in two tab
layouts, the
`Theme` fields and accessors, the `Keymap` daemon bindings, the
`DaemonBindings` variant of the crate `Error`, the validation
call in the shortcut editor, the `sidebar_worktrees` entry in the list of
faces that take no terminal-only keys, `menu.svg` in the sorted
`embedded_icons!` table with the `branch` and `branch_dot` helpers after it,
the `style` argument of `titlebar::bar`, `mod app_name` and the
`app_name::named` call in `i18n::format`, the `SidebarWorktrees` arms of the
Fonts page's `face_message` and `sample_messages`, and one-line hooks in the
window render.

## Breaks without a conflict

After a stop, these fail to compile, to pass tests, or to pass a gate although
git merged them cleanly. The type-check finds the first four; only the tests and
`just ci` find the rest.

- **New callers of functions the fork changed.** `titlebar::bar` takes the
  title bar style in the fork, and the shared sidebar `row` takes the fork's
  `tokens` list. Every window or layout upstream adds calls the old shape.
  `SidebarLook::highlight` and `SidebarLook::mark` keep upstream's signatures:
  the Orbita highlight is nested by `mark_nested`, so do not add parameters
  to them again. `mark` takes the row's `(RowState, HostMark)` pair, and
  `mark_nested` takes the same pair and layers wash and highlight through
  upstream's `washed`, so the host colour sits in the order upstream picks.
  The title bar's attention dot is another `titlebar::bar` caller.
- **New constructions of structs the fork extended.** `RowContext` has a
  `worktree_font`, and `Theme` has `chrome`. Upstream's new tests,
  previews, and row builders (`append_agent_rows`, `append_device_agents`)
  build them without those.
- **New fields on structs the fork builds.** The Orbita layout rebuilds a
  `RowContext` for worktree rows and `RowBadge` through `RowBadge::new`, so a
  field upstream adds (`nest`, `mark`, `noted`, `counts`, `muted`) has to be
  forwarded there, and through `RowBadge::without_pr` and `without_dirty`.
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
  A pattern that names every face, as the CJK cascade's
  `let [sidebar, tabs, terminal, ui, sidebar_worktrees] = faces` does, names
  the fork's too, and the `for` under it takes `sidebar_worktrees` with the
  other non-terminal faces.
- **Error variants upstream moves into a feature's `Error`.** On 2026-10-24
  upstream moved config validation into `config::Error`, wrapped as
  `crate::Error::Settings`, and keymap failures into `keymap::Error`. Inside
  `config/tests/`, `Error` now names `config::Error`, so a fork test that
  expects a root variant (`MissingHome`) writes `crate::Error::...`, and one
  that expects a moved variant writes
  `crate::Error::Settings(Error::InvalidFontSize(..))`. The rebase stops on
  none of this; only the test target fails to compile.
- **Upstream replacing an API the fork calls.** A fork function that used
  `status_style` broke when upstream moved status colors into `Indicators`;
  the fork's `FontConfig::apply` became redundant when upstream added
  `FontSettings::apply`. Follow upstream's replacement and drop the fork's copy.
  `CommandInfo` lost its `shortcuts` field to per-platform profiles
  (`default_shortcuts()`), which the daemon binding check reads; a fork test
  that names a native default wraps it in `platform_default`, since Linux
  binds other keys.
- **Code upstream moves into a new crate.** Git carries a fork hunk into the
  moved file without a conflict, but the new crate lacks what the hunk relied
  on in `herdr-gpui`. On 2026-10-24 the command catalog moved to
  `herdr-commands`, and the palette's About label
  (`env!("HERDR_BUILD_APP_NAME")`) stopped compiling, since only
  `herdr-gpui`'s build script set the variable. The rebase did not stop; the
  first gate's `lint` failed. Type-check the workspace, not only `herdr-gpui`,
  after a range that splits crates out.
- **Text in a module marked `// i18n: migrated`.** `just check-i18n` refuses
  a string literal that reads as English in such a file, and in every file
  in the directory of the same name (`settings_window.rs` covers all of
  `settings_window/`, `menus.rs` all of `menus/`); tests are exempt. Neither
  the rebase nor the type-check runs it: run `python3 scripts/check-i18n.py`
  after the last stop. Fork text there becomes an `orbita-` message in
  `orbita.ftl`, called with `fl!`. A label upstream turns from a literal into a
  message ID, as `LayoutMode::message` did, needs an `orbita-` ID for the
  fork's arm. Check `git grep -l '// i18n: migrated'` against the fork's
  files: upstream is migrating module by module, so the set grows.
- **Upstream adding a parameter to a function the fork calls.**
  `HerdrWindow::reconnect` took a `cx`, which broke the fork's window tests
  (`orbita_tests`, the daemon binding test) in test targets only, and
  `Target::capture` took the pane's selection, which the daemon binding runner
  fills with `plugin_selection()` like upstream's `invoke_custom_command`.
  `agent_labels` took the pane's name; the Orbita layout's `agent` mirrors
  Herdr's and passes `agent.pane`.
- **Pull request fixtures.** Upstream shows a cached pull request only under
  the device it was found for, which on this machine needs a verified local
  daemon. A fork fixture that seeds `pr_cache` directly draws no badge and its
  test fails on a missing `pr-{key}`; seed through `HerdrWindow::seed_pr`, as
  upstream's own fixtures do.
- **Controls the chrome reaches by keyboard.** Upstream names every sidebar
  button for screen readers and makes it an F6-region stop through
  `HerdrWindow::sidebar_button` (`chrome_stop`). The Orbita header's plus and
  hamburger (`spaces_actions`, which takes the window for it) go through it
  too. A control the fork adds to the chrome needs the same call; nothing
  fails without it.
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
  On 2026-10-22 upstream left `sidebar/render.rs` at 993 lines and the fork's
  nine pushed it to 1,004; the fork's lines there are now two: the
  `worktree_font` field of `RowContext` and the `spaces_header` call, with the
  footer wrapped as `.children((!look.header_actions()).then(|| ...))`, which
  adds no line and keeps upstream's footer at its indentation. After the
  2026-10-30 sync `render.rs` stands at 981, the window's
  `window/render.rs` at 969, the crate `Error` file at 378, and `config.rs`
  at 959. To make room, turn a fork statement into a
  call to a helper in a fork-owned module; do not squeeze upstream's own
  lines.
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
  Three `herdr-orchestrator` tests in `tests::opening_script` expect only the
  fixture's `claude` to be found, but the probe (`installed_programs_script`)
  adds a login shell's PATH, and `/etc/paths.d/homebrew` puts
  `/opt/homebrew/bin`, where `codex` lives, on every login PATH here; no
  environment variable reaches the test, which clears it. On 2026-10-24 the
  gate ran its other steps, then both `cargo test` runs with `--no-fail-fast`
  and `-- --skip` for those three, `just test-mockup-wait`, and
  `scripts/fork/update.sh --no-sync --skip-ci`. Do the same while they fail
  and the fork leaves both crates untouched.
- **Upstream's process-timing tests.** On this machine
  `timeout_kills_shell_and_descendant_group` (`login_env`) fails in the first
  runs after a rebuild and passes alone: it gives a fresh shell script one
  second to start, which is lost while `syspolicyd` assesses the new
  executables. On 2026-10-15 it failed in a warm-up `cargo test` and the next
  gate, the brew `relaunch_retries_then_alerts_when_open_keeps_failing` test
  timed out beside it in the gate after that, and the third gate passed. The
  fork touches neither file. A warm-up run does not reliably help; rerun the
  gate until `syspolicyd` settles, and report the failures as upstream's
  instead of editing them. On 2026-10-21 the brew test failed three gates in a
  row (`login_env` in one of them) and failed on upstream `main`'s own binary
  too; the gate passed with `RUST_TEST_THREADS=8`, which runs every test, only
  fewer at a time. Say so in the report when it was used.
- **Comparing with upstream in a second worktree.** Workspace members' build
  artifacts are named without the checkout's path, so a worktree that shares
  `target/` overwrites this checkout's binaries, and Cargo then calls them
  fresh: a `cargo test` here ran upstream's binary. Give the other worktree
  its own `CARGO_TARGET_DIR`, or run `cargo clean -p herdr-gpui` here before
  trusting another build.
- **Test placement.** Test bodies live in `tests.rs` or `tests/` topic files,
  never inline in a production file. A fork test written inline has to move.
- **Fixtures other fork commits set up.** Dropping a fork commit can drop a line
  a later fork test relied on, and the test then passes for the wrong reason or
  fails. Check what the dropped commit added to shared test helpers.

## Recurring conflicts

- The Orbita paragraph of the crate README is edited by most fork commits, so
  it conflicts at nearly every stop. Keep the sentence about configured rows
  next to the badge description it qualifies. A wording fix in the commit
  that added the paragraph (the Orbita layout, on 2026-10-22) comes back at every
  later stop: take the later commit's side and reapply the fix.
- `Keymap` and the config `Settings` struct gain fields on both sides. Keep
  both; `with_overrides` must stay available outside tests because upstream
  builds server keymaps with it, and it forwards to `with_daemon_bindings`.
- Sorted shared structs. Since 2026-10-16 `Config`, its `Default`, the
  private `Settings`, and the crate `Error` enum sit in `keep-sorted` regions,
  so every fork field conflicts as a block that also holds upstream's
  reordered lines. Take upstream's side and put the fork's lines back in
  their sorted place: `daemon_keybindings` before `daytona`,
  `sidebar_worktrees` after `sidebar_style`, and `theme_overrides` after
  `theme`. The `Error` enum holds no fork variant since 2026-10-21. Then run
  `python3 scripts/check-sorted.py`.
- Upstream tests that name the application menu or its items "Herdr"
  (`menus/tests.rs`, `about.rs`). The fork's build-set name makes that menu
  `crate::WINDOW_TITLE`, so take upstream's test and write the title through
  it. On 2026-10-26 upstream's QA menu flag rewrote the test the fork had
  ported; the rest of the file merged cleanly.
- Fork tests that asserted an unknown config key is rejected fail now: upstream
  ignores unknown keys and reports them in `unknown_keys`.
- When upstream splits a file the fork also edits, or moves its test module
  out, git reports the whole old block as the conflict. Take upstream's side
  and replay the fork's hunks with `port_hunks.py`.
- The Spaces section of `render_sidebar`: the fork maps `spaces_header` over
  whatever upstream's heading chain returns (the search field or the plain
  header) and turns the footer's `.child(` into
  `.children((!look.header_actions()).then(|| {`, so an upstream edit to the
  footer's first or last line conflicts. Keep upstream's body between them.
- The test module list in `config/tests.rs` and the `mod` lists in `main.rs`,
  `config.rs` (`orbita`, `theme_overrides`), and the palette file conflict whenever upstream adds an entry next to the
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
  not formatted. Add `bash scripts/check-file-size.sh` to the command, since
  each commit has to stay under the limit too. It takes most of an hour, so run
  it in the background. If the session ends or the scratch copy of the helpers
  disappears mid-rebase, `git status` shows where it stopped: rerun that
  commit's check by hand and `git rebase --continue`, and take the helpers back
  out of the pre-sync tag with `git show <tag>:.claude/skills/orbita-sync/scripts/<helper>`.
