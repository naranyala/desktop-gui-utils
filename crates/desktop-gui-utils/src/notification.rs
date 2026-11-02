use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct FileDialogOptions {
    pub title: Option<String>,
    pub initial_directory: Option<PathBuf>,
    pub filters: Vec<(&'static str, Vec<&'static str>)>,
    pub multiple: bool,
}

pub fn pick_file(options: FileDialogOptions) -> Option<PathBuf> {
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

    dialog.pick_file()
}

pub fn pick_directory(options: FileDialogOptions) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new();

    if let Some(title) = options.title {
        dialog = dialog.set_title(&title);
    }

    if let Some(directory) = options.initial_directory {
        dialog = dialog.set_directory(directory);
    }

    dialog.pick_folder()
}

pub fn save_file(options: FileDialogOptions) -> Option<PathBuf> {
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

    dialog.save_file()
}
