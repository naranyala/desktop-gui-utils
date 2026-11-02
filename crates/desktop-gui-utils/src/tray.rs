#[derive(Debug, Clone, Default)]
pub struct TrayBuilder {
    pub title: Option<String>,
    pub tooltip: Option<String>,
    pub icon_path: Option<String>,
}

impl TrayBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn icon(mut self, icon_path: impl Into<String>) -> Self {
        self.icon_path = Some(icon_path.into());
        self
    }

    pub fn build(self) -> Tray {
        Tray {
            title: self.title,
            tooltip: self.tooltip,
            icon_path: self.icon_path,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tray {
    pub title: Option<String>,
    pub tooltip: Option<String>,
    pub icon_path: Option<String>,
}

impl Tray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&self) {
        let _ = &self.title;
        let _ = &self.tooltip;
        let _ = &self.icon_path;
    }

    pub fn hide(&self) {
        let _ = &self.title;
    }

    pub fn set_tooltip(&mut self, tooltip: impl Into<String>) {
        self.tooltip = Some(tooltip.into());
    }

    pub fn set_icon(&mut self, icon_path: impl Into<String>) {
        self.icon_path = Some(icon_path.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_new_defaults_are_empty() {
        let builder = TrayBuilder::new();
        assert!(builder.title.is_none());
        assert!(builder.tooltip.is_none());
        assert!(builder.icon_path.is_none());
    }

    #[test]
    fn builder_chaining_sets_all_fields() {
        let tray = TrayBuilder::new()
            .title("My App")
            .tooltip("Runs in the background")
            .icon("/usr/share/icons/my-app.png")
            .build();

        assert_eq!(tray.title.as_deref(), Some("My App"));
        assert_eq!(tray.tooltip.as_deref(), Some("Runs in the background"));
        assert_eq!(tray.icon_path.as_deref(), Some("/usr/share/icons/my-app.png"));
    }

    #[test]
    fn builder_accepts_owned_and_borrowed_string_types() {
        let owned = TrayBuilder::new().title(String::from("owned")).build();
        assert_eq!(owned.title.as_deref(), Some("owned"));

        let borrowed = TrayBuilder::new().title("borrowed").build();
        assert_eq!(borrowed.title.as_deref(), Some("borrowed"));

        let cow = TrayBuilder::new()
            .title(std::borrow::Cow::Borrowed("cow"))
            .build();
        assert_eq!(cow.title.as_deref(), Some("cow"));
    }

    #[test]
    fn tray_new_defaults_are_empty() {
        let tray = Tray::new();
        assert!(tray.title.is_none());
        assert!(tray.tooltip.is_none());
        assert!(tray.icon_path.is_none());
    }

    #[test]
    fn setters_update_fields() {
        let mut tray = Tray::new();

        tray.set_tooltip("fresh tooltip");
        assert_eq!(tray.tooltip.as_deref(), Some("fresh tooltip"));

        tray.set_icon("icon.png");
        assert_eq!(tray.icon_path.as_deref(), Some("icon.png"));
    }

    #[test]
    fn show_and_hide_do_not_panic() {
        let tray = TrayBuilder::new()
            .title("smoke tray")
            .tooltip("no-op show/hide")
            .icon("icon.png")
            .build();

        tray.show();
        tray.hide();
        tray.show();
    }

    #[test]
    fn show_and_hide_work_on_default_tray() {
        let tray = Tray::new();
        tray.show();
        tray.hide();
    }
}

