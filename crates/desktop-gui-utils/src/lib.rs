#![allow(dead_code)]

#[cfg(feature = "file-dialog")]
pub mod dialog;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "tray")]
pub mod tray;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "open-url")]
pub mod open;

pub mod platform;

pub use platform::{Platform, platform};

#[cfg(feature = "file-dialog")]
pub use dialog::{
    pick_directory, pick_file, save_file, FileDialogOptions, FileDialogResult,
};

#[cfg(feature = "notification")]
pub use notification::{notify, NotificationOptions};

#[cfg(feature = "tray")]
pub use tray::{Tray, TrayBuilder};

#[cfg(feature = "clipboard")]
pub use clipboard::{copy_text, paste_text};

#[cfg(feature = "open-url")]
pub use open::{open_file, open_path, open_url};
