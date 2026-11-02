# Platform support

Support matrix per capability, followed by platform-specific notes. `platform()`
(`Platform::Windows | Linux | MacOs | Other`) is always available for branching.

## Support matrix

| Capability | Linux | Windows | macOS |
| --- | --- | --- | --- |
| File dialogs (`rfd`) | ✅ | ✅ | ✅ |
| Notifications (`notify-rust`) | ✅ (freedesktop/DBus daemon required) | ✅ | ✅ |
| Clipboard text (`arboard`) | ✅ (X11, Wayland data-control) | ✅ | ✅ |
| Open URL / path (`open`) | ✅ (portal/xdg-open) | ✅ | ✅ |
| System tray | 🚧 planned (`ksni`, see [TRAY-DESIGN.md](TRAY-DESIGN.md)) | 🚧 planned (`tray-icon`) | 🚧 deferred (event-loop integration) |
| `platform()` detection | ✅ | ✅ | ✅ |

✅ = implemented and covered by the interactive test tier · 🚧 = designed /
planned, current code is a shape-only stub.

## Linux

- **Notifications** need a common freedesktop implementation: GNOME Shell
  notification service, Plasma, or a standalone daemon (`mako`, `dunst`,
  `swaync`). Without one, `notify()` returns `GuiUtilsError::Backend` (the
  DBus call fails).
- **File dialogs** go through `rfd`, which uses either the GTK dialog stack or
  the xdg-desktop-portal on Wayland systems. `Filter` entries
  (`("Name", ["ext", ...])`) and `file_name` are honored.
- **Clipboard** under Wayland uses the data-control protocol. Two behaviors
  worth knowing:
  - the clipboard ownership transfer right after `copy_text()` may need a
    moment to propagate to new readers (see `docs/TESTING.md`);
  - content persists only while some independent owner exists — a topic for
    the planned `ClipboardHandle` API.
- **Tray (planned)** builds on `ksni` (StatusNotifierItem over DBus). Note
  GNOME does **not** implement SNI natively — users need an AppIndicator
  extension (or a DE with built-in SNI: KDE, Xfce, Budgie, ...). The tray API
  will detect this at runtime and report `Unsupported` with an actionable
  message instead of silently disappearing.

## Windows

All implemented capabilities use native Windows APIs through the backend
crates. `save_file`/`pick_files` behavior follows rfd's Win32 implementation.
The tray implementation will use `tray-icon` (no framework); interactive
tests belong in the `#[ignore]` tier just like on Linux.

## macOS

Implemented capabilities ride on the backend crates' native support. The
tray implementation is **deferred in v0.1**: `tray-icon` on macOS requires
integration with a `winit`/`tao` event loop, which conflicts with the
crate's "no framework, no required main-loop" principle — a stub returning
`GuiUtilsError::Unsupported` with an explanation is the planned v0.1
behavior, and integration options are tracked in `TODOS.md` Phase 4.

## Headless / CI environments

`Platform::Other` is returned for unrecognized targets. Clipboard calls fail
cleanly with a descriptive `Backend` error; dialogs are interactive-only and
should be skipped in CI; notifications fail if no DBus daemon is reachable.
The automated test suite is designed to pass in all of these environments —
only the `#[ignore]` tier needs a real desktop session.
