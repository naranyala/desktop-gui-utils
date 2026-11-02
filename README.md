# Desktop GUI utilities, without a GUI framework

`desktop-gui-utils` is a Rust workspace that gives desktop applications native
OS capabilities through one small, framework-neutral API:

- file and directory picking, save dialogs
- system notifications
- clipboard access
- opening URLs and local files with the platform handler
- platform detection helpers

It deliberately avoids pulling in a UI toolkit — no `gtk`, `qt`, `slint`,
`egui`, `iced`, or `tauri`. Each capability delegates to a focused native
backend crate, so your binary only pays for the features it enables.

## Status

Version `0.1.0`, in active development. File dialogs, notifications,
clipboard, and URL/path opening are working. The system-tray capability is
currently a documented stub — see the [tray design](docs/TRAY-DESIGN.md) and
the [roadmap](TODOS.md).

## Quick start

```toml
# While the crate is in development, install from git:
[dependencies]
desktop-gui-utils = { git = "https://github.com/naranyala/desktop-gui-utils" }
```

```rust
use desktop_gui_utils::{
    dialog::{pick_file, FileDialogOptions},
    notification::notify,
    open::open_url,
};

fn main() {
    // Native file picker. `None` simply means the user cancelled — this is
    // not an error, which is why dialogs return Options.
    if let Some(path) = pick_file(FileDialogOptions {
        title: Some("Open a report".to_string()),
        initial_directory: None,
        filters: vec![("Markdown", vec!["md", "markdown"])],
        multiple: false,
        file_name: None,
    }) {
        println!("Picked: {}", path.display());
    }

    // Notifications instead return a Result.
    if let Err(err) = notify("My app", "Everything is ready.") {
        eprintln!("notification failed: {err}");
    }

    open_url("https://example.com").expect("browser should open");
}
```

More runnable snippets live in [`crates/desktop-gui-utils/examples/`](crates/desktop-gui-utils/examples)
and are compile-checked by `cargo test`.

## Capabilities

| Capability | Feature flag (default on) | Backend crate | Notes |
| --- | --- | --- | --- |
| File / directory pick, save-as | `file-dialog` | [`rfd`](https://crates.io/crates/rfd) 0.15 | `pick_file`, `pick_files`, `pick_directory`, `save_file` |
| System notifications | `notification` | [`notify-rust`](https://crates.io/crates/notify-rust) 4 | `notify`, `notify_with` for app name, icon and timeout |
| System tray | `tray` | planned: `ksni` / `tray-icon` | stub today — see design doc |
| Clipboard text | `clipboard` | [`arboard`](https://crates.io/crates/arboard) | `copy_text`, `paste_text` |
| Open URL / path | `open-url` | [`open`](https://crates.io/crates/open) | `open_url`, `open_path`, `open_file` |
| Platform detection | always | std | `platform()`, `Platform` enum |

Disable what you don't need:

```toml
desktop-gui-utils = { git = "...", default-features = false, features = ["clipboard"] }
```

## Error model

Two clear failure shapes instead of strings:

- **Dialogs return `Option`** — the user cancelling is not an error.
- **Everything else returns `Result<T, GuiUtilsError>`** with
  - `GuiUtilsError::Unsupported` — the feature can't work in this environment
    (headless session, no notification daemon), and
  - `GuiUtilsError::Backend` — the native backend reported an operational
    failure.

`is_unsupported()` / `is_backend()` let you branch on that distinction; type
`GuiUtilsError` implements `Display` and `std::error::Error`, so it plays well
with `anyhow`/`eyre` applications.

## Testing and quality

```sh
cargo test                      # headless-safe suite (unit + public-API smoke)
cargo test -- --ignored         # interactive tests: native dialogs, real
                                # notifications, browser/file-manager launches
cargo clippy --workspace --all-targets
```

The whole suite is designed to pass in headless CI environments — anything
that pops desktop UI is gated behind `#[ignore = "..."]` with instructions.
Details and platform caveats: [docs/TESTING.md](docs/TESTING.md).

## Documentation

| Doc | Contents |
| --- | --- |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | workspace layout, module design, dependency map |
| [`docs/TESTING.md`](docs/TESTING.md) | test strategy, how to run, clipboard/platform caveats |
| [`docs/PLATFORMS.md`](docs/PLATFORMS.md) | per-platform support matrix and Linux specifics |
| [`docs/TRAY-DESIGN.md`](docs/TRAY-DESIGN.md) | design sketch for the upcoming tray implementation |
| [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md) | dev workflow, commit conventions, checklist |
| [`TODOS.md`](TODOS.md) | development plan and status tracker |

## Contributing

PRs welcome! Read [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) first; keep the
"no GUI framework" rule in mind — capabilities should ride on small native
backends, not embed a toolkit.

## License

MIT — see [`LICENSE`](LICENSE).
