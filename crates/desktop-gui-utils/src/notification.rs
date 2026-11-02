use crate::error::{GuiUtilsError, Result};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NotificationOptions {
    /// Application name shown by the notification daemon.
    pub app_name: Option<String>,
    /// Path to an icon file shown next to the notification.
    pub icon_path: Option<String>,
    /// Expiration timeout in milliseconds.
    pub timeout: Option<u32>,
}

impl NotificationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Send a notification built from `options`.
///
/// All option fields are wired through to the underlying backend — unlike the
/// plain [`notify`] helper, which uses an all-defaults options struct.
pub fn notify_with(options: &NotificationOptions, summary: &str, body: &str) -> Result<()> {
    let mut notification = notify_rust::Notification::new();

    notification.summary(summary).body(body);

    if let Some(app_name) = &options.app_name {
        notification.appname(app_name);
    }

    if let Some(icon_path) = &options.icon_path {
        notification.image_path(icon_path);
    }

    if let Some(timeout) = options.timeout {
        notification.timeout(notify_rust::Timeout::Milliseconds(timeout));
    }

    notification
        .show()
        .map_err(|err| GuiUtilsError::Backend(err.to_string()))?;
    Ok(())
}

/// Send a notification with all-default options.
pub fn notify(summary: &str, body: &str) -> Result<()> {
    notify_with(&NotificationOptions::new(), summary, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_default_is_unset() {
        let options = NotificationOptions::default();
        assert!(options.app_name.is_none(), "app_name should default to None");
        assert!(
            options.icon_path.is_none(),
            "icon_path should default to None"
        );
        assert!(options.timeout.is_none(), "timeout should default to None");
    }

    #[test]
    fn options_new_matches_default() {
        assert_eq!(NotificationOptions::new(), NotificationOptions::default());
    }

    #[test]
    fn options_hold_every_field() {
        let options = NotificationOptions {
            app_name: Some("my-app".to_string()),
            icon_path: Some("/usr/share/icons/app.png".to_string()),
            timeout: Some(5000),
        };
        assert_eq!(options.app_name.as_deref(), Some("my-app"));
        assert_eq!(options.icon_path.as_deref(), Some("/usr/share/icons/app.png"));
        assert_eq!(options.timeout, Some(5000));
    }

    #[test]
    fn options_are_clonable_and_debuggable() {
        let options = NotificationOptions::new();
        let clone = options.clone();
        assert_eq!(options, clone);
        assert_eq!(
            format!("{:?}", options),
            "NotificationOptions { app_name: None, icon_path: None, timeout: None }"
        );
    }

    #[test]
    fn public_signatures_are_stable() {
        // Non-generic fns coerce to fn pointers directly.
        let _notify: fn(&str, &str) -> Result<()> = notify;
        let _notify_with: fn(&NotificationOptions, &str, &str) -> Result<()> = notify_with;
    }

    // --- Real notification tests: pop actual desktop notifications. ----------
    // Run manually with: cargo test -- --ignored

    #[test]
    #[ignore = "sends a real system notification; run with -- --ignored"]
    fn notify_sends_real_notification() {
        let result = notify(
            "desktop-gui-utils test",
            "If you can read this, notifications work.",
        );
        assert!(result.is_ok(), "notify failed: {:?}", result.err());
    }

    #[test]
    #[ignore = "sends a real system notification; run with -- --ignored"]
    fn notify_with_sends_configured_notification() {
        let options = NotificationOptions {
            app_name: Some("desktop-gui-utils".to_string()),
            icon_path: None,
            timeout: Some(5000),
        };
        let result = notify_with(&options, "desktop-gui-utils test", "notify_with wired options.");
        assert!(result.is_ok(), "notify_with failed: {:?}", result.err());
    }
}
