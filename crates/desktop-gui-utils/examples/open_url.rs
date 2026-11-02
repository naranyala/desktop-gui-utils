//! Open a URL and a local path with the platform default handlers.

use desktop_gui_utils::open::{open_path, open_url};

fn main() {
    match open_url("https://www.rust-lang.org") {
        Ok(()) => println!("Browser should be opening the Rust site."),
        Err(err) => eprintln!("open_url failed: {}", err),
    }

    match open_path(std::env::temp_dir()) {
        Ok(()) => println!("File manager should be opening the temp dir."),
        Err(err) => eprintln!("open_path failed: {}", err),
    }
}
