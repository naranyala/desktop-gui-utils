//! Open a file, a directory, and save into a target — native dialogs only,
//! no GUI framework.

use desktop_gui_utils::dialog::{
    pick_directory, pick_file, save_file, FileDialogOptions,
};

fn main() {
    let options = FileDialogOptions::default();

    if let Some(file) = pick_file(options.clone()) {
        println!("Picked file: {}", file.display());
    } else {
        println!("No file picked.");
    }

    if let Some(mut dir) = pick_directory(options.clone()) {
        println!("Picked directory: {}", dir.display());
        dir.push("pick_files-count.txt");
        println!("(A save dialog would naturally offer something under {})", dir.display());
    } else {
        println!("No directory picked.");
    }

    let mut save_options = FileDialogOptions {
        title: Some("Save a file".to_string()),
        file_name: Some("report.md".to_string()),
        ..FileDialogOptions::default()
    };
    save_options.filters.push(("Markdown", vec!["md", "markdown"]));
    save_options
        .filters
        .push(("Text", vec!["txt"]));

    if let Some(target) = save_file(save_options) {
        println!("Save target: {}", target.display());
    } else {
        println!("Save cancelled.");
    }
}
