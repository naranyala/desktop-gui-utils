# Architecture

How `desktop-gui-utils` is put together and why.

## Workspace layout

```
Cargo.toml                      # workspace: resolver 2, shared pkg metadata
crates/desktop-gui-utils/       # the single public crate
    src/lib.rs                  # feature-gated module declarations + re-exports
    src/error.rs                # GuiUtilsError / Result — always available
    src/platform.rs             # Platform enum + platform() — always available
    src/dialog.rs               # rfd-backed file dialogs
    src/notification.rs         # notify-rust-backed notifications
    src/clipboard.rs            # arboard-backed clipboard text
    src/open.rs                 # `open`-backed URL/path handle launching
    src/tray.rs                 # Tray/TrayBuilder (shape stub, implementation pending)
    examples/                   # compile-checked runnable snippets
    tests/smoke.rs              # public-API smoke tests
```

## Design principles

1. **No GUI framework.** Capabilities ride on small native backend crates
   (`rfd`, `notify-rust`, `arboard`, `open`), never a toolkit like gtk/egui/
   tauri. The payoff is a tiny dependency tree per enabled feature and no
   event-loop baggage in applications that shouldn't have one.

2. **Feature gates mirror capabilities.** Every optional module is declared in
   `lib.rs` behind a cargo feature with the same name and default-on:
   `file-dialog`, `notification`, `tray`, `clipboard`, `open-url`. A consumer
   setting `default-features = false` gets a minimal build; each feature maps
   1:1 to its optional dependency via the `[features]` table in the crate's
   `Cargo.toml`.

3. **Thin, honest mapping — not wrappers.** The crate normalizes *shapes*
   (error type, options builders) but stays close to what the backend does.
   Backend-specific semantics leak intentionally where they matter (see the
   X11/Wayland clipboard note in `docs/TESTING.md`).

4. **`Option` for cancellation, `Result` for failure.**
   - Dialogs return `Option<..>`: the user pressing *Cancel* is not an error.
   - Notifications, clipboard, and `open` return `Result<T, GuiUtilsError>`:
     - `Unsupported(cause)` — the environment cannot provide the capability
       (headless session, missing notification daemon, no tray host), and
     - `Backend(cause)` — the native channel reported an operational failure.

5. **Library, not framework.** No global state, no required main-loop, no
   background threads the caller can't see. A call either completes (possibly
   blocking on user interaction, as dialogs must) and returns, or returns an
   error. The upcoming tray event model stays poll-based (`pop_event()`) for
   the same reason: the host application owns the loop.

## Error type internals

`error.rs` is always compiled (not feature-gated) so every module — and every
consumer — can name the same error type regardless of feature selection.
`Result<T> = std::result::Result<T, GuiUtilsError>` is a plain alias; the type
itself derives `Debug, Clone, PartialEq, Eq`, implements `Display` +
`std::error::Error`, and exposes `message()`, `is_unsupported()`, `is_backend()`.

## Backend API notes (for maintainers)

- `rfd::FileDialog` is configured through `dialog::build_dialog`, a single
  helper applying title / initial directory / `file_name` (`set_file_name`) /
  filters (`add_filter(*name, extensions)` — note the deref: iterating
  `Vec<(&'static str, ...)>` by reference yields `&&str`).
- `notify-rust` methods take `&mut self` and return `&mut Notification` —
  never reassign their result. Icons go through `image_path`, app identity
  through `appname`, timeouts through `Timeout::Milliseconds` (`From<i32>`
  exists but `u32`-safe construction is preferred).
- `arboard::Clipboard` is constructed per call today; a long-lived handle is
  a planned improvement (clipboard lifecycle, `docs/TESTING.md`).

## Testing layers

Unit tests live in `#[cfg(test)]` modules inside each `src` file (they can
reach internals); the public surface is pinned by `tests/smoke.rs`, which
couples the re-export view to the documented signatures. See
`docs/TESTING.md` for the full strategy and the `#[ignore]` interactive tier.
