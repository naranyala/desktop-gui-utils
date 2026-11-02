# TODOS + Development plan (desktop-gui-utils)

Cross-platform Rust crate collection for desktop capabilities (dialogs, notifications,
tray, clipboard, opening URLs/files) without a GUI framework.

Test entry points:
- `cargo test` — unit + integration tests, all headless-safe
- `cargo test -- --ignored` — integration tests that touch real desktop services
  (dialogs, notifications, browser opening); run interactively only

---

## Phase 1 — Restore the intended module layout ✅ (complete)

The original commit shipped source files in a cyclic rotation. Each block of code
was moved back to the module it belongs to, per the re-exports in `lib.rs`:

- [x] `notification.rs` — notification code (`NotificationOptions`, `notify`)
- [x] `tray.rs` — tray code (`Tray`, `TrayBuilder`) that `lib.rs` re-exports
- [x] `clipboard.rs` — clipboard code (`copy_text`, `paste_text`, backed by `arboard`)
- [x] `open.rs` — open code (`open_url`, `open_path`, `open_file`, backed by `open`)
- [x] `dialog.rs` — was already correct; left as-is
- [x] `tests/smoke.rs` — replaced orphaned code with a real smoke test

## Phase 2 — Test coverage for existing features ✅ (complete)

Headless-safe tests live in `#[cfg(test)]` modules inside each source file, plus
public-API shape tests in `tests/smoke.rs`:

- [x] `platform.rs` — variant consistency, copy/eq semantics of `Platform`
- [x] `tray.rs` — builder defaults, chaining, `build()` field transfer, setters,
  `show()`/`hide()` no-panic
- [x] `dialog.rs` — `FileDialogOptions` default/all-fields, `FileDialogResult`
  variants, `file_exists` with a real temp file
- [x] `notification.rs` — options defaults, `new()` vs `default()`
- [x] `clipboard.rs` — copy/paste round-trip when a clipboard backend is
  available; graceful error assertions in headless environments
- [x] `open.rs` / dialogs / real notifications — flagged `#[ignore]` because they
  trigger real desktop UI; runnable with `cargo test -- --ignored`

## Phase 3 — API hardening ✅ (complete)

- [x] Replace `Result<_, String>` with a `GuiUtilsError` enum
  (`Unsupported`, `Backend(String)`) + `Result<T>` alias, `message()`,
  `is_unsupported()`/`is_backend()` probes, `Display`, `std::error::Error`.
  Now returned by notification, clipboard, and open-modules (dialog stays
  `Option`-based since cancel is not an error).
- [x] Fix multi-select: added `pick_files() -> Option<Vec<PathBuf>>` which
  honors `multiple: true` and degrades to a 1-element vec otherwise; native
  `pick_files` results are no longer truncated to the first entry.
- [x] Add `FileDialogOptions::file_name` and wire it through `save_file` via
  rfd's `set_file_name`.
- [x] `notify()` now delegates to `notify_with(options, ...)`; `app_name`,
  `icon_path` (`image_path`), and `timeout` (`Timeout::Milliseconds`) are all
  actually wired to the backend.
- [ ] Docs: `#![deny(missing_docs)]`, doc comments on every public item
  (new APIs carry docs already; a full sweep still pending).
- [x] Runnable examples under `examples/` (dialog, notification, clipboard,
  open_url) — compiled as part of `cargo test`, so they double as API checks.

## Phase 4 — Real tray implementation (centerpiece, biggest task)

The current `Tray` type is a shape-only stub. Design sketch:

- [ ] **Linux**: `ksni` (pure-Rust, DBus/StatusNotifierItem — no GTK, fits the
  "no GUI framework" philosophy).
  - Shared-state struct (`Config` / `Status`) behind `Arc<Mutex<..>>` receiving
    event/activation updates from ksni's service thread.
  - Poll-based event model: menu activations push into an mpsc queue that the
    host app pops from its own loop — fits the crate's non-blocking pattern.
- [ ] **Windows**: `tray-icon` crate (native Win32, no framework).
- [ ] **macOS**: documented `Error::Unsupported` stub in v0.1 (tray-icon on macOS
  requires winit/tao event-loop integration — defer).
- [ ] **Runtime detection**: GNOME has no native SNI support (needs an
  AppIndicator extension) — detect at runtime and return a clear, actionable
  error instead of silently failing.
- [ ] **Builder API**: keep `TrayBuilder`; add `.menu_item(id, label)`,
  `.separator()`, icon-from-file, and menu/event plumbing.
- [ ] Menu items carry ids; `Tray::pop_event()` yields "which item was clicked".

## Phase 5 — Extended capabilities

- [ ] Notification actions: expose buttons ("OK"/"Cancel"), let the app read the
  clicked action via a handle (poll-based). Guard the known empty-summary panic
  in `notify-rust` by defaulting summary to `" "` when actions are present.
- [ ] Clipboard images: `copy_image` / `paste_image` (`arboard` image data).
- [ ] Clipboard persistence: on X11/Wayland a `Clipboard` dropped right after
  `copy_text` can briefly report empty/unavailable to a new reader (observed in
  tests even with immediate sequential calls). Investigate exposing a
  long-lived `ClipboardHandle` (or keeping a background clipboard owner) so
  copy-then-read is synchronous-safe for API users, not just tests.
- [ ] `rfd` daemon/dialog edge cases: directory shortcuts, dialog titles on
  Wayland, document portal quirks.

## Phase 6 — CI, packaging, polish

- [ ] GitHub Actions: Linux build + test matrix; Windows/macOS at least
  `cargo check` (via cross-compilation targets or feature-gated stubs).
- [ ] Package metadata for crates.io: `keywords`, `categories`, refresh README
  examples, pick semver story, publish `0.1.0` once Phase 3 lands.
- [ ] README: feature-flag table, quick-start, known limitations (headless envs,
  Wayland/X11 notes, GNOME SNI requirement).

## Phase 7 — Future ideas (parking lot)

- Async variants: `rfd::AsyncFileDialog`, notification/clipboard without blocking.
- Tray: submenus, checkable/radio menu items, per-item icon cache, dynamic menu
  re-render, notify-rust daemon exit handling.
- Platform helpers surface: freedesktop "is this Wayland/X11?", display-server
  inspector (`xdg-desktop-portal` presence), .desktop-file launcher discovery.
- Optional pixel-screenshot utilities — blocked on "no GUI framework" principle;
  only with pure-OS backends (e.g. `screenshot` via portal DBus).
