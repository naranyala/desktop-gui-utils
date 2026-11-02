# Testing strategy

The suite is designed so `cargo test` passes in **headless** environments
(CI containers, SSH sessions) — anything that would pop desktop UI is
explicitly gated behind `#[ignore = "<reason>"]`.

## Running

```sh
cargo test                      # headless-safe suite — should always pass
cargo test -- --ignored         # interactive tier: needs a display + you at the
                                # keyboard; opens dialogs/notifications/apps
cargo clippy --workspace --all-targets    # lint gate, expected clean
```

## Three tiers

### 1. Unit tests (`#[cfg(test)]` modules in `src/*.rs`)

- `platform.rs` — variant/helper consistency, mutual exclusivity, copy/eq.
- `error.rs` — Display format, variant probes, `Box<dyn std::error::Error>`
  compatibility, `Result` alias usage.
- `tray.rs` — builder defaults, chaining (owned/borrowed/Cow inputs),
  `build()` field transfer, setters, no-panic `show`/`hide`.
- `dialog.rs` — options defaults/all-fields/clone (including `file_name`),
  `FileDialogResult` variants, `file_exists` against real temp files and
  directories, and public-signature stability checks.
- `notification.rs` — options defaults/equality/debug, fn-signature checks.
- `clipboard.rs` — copy→paste round-trip when a backend exists, graceful
  descriptive errors when it doesn't, overwrite semantics.

Signature-stability tests (`let _f: fn(..) = f;`) compile-check the documented
types of non-generic functions directly; generic ones are pinned through
closures (a generic fn cannot coerce to a higher-ranked fn pointer directly).

### 2. Public-API smoke tests (`tests/smoke.rs`)

Iterate the crate exactly as an external consumer would: feature-gated tests
per module verify that the re-exports in `lib.rs` keep their documented shapes
(`FileDialogOptions` construction, `notify_with` signature, tray builder
chaining, clipboard grace behavior, open fn-pointer signatures). If a public
type or signature changes without intent, this file fails to compile.

### 3. Interactive tier (`#[ignore]`, run with `-- --ignored`)

- `dialog.rs`: native pick-file / pick-files / pick-directory / save-file
  dialogs (save-file passes a `file_name` to verify `set_file_name`).
- `notification.rs`: real `notify()` and `notify_with()` sends.
- `open.rs`: real browser / file-manager / editor launches.

Interactive tests assert only that the call *runs* and returns; whether the
user completes or cancels the interaction is not asserted.

## Platform caveats baked into tests

### Clipboard on X11/Wayland

A reader that opens a *new* clipboard connection immediately after
`copy_text()` may transiently see the clipboard as unavailable ("contents were
not available in the requested format or the clipboard is empty"), because on
Wayland data-control (and X11 selection transfer) the source must have fully
handed ownership to the compositor. Test remedy: a shared process-wide `Mutex`
serializes the clipboard tests (cargo runs tests on parallel threads, and they
all touch the one system clipboard) plus a bounded retry (5 × 100 ms) for the
paste. The round-trip tests also restore the original clipboard content.

This caveat is tracked as a potential API improvement (`ClipboardHandle`)
in `TODOS.md` — tests should not have to retry the public API.

### Headless environments

There is no clipboard backend without X11/Wayland, so clipboard tests assert
the *degradation path*: results are `GuiUtilsError::Backend` with a
non-empty descriptive message. Dialog tests are fully interactive-tier.

## CI plans

`TODOS.md` Phase 6: GitHub Actions on Linux running test + clippy, plus
`cargo check` for Windows/macOS targets (the invalid-`tray.rs` class of bug —
a file existing but never compiling — is exactly what CI catches early).
