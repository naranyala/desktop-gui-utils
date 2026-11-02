use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct FileDialogOptions {
    pub title: Option<String>,
    pub initial_directory: Option<PathBuf>,
    pub filters: Vec<(&'static str, Vec<&'static str>)>,
    pub multiple: bool,
}

#[derive(Debug, Clone)]
pub enum FileDialogResult {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

pub fn pick_file(options: FileDialogOptions) -> Option<PathBuf> {
    #[cfg(feature = "file-dialog")]
    {
        let mut dialog = rfd::FileDialog::new();

        if let Some(title) = options.title {
            dialog = dialog.set_title(&title);
        }

        if let Some(directory) = options.initial_directory {
            dialog = dialog.set_directory(directory);
        }

        for (name, extensions) in options.filters {
            dialog = dialog.add_filter(name, &extensions);
        }

        if options.multiple {
            return dialog.pick_files().and_then(|paths| paths.into_iter().next());
        }

        return dialog.pick_file();
    }

    #[cfg(not(feature = "file-dialog"))]
    {
        let _ = options;
        None
    }
}

pub fn pick_directory(options: FileDialogOptions) -> Option<PathBuf> {
    #[cfg(feature = "file-dialog")]
    {
        let mut dialog = rfd::FileDialog::new();

        if let Some(title) = options.title {
            dialog = dialog.set_title(&title);
        }

        if let Some(directory) = options.initial_directory {
            dialog = dialog.set_directory(directory);
        }

        return dialog.pick_folder();
    }

    #[cfg(not(feature = "file-dialog"))]
    {
        let _ = options;
        None
    }
}

pub fn save_file(options: FileDialogOptions) -> Option<PathBuf> {
    #[cfg(feature = "file-dialog")]
    {
        let mut dialog = rfd::FileDialog::new();

        if let Some(title) = options.title {
            dialog = dialog.set_title(&title);
        }

        if let Some(directory) = options.initial_directory {
            dialog = dialog.set_directory(directory);
        }

        for (name, extensions) in options.filters {
            dialog = dialog.add_filter(name, &extensions);
        }

        return dialog.save_file();
    }

    #[cfg(not(feature = "file-dialog"))]
    {
        let _ = options;
        None
    }
}

pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}
