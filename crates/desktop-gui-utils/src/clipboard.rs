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
