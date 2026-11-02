use std::path::Path;

use crate::error::{GuiUtilsError, Result};

/// Open `url` with the platform's default browser/handler.
pub fn open_url(url: &str) -> Result<()> {
    open::that(url).map_err(|err| GuiUtilsError::Backend(err.to_string()))
}

/// Open `path` with the platform's default handler for its file type
/// (or the file manager, for directories).
pub fn open_path<P: AsRef<Path>>(path: P) -> Result<()> {
    open::that(path.as_ref()).map_err(|err| GuiUtilsError::Backend(err.to_string()))
}

/// Open `path` like [`open_path`] — kept as a distinct intent-revealing name.
pub fn open_file<P: AsRef<Path>>(path: P) -> Result<()> {
    open::that(path.as_ref()).map_err(|err| GuiUtilsError::Backend(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // All of these launch real desktop apps (browser, file manager, editor),
    // so they are ignored by default. Run manually with: cargo test -- --ignored

    #[test]
    #[ignore = "launches a real browser; run with -- --ignored"]
    fn open_url_launches_the_default_browser() {
        let result = open_url("https://www.rust-lang.org");
        assert!(result.is_ok(), "open_url failed: {:?}", result.err());
    }

    #[test]
    #[ignore = "launches the file manager; run with -- --ignored"]
    fn open_path_launches_the_default_handler() {
        let result = open_path(std::env::temp_dir());
        assert!(result.is_ok(), "open_path failed: {:?}", result.err());
    }

    #[test]
    #[ignore = "launches an editor for the file; run with -- --ignored"]
    fn open_file_launches_the_default_handler() {
        let probe = std::env::temp_dir().join("desktop-gui-utils-open-probe.txt");
        std::fs::write(&probe, "open.rs probe").expect("writing the probe file failed");
        let result = open_file(&probe);
        let _ = std::fs::remove_file(&probe);
        assert!(result.is_ok(), "open_file failed: {:?}", result.err());
    }
}
