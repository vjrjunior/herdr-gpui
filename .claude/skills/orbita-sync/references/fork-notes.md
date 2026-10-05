# Fork notes for the next sync

What earlier syncs decided and where they got hurt. Upstream moves code between
files often, so entries name symbols, not paths: find them with a search.

## Standing decisions

| Area | Decision | Since |
| --- | --- | --- |
| Ahead and behind counts | Upstream draws them (`Upstream` in the sidebar row). The fork's own counts were dropped; the Orbita layout passes `row.upstream()` like Herdr's layout. | 2026-10-03 |
| `[daemon_keybindings]` | Kept. The user's `config-gpui.local.toml` binds `"prefix+m" = "cmd-shift-m"`. Upstream runs `[[keys.command]]` chords itself, so the table only adds a native shortcut beside the chord. Its bindings come from the local GUI config even while a device uses its server's keybindings. | 2026-10-03 |
| Plugin token badges | Open. The Orbita layout follows rows configured in the daemon's `[ui.sidebar.spaces]` like every layout, and draws its own badge line only on native rows (no configured rows, or `[usage] inline = false`). Whether the badge line stays is the user's call. | 2026-10-03 |
| Uncommitted mark on rows | Orbita rows draw none (`marks_uncommitted` is false for the Orbita style); the title bar marks the focused checkout. | 2026-09-28 |
| Tab strip height | The fork's extra 6px lives inside upstream's `tab_strip_height`, so everything upstream positions from the strip follows it. Revisit at the next sync: upstream then moves the tab row into the title bar and gives it a minimum height of its own (`titlebar::strip_height`), which may make the extra height redundant. | 2026-10-03 |

## Breaks without a conflict

After a stop, these fail to compile, to pass tests, or to pass a gate although
git merged them cleanly. The type-check finds the first four; only the tests and
`just ci` find the rest.

- **New callers of functions the fork changed.** `titlebar::render` takes a
  `&Theme` in the fork; `SidebarLook::mark` takes an `indent`; the shared
  sidebar `row` takes the fork's `tokens` list. Every window, layout, or rail
  upstream adds calls the old shape.
- **New constructions of structs the fork extended.** `RowContext` has a
  `worktree_font`. Upstream's new tests and previews build it without one.
- **New exhaustive matches.** `LayoutMode::Orbita` and
  `FontFace::SidebarWorktrees` are fork variants; upstream's new `match`es do
  not list them.
- **Upstream replacing an API the fork calls.** A fork function that used
  `status_style` broke when upstream moved status colors into `Indicators`.
  Follow upstream's replacement so the fork's marks keep matching its colors.
- **Upstream tests over `LayoutMode::ALL`.** They assert a contract for every
  layout, Orbita included. Make the Orbita layout honor the contract.
- **Gates upstream added after the fork's code was written.** From the sync
  after 2026-10-03: no Rust, Python, shell, or Swift file may pass 1,000 lines
  (`just check-file-size`, part of `just ci`), and test bodies live in `tests/`
  topic files, not inline in production files. Fork hooks added to a file
  upstream keeps near the limit tip it over, and fork tests written inline have
  to move. When a file crosses the limit, move the fork's part into a module of
  its own rather than trimming upstream's code.

## Recurring conflicts

- The Orbita paragraph of the crate README is edited by most fork commits, so
  it conflicts at nearly every stop. Keep the sentence about configured rows
  next to the badge description it qualifies.
- `Keymap` and the config `Settings` struct gain fields on both sides. Keep
  both; `with_overrides` must stay available outside tests because upstream
  builds server keymaps with it.
- Fork tests that asserted an unknown config key is rejected fail now: upstream
  ignores unknown keys and reports them in `unknown_keys`.
- When upstream splits a file the fork also edits, the fork's additions follow
  each symbol to its new file; git reports the old file as the conflict.
