//! Send notifications — both with default settings and fully configured.

use desktop_gui_utils::notification::{notify, notify_with, NotificationOptions};

fn main() {
    if let Err(err) = notify("desktop-gui-utils", "Plain notify() call.") {
        eprintln!("notify failed: {}", err);
    }

    let options = NotificationOptions {
        app_name: Some("desktop-gui-utils-example".to_string()),
        icon_path: None,
        timeout: Some(5000),
    };
    if let Err(err) = notify_with(&options, "desktop-gui-utils", "notify_with() with wired options") {
        // Err may be Unsupported (no notification daemon) or Backend.
        eprintln!("notify_with failed: {}", err);
    }
}
