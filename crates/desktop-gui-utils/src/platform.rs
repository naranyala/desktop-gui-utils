#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOs,
    Other,
}

impl Platform {
    pub fn is_windows(self) -> bool {
        matches!(self, Self::Windows)
    }

    pub fn is_linux(self) -> bool {
        matches!(self, Self::Linux)
    }

    pub fn is_macos(self) -> bool {
        matches!(self, Self::MacOs)
    }
}

pub fn platform() -> Platform {
    #[cfg(target_os = "windows")]
    {
        return Platform::Windows;
    }

    #[cfg(target_os = "linux")]
    {
        return Platform::Linux;
    }

    #[cfg(target_os = "macos")]
    {
        return Platform::MacOs;
    }

    Platform::Other
}
