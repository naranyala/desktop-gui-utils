# Contributing

Thanks for helping improve `desktop-gui-utils`! Keep the core promise in
mind: **native capabilities, no GUI framework** — new features should ride on
small native backend crates, never on gtk/egui/tauri-style toolkits.

## Development workflow

```sh
git clone git@github.com:naranyala/desktop-gui-utils.git
cd desktop-gui-utils
cargo build
cargo test                # must stay green — it is headless-safe
cargo clippy --workspace --all-targets   # keep it warning-free
cargo fmt                 # format before committing
```

The `#[ignore]` tier (`cargo test -- --ignored`) opens real dialogs,
notifications, and applications — run it manually on a desktop session when
you touch that area. Expect popups.

## Conventions

- **Language/edition**: Rust (edition 2021), workspace version `0.1.0`,
  shared metadata in the root `Cargo.toml`.
- **Errors**: extend/return `GuiUtilsError` (`Unsupported` vs `Backend`);
  dialogs stay `Option`-based — user cancel is never an error.
- **Feature gates**: new optional capabilities get an entry in
  `crates/desktop-gui-utils/Cargo.toml` `[features]` plus the same-named
  optional dependency; module declaration in `lib.rs` behind the feature.
- **Tests**: every public API ships with (a) headless unit coverage in its
  module's `#[cfg(test)]` block, (b) a public-API entry in
  `tests/smoke.rs` (shape/signature checks), and (c) `#[ignore]`-flagged
  interactive tests when real desktop UI is involved — always with a
  `#[ignore = "<why, and how to run>"]` reason.
- **Docs**: document every public item; keep `docs/` pages and `TODOS.md`
  in sync with reality (platform caveats referenced in code comments point to
  `docs/TESTING.md`).

## Commit style

Short imperative subjects summarizing intent, one logical change per commit:

```
Add cross-platform tray builder API sketch
Fix multi-select truncation in file dialogs
Mark TODO items for Phase 4
```

## Planning

`TODOS.md` is the single source of truth for what to build next (phases 3–7
are open, each item carries priority context). Architecture rationale lives
in `docs/ARCHITECTURE.md`; platform specificities in `docs/PLATFORMS.md`.

## Reporting issues

Include the output of `uname -a` (or `systeminfo`), desktop environment /
compositor, and — for stack traces — `RUST_BACKTRACE=1`. Issues around
Wayland clipboard propagation or GNOME tray support are known and documented
(`docs/TESTING.md`, `docs/PLATFORMS.md`); check those pages before filing.
