# desktop-gui-utility

A cross-platform desktop utility crate collection for Rust without requiring a GUI framework.

This workspace provides a stable API for desktop capabilities such as:

- file and folder picking
- system notifications
- system tray support
- clipboard access
- opening URLs and local files
- platform detection and integration helpers

The design intentionally avoids UI frameworks like `gtk`, `qt`, `slint`, `egui`, `iced`, or `tauri`.
Instead, it uses native platform integrations through focused crates and keeps a single, framework-neutral public API.

## Workspace structure

- `crates/desktop-gui-utils` — primary crate exposing the public API

## Features

Enabled by default:

- `file-dialog`
- `notification`
- `tray`
- `clipboard`
- `open-url`

## Example

```rust
use desktop_gui_utils::{
    dialog::{pick_file, pick_directory, FileDialogOptions},
    notification::notify,
};

fn main() {
    if let Some(path) = pick_file(FileDialogOptions::default()) {
        println!("Picked file: {}", path.display());
    }

    if let Some(dir) = pick_directory(FileDialogOptions::default()) {
        println!("Picked directory: {}", dir.display());
    }

    let _ = notify("Application started", "The desktop utility is ready.");
}
```

## Intended usage

Use this crate as a shared library in desktop applications, tray-based tools, or utility apps that need native OS behaviors without tying themselves to a specific GUI toolkit.

## License

MIT
