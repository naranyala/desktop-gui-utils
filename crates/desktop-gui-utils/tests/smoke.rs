//! Public-API smoke tests.
//!
//! Everything here is headless-safe: no native dialogs, no real notifications,
//! no browser/file-manager launches. Those heavier paths are covered by
//! `#[ignore]`-flagged tests inside the modules (run with `cargo test -- --ignored`).

use std::path::Path;
use std::path::PathBuf;

use desktop_gui_utils::error::GuiUtilsError;
use desktop_gui_utils::{platform, Result, Platform};

#[test]
fn public_platform_api_works() {
    let detected = platform();

    assert!(
        matches!(
            detected,
            Platform::Windows | Platform::Linux | Platform::MacOs | Platform::Other
        ),
        "unexpected platform variant: {:?}",
        detected
    );
    assert_eq!(
        detected.is_linux(),
        matches!(detected, Platform::Linux),
        "is_linux should agree with the variant"
    );
}

#[test]
fn public_error_api_surface() {
    // Re-exported error type is constructible, comparable, and well-displayed.
    let backend = GuiUtilsError::Backend("integration probe".to_string());
    assert!(backend.is_backend());
    assert!(!backend.is_unsupported());
    assert!(backend.to_string().contains("integration probe"));
    assert_eq!(backend.message(), "integration probe");

    // It is usable behind the crate's `Result` alias and as a std error.
    fn take(_err: &dyn std::error::Error) {}
    take(&backend);
    let with_alias: Result<()> = Err(backend);
    assert!(with_alias.is_err());
}

#[cfg(feature = "file-dialog")]
#[test]
fn public_dialog_api_shape() {
    use desktop_gui_utils::dialog::FileDialogOptions;

    let options = FileDialogOptions::default();
    assert!(options.title.is_none());
    assert!(!options.multiple);
    assert!(options.file_name.is_none(), "save file_name defaults to None");

    let built = FileDialogOptions {
        title: Some("smoke".to_string()),
        multiple: true,
        ..FileDialogOptions::default()
    };
    assert_eq!(built.title.as_deref(), Some("smoke"));
    assert!(built.multiple);

    // Signatures match the documented contract without opening dialogs.
    let _pick_file: fn(FileDialogOptions) -> Option<PathBuf> =
        desktop_gui_utils::pick_file;
    let _pick_files: fn(FileDialogOptions) -> Option<Vec<PathBuf>> =
        desktop_gui_utils::pick_files;
    let _pick_directory: fn(FileDialogOptions) -> Option<PathBuf> =
        desktop_gui_utils::pick_directory;
    let _save_file: fn(FileDialogOptions) -> Option<PathBuf> =
        desktop_gui_utils::save_file;
}

#[cfg(feature = "file-dialog")]
#[test]
fn public_dialog_result_is_constructible() {
    use desktop_gui_utils::dialog::FileDialogResult;

    let single = FileDialogResult::Single(PathBuf::from("/tmp/a"));
    let multiple =
        FileDialogResult::Multiple(vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")]);

    assert!(matches!(single, FileDialogResult::Single(_)));
    match multiple {
        FileDialogResult::Multiple(paths) => assert_eq!(paths.len(), 2),
        _ => panic!("expected Multiple"),
    }
}

#[cfg(feature = "notification")]
#[test]
fn public_notification_api_shape() {
    use desktop_gui_utils::notification::{notify, notify_with, NotificationOptions};

    let options = NotificationOptions::new();
    assert!(options.app_name.is_none());
    assert!(options.icon_path.is_none());
    assert!(options.timeout.is_none());

    let configured = NotificationOptions {
        app_name: Some("smoke".to_string()),
        timeout: Some(1000),
        ..NotificationOptions::default()
    };
    assert_eq!(configured.app_name.as_deref(), Some("smoke"));
    assert_eq!(configured.timeout, Some(1000));

    // Signature checks only — real sends live in the module's #[ignore] tests.
    let _notify: fn(&str, &str) -> Result<()> = notify;
    let _notify_with: fn(&NotificationOptions, &str, &str) -> Result<()> = notify_with;
}

#[cfg(feature = "tray")]
#[test]
fn public_tray_api_shape() {
    use desktop_gui_utils::tray::{Tray, TrayBuilder};

    let tray = TrayBuilder::new()
        .title("smoke")
        .tooltip("smoke tray")
        .icon("smoke.png")
        .build();

    assert_eq!(tray.title.as_deref(), Some("smoke"));
    assert_eq!(tray.tooltip.as_deref(), Some("smoke tray"));
    assert_eq!(tray.icon_path.as_deref(), Some("smoke.png"));

    let default_tray = Tray::new();
    assert!(default_tray.title.is_none());
}

#[cfg(feature = "clipboard")]
#[test]
fn public_clipboard_api_graceful_everywhere() {
    use std::thread;
    use std::time::Duration;

    use desktop_gui_utils::clipboard::{copy_text, paste_text};

    match copy_text("public smoke probe") {
        Ok(()) => {
            // On X11/Wayland a read connection created immediately after the
            // copy can briefly see the clipboard as unavailable; give it a
            // moment instead of failing on a transient state.
            let mut pasted = Err(GuiUtilsError::Backend(String::new()));
            for attempt in 0..5 {
                if attempt > 0 {
                    thread::sleep(Duration::from_millis(100));
                }
                match paste_text() {
                    Ok(text) => {
                        pasted = Ok(text);
                        break;
                    }
                    Err(err) => pasted = Err(err),
                }
            }
            assert_eq!(
                pasted.expect("paste should succeed after copy (with retries)"),
                "public smoke probe"
            );
        }
        Err(err) => {
            // Headless backend failure is acceptable in CI; it must be descriptive.
            assert!(!err.message().is_empty(), "copy error should be descriptive");
            assert!(
                paste_text().is_err(),
                "when the backend is unavailable, paste should fail too"
            );
        }
    }
}

#[cfg(feature = "open-url")]
#[test]
fn public_open_api_signatures() {
    use desktop_gui_utils::open::{open_file, open_path, open_url};

    // Signature-only checks: referencing the functions proves the public
    // exports exist with the documented signatures, without launching apps
    // (real launches are covered by #[ignore] tests in the open module).
    // The path functions are generic, so wrap them in closures to pin a
    // concrete fn-pointer type.
    let url_fn: fn(&str) -> Result<()> = open_url;
    let path_fn: fn(&Path) -> Result<()> = |path| open_path(path);
    let file_fn: fn(&Path) -> Result<()> = |path| open_file(path);

    let _ = (url_fn, path_fn, file_fn);
}
