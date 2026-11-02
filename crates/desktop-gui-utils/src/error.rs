//! Unified error type shared by every capability of the crate.
//!
//! Backend failures surface as [`GuiUtilsError::Backend`] with the native
//! error's message; capability gaps (platform or environment) surface as
//! [`GuiUtilsError::Unsupported`] so callers can branch on them.

use std::fmt;

/// Error returned by the notification, clipboard, and open-modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiUtilsError {
    /// The feature is not supported on this platform or environment
    /// (e.g. no notification daemon, no clipboard backend, headless session).
    Unsupported(String),

    /// The native backend (DBus, X11/Wayland selection, shell opener, ...)
    /// reported a failure. The string carries the backend's own message.
    Backend(String),
}

impl GuiUtilsError {
    /// True when the error means "this cannot work here" — usable to decide
    /// whether retrying makes sense.
    pub fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported(_))
    }

    /// True when the underlying backend reported an operational failure.
    pub fn is_backend(&self) -> bool {
        matches!(self, Self::Backend(_))
    }

    /// The message carried by the variant, without the variant label.
    pub fn message(&self) -> &str {
        match self {
            Self::Unsupported(message) | Self::Backend(message) => message,
        }
    }
}

impl fmt::Display for GuiUtilsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(message) => {
                write!(f, "not supported on this platform or environment: {}", message)
            }
            Self::Backend(message) => write!(f, "backend failure: {}", message),
        }
    }
}

impl std::error::Error for GuiUtilsError {}

/// Convenience alias so call sites can read `fn notify(...) -> Result<()>`.
pub type Result<T> = std::result::Result<T, GuiUtilsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_displays_with_label_and_message() {
        let error = GuiUtilsError::Unsupported("no tray daemon".to_string());
        let display = error.to_string();
        assert!(
            display.starts_with("not supported"),
            "label missing: {}",
            display
        );
        assert!(display.contains("no tray daemon"));
    }

    #[test]
    fn backend_displays_with_label_and_message() {
        let error = GuiUtilsError::Backend("selection owner declined".to_string());
        let display = error.to_string();
        assert!(display.starts_with("backend failure"), "got: {}", display);
        assert!(display.contains("selection owner declined"));
    }

    #[test]
    fn variant_probes_are_mutually_exclusive() {
        let unsupported = GuiUtilsError::Unsupported(String::new());
        let backend = GuiUtilsError::Backend(String::new());

        assert!(unsupported.is_unsupported());
        assert!(!unsupported.is_backend());

        assert!(backend.is_backend());
        assert!(!backend.is_unsupported());
    }

    #[test]
    fn message_strips_the_variant_label() {
        let unsupported = GuiUtilsError::Unsupported("headless".to_string());
        assert_eq!(unsupported.message(), "headless");

        let backend = GuiUtilsError::Backend("timed out".to_string());
        assert_eq!(backend.message(), "timed out");
    }

    #[test]
    fn errors_are_comparable_and_clonable() {
        let first = GuiUtilsError::Backend("x".to_string());
        let second = first.clone();
        assert_eq!(first, second);

        assert_ne!(
            GuiUtilsError::Backend("x".to_string()),
            GuiUtilsError::Unsupported("x".to_string()),
            "different variants with the same text are not equal"
        );
    }

    #[test]
    fn works_as_a_boxed_std_error() {
        let error = GuiUtilsError::Backend("boxed".to_string());
        let boxed: Box<dyn std::error::Error> = Box::new(error);
        assert!(boxed.to_string().contains("boxed"));
    }

    #[test]
    fn result_alias_wraps_the_error() {
        fn failing() -> Result<()> {
            Err(GuiUtilsError::Unsupported(String::new()))
        }

        let result = failing();
        assert!(result.is_err());
        assert!(result.unwrap_err().is_unsupported());
    }
}
