#[derive(Debug, Clone, Default)]
pub struct NotificationOptions {
    pub app_name: Option<String>,
    pub icon_path: Option<String>,
    pub timeout: Option<u32>,
}

pub fn notify(summary: &str, body: &str) -> Result<(), String> {
    let mut notification = notify_rust::Notification::new();

    notification = notification.summary(summary).body(body);

    if let Some(timeout) = NotificationOptions::default().timeout {
        notification = notification.timeout(timeout);
    }

    notification.show().map_err(|err| err.to_string())?;
    Ok(())
}

impl NotificationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
