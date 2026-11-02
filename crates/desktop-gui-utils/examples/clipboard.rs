//! Copy and paste text through the system clipboard.

use desktop_gui_utils::clipboard::{copy_text, paste_text};

fn main() {
    let message = "Hello from desktop-gui-utils!";

    match copy_text(message) {
        Ok(()) => match paste_text() {
            Ok(pasted) => println!("Copied and read back: {}", pasted),
            Err(err) => eprintln!("paste failed: {}", err),
        },
        Err(err) => {
            // `Unsupported` (headless session) vs `Backend` (operational failure).
            if err.is_unsupported() {
                eprintln!("No clipboard backend available in this environment.");
            } else {
                eprintln!("copy failed: {}", err);
            }
        }
    }
}
