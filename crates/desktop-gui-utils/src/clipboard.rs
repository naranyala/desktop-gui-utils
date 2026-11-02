use arboard::Clipboard;

use crate::error::{GuiUtilsError, Result};

/// Open a fresh clipboard handle, mapping backend failures into
/// [`GuiUtilsError::Backend`].
fn open_clipboard() -> Result<Clipboard> {
    Clipboard::new().map_err(|err| GuiUtilsError::Backend(err.to_string()))
}

/// Copy `value` to the system clipboard.
///
/// On X11/Wayland the backend hands its content to the compositor when this
/// call returns; a reader that opens a new connection immediately afterwards
/// may — very rarely — still need a moment for the ownership to propagate.
pub fn copy_text(value: &str) -> Result<()> {
    let mut clipboard = open_clipboard()?;
    clipboard
        .set_text(value.to_owned())
        .map_err(|err| GuiUtilsError::Backend(err.to_string()))?;
    Ok(())
}

/// Read the current clipboard text.
pub fn paste_text() -> Result<String> {
    let mut clipboard = open_clipboard()?;
    clipboard
        .get_text()
        .map_err(|err| GuiUtilsError::Backend(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Mutex, OnceLock};
    use std::thread;
    use std::time::Duration;

    const PROBE_TEXT: &str = "desktop-gui-utils clipboard probe";

    /// The tests below all use the *system* clipboard, so they must not run in
    /// parallel with each other or they will overwrite each other's content.
    /// (cargo runs tests on parallel threads by default.)
    fn clipboard_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    /// On X11/Wayland the copy must fully hand its content to the compositor
    /// before another client can read it; a fresh read connection right after
    /// `copy_text` may occasionally see the clipboard as not-yet-available.
    /// Retry a handful of times before declaring the round trip broken.
    fn paste_with_retry(attempts: usize) -> Result<String> {
        let mut last_err = GuiUtilsError::Backend(String::new());
        for attempt in 0..attempts {
            if attempt > 0 {
                thread::sleep(Duration::from_millis(100));
            }
            match paste_text() {
                Ok(text) => return Ok(text),
                Err(err) => last_err = err,
            }
        }
        Err(last_err)
    }

    #[test]
    fn copy_and_paste_round_trip_when_backend_available() {
        let _guard = clipboard_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        // Remember the current content so it can be restored afterwards.
        let original = paste_text();

        match copy_text(PROBE_TEXT) {
            Ok(()) => {
                let pasted =
                    paste_with_retry(5).expect("paste should succeed right after a copy");
                assert_eq!(
                    pasted, PROBE_TEXT,
                    "pasted text should match what was copied"
                );

                // Restore whatever was on the clipboard before the probe.
                if let Ok(original_text) = original {
                    let _ = copy_text(&original_text);
                }
            }
            Err(err) => {
                // Headless environments (no X11/Wayland/portal) legitimately fail.
                // The error must be a descriptive backend failure.
                assert!(err.is_backend(), "clipboard failures are backend errors");
                assert!(!err.message().is_empty(), "error should carry a message");

                let paste_err = paste_text().unwrap_err();
                assert!(
                    !paste_err.message().is_empty(),
                    "paste errors describe the cause"
                );
            }
        }
    }

    #[test]
    fn copy_overwrites_previous_content() {
        let _guard = clipboard_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if paste_text().is_err() && copy_text("first value").is_err() {
            // No clipboard backend in this environment; nothing to verify.
            return;
        }

        let _ = copy_text("second value");
        let pasted = paste_with_retry(5).expect("paste should succeed when copy did");
        assert_eq!(
            pasted, "second value",
            "the second copy should win"
        );
    }
}
