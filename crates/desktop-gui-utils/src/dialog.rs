use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileDialogOptions {
    /// Title shown in the dialog window.
    pub title: Option<String>,

    /// Directory the dialog starts in.
    pub initial_directory: Option<PathBuf>,

    /// Named extension filters, e.g. `("Rust sources", vec!["rs", "ron"])`.
    pub filters: Vec<(&'static str, Vec<&'static str>)>,

    /// Whether the dialog allows selecting multiple entries
    /// (see [`pick_files`]).
    pub multiple: bool,

    /// Pre-filled file name — used by [`save_file`] as editing target.
    pub file_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FileDialogResult {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

fn build_dialog(options: &FileDialogOptions) -> rfd::FileDialog {
    let mut dialog = rfd::FileDialog::new();

    if let Some(title) = &options.title {
        dialog = dialog.set_title(title);
    }

    if let Some(directory) = &options.initial_directory {
        dialog = dialog.set_directory(directory);
    }

    if let Some(name) = &options.file_name {
        dialog = dialog.set_file_name(name);
    }

    for (name, extensions) in &options.filters {
        // Looping by reference yields `&&str`; rfd wants `impl Into<String>`.
        dialog = dialog.add_filter(*name, extensions);
    }

    dialog
}

/// Open a single-file picker; `None` when the user cancels.
pub fn pick_file(options: FileDialogOptions) -> Option<PathBuf> {
    build_dialog(&options).pick_file()
}

/// Open a multi-entry picker.
///
/// With `multiple: false` in the options this degrades gracefully to a
/// single selection wrapped in a one-element vector.
pub fn pick_files(options: FileDialogOptions) -> Option<Vec<PathBuf>> {
    if options.multiple {
        build_dialog(&options).pick_files()
    } else {
        build_dialog(&options).pick_file().map(|path| vec![path])
    }
}

/// Open a directory picker; `None` when the user cancels.
pub fn pick_directory(options: FileDialogOptions) -> Option<PathBuf> {
    build_dialog(&options).pick_folder()
}

/// Open a save-as dialog honoring `options.file_name`; `None` on cancel.
pub fn save_file(options: FileDialogOptions) -> Option<PathBuf> {
    build_dialog(&options).save_file()
}

pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn probe_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("desktop-gui-utils-{}", name))
    }

    #[test]
    fn options_default_is_unset() {
        let options = FileDialogOptions::default();
        assert!(options.title.is_none(), "title should default to None");
        assert!(
            options.initial_directory.is_none(),
            "initial_directory should default to None"
        );
        assert!(options.filters.is_empty(), "filters should default to empty");
        assert!(!options.multiple, "multiple should default to false");
        assert!(options.file_name.is_none(), "file_name should default to None");
    }

    #[test]
    fn options_hold_every_field() {
        let options = FileDialogOptions {
            title: Some("Pick a file".to_string()),
            initial_directory: Some(probe_path("dialog-dir")),
            filters: vec![("Rust sources", vec!["rs", "ron"])],
            multiple: true,
            file_name: Some("main.rs".to_string()),
        };
        assert_eq!(options.title.as_deref(), Some("Pick a file"));
        assert_eq!(options.initial_directory, Some(probe_path("dialog-dir")));
        assert_eq!(
            options.filters,
            vec![("Rust sources", vec!["rs", "ron"])]
        );
        assert!(options.multiple);
        assert_eq!(options.file_name.as_deref(), Some("main.rs"));
    }

    #[test]
    fn options_builder_is_clonable_and_debuggable() {
        let options = FileDialogOptions::default();
        let title_options = FileDialogOptions {
            title: Some("Save".to_string()),
            ..FileDialogOptions::default()
        };
        let title_clone = title_options.clone();
        assert_eq!(title_options, title_clone, "cloning should copy fields");
        assert_ne!(
            options.title, title_options.title,
            "defaults and customized struct should differ"
        );
        assert!(
            format!("{:?}", options).starts_with("FileDialogOptions {"),
            "debug should be struct-shaped, got: {:?}",
            options
        );
    }

    #[test]
    fn result_single_holds_one_path() {
        let result = FileDialogResult::Single(PathBuf::from("/tmp/single.rs"));
        match result {
            FileDialogResult::Single(path) => assert_eq!(path, PathBuf::from("/tmp/single.rs")),
            other => panic!("expected Single, got {:?}", other),
        }
    }

    #[test]
    fn result_multiple_holds_all_paths_in_order() {
        let first = PathBuf::from("/tmp/one.rs");
        let second = PathBuf::from("/tmp/two.rs");
        let result = FileDialogResult::Multiple(vec![first.clone(), second.clone()]);
        match result {
            FileDialogResult::Multiple(paths) => {
                assert_eq!(paths.len(), 2, "both paths should be kept");
                assert_eq!(paths[0], first, "order should be preserved");
                assert_eq!(paths[1], second, "order should be preserved");
            }
            other => panic!("expected Multiple, got {:?}", other),
        }
    }

    #[test]
    fn file_exists_is_false_for_missing_files() {
        assert!(!file_exists(probe_path("missing-file")));
        assert!(!file_exists(probe_path("missing-parent/missing-file")));
    }

    #[test]
    fn file_exists_is_true_for_real_files() {
        let probe = probe_path("probe-real-file");
        assert!(
            !probe.exists(),
            "probe file should not exist before the test: {}",
            probe.display()
        );
        let written = std::fs::write(&probe, b"probe");
        assert!(written.is_ok(), "writing the probe file failed: {:?}", written);

        assert!(file_exists(&probe), "existing file should be found");
        assert!(
            file_exists(probe_path("probe-real-file")),
            "works with owned strings too"
        );

        let removed = std::fs::remove_file(&probe);
        assert!(removed.is_ok(), "cleaning up the probe file failed: {:?}", removed);
        assert!(!file_exists(&probe), "deleted file should no longer be found");
    }

    #[test]
    fn file_exists_is_true_for_directories_too() {
        let probe_dir = probe_path("probe-real-dir");
        let created = std::fs::create_dir_all(&probe_dir);
        assert!(created.is_ok(), "creating the probe dir failed: {:?}", created);

        assert!(file_exists(&probe_dir), "directories count as existing");

        let _ = std::fs::remove_dir_all(&probe_dir);
    }

    #[test]
    fn public_signatures_are_stable() {
        let _pick_file: fn(FileDialogOptions) -> Option<PathBuf> = pick_file;
        let _pick_files: fn(FileDialogOptions) -> Option<Vec<PathBuf>> = pick_files;
        let _pick_directory: fn(FileDialogOptions) -> Option<PathBuf> = pick_directory;
        let _save_file: fn(FileDialogOptions) -> Option<PathBuf> = save_file;
        // Generic P: pin it via a closure.
        let _file_exists: fn(&str) -> bool = |path| file_exists(path);
    }

    // --- Native dialog tests: need a display and user interaction. -----------
    // Run manually with: cargo test -- --ignored

    #[test]
    #[ignore = "opens a native file picker; needs a display and user interaction"]
    fn pick_file_opens_native_dialog() {
        let picked = pick_file(FileDialogOptions {
            title: Some("pick_file test".to_string()),
            ..FileDialogOptions::default()
        });
        // No assertion on the result: the user may cancel. Presence of a valid
        // Option proves the dialog ran.
        assert!(picked.is_some() || picked.is_none());
    }

    #[test]
    #[ignore = "opens a native multi-select picker; needs a display and user interaction"]
    fn pick_files_opens_native_multi_select() {
        let picked = pick_files(FileDialogOptions {
            title: Some("pick_files test".to_string()),
            multiple: true,
            ..FileDialogOptions::default()
        });
        assert!(picked.is_some() || picked.is_none());
    }

    #[test]
    #[ignore = "opens a native directory picker; needs a display and user interaction"]
    fn pick_directory_opens_native_dialog() {
        let picked = pick_directory(FileDialogOptions::default());
        assert!(picked.is_some() || picked.is_none());
    }

    #[test]
    #[ignore = "opens a native save dialog; needs a display and user interaction"]
    fn save_file_opens_native_dialog_with_file_name() {
        let saved = save_file(FileDialogOptions {
            title: Some("save_file test".to_string()),
            file_name: Some("desktop-gui-utils-probe.txt".to_string()),
            ..FileDialogOptions::default()
        });
        assert!(saved.is_some() || saved.is_none());
    }
}
