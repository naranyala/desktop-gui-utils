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
    if cfg!(target_os = "windows") {
        return Platform::Windows;
    }

    if cfg!(target_os = "linux") {
        return Platform::Linux;
    }

    if cfg!(target_os = "macos") {
        return Platform::MacOs;
    }

    Platform::Other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_helpers_match_the_variant() {
        let detected = platform();

        assert_eq!(
            detected.is_windows(),
            matches!(detected, Platform::Windows),
            "is_windows disagrees with the variant"
        );
        assert_eq!(
            detected.is_linux(),
            matches!(detected, Platform::Linux),
            "is_linux disagrees with the variant"
        );
        assert_eq!(
            detected.is_macos(),
            matches!(detected, Platform::MacOs),
            "is_macos disagrees with the variant"
        );
    }

    #[test]
    fn platform_helpers_are_mutually_exclusive() {
        let detected = platform();
        let hits = [
            detected.is_windows(),
            detected.is_linux(),
            detected.is_macos(),
        ];
        assert_eq!(
            hits.iter().filter(|hit| **hit).count(),
            1,
            "exactly one platform helper should be true, detected: {:?}",
            detected
        );
    }

    #[test]
    fn detected_platform_is_a_known_variant() {
        let detected = platform();
        assert!(
            matches!(
                detected,
                Platform::Windows | Platform::Linux | Platform::MacOs | Platform::Other
            ),
            "unexpected platform variant: {:?}",
            detected
        );
    }

    #[test]
    fn platform_is_copy_and_eq() {
        let first = platform();
        let second = first;
        assert_eq!(first, second, "copying should preserve the value");
        assert_eq!(
            format!("{:?}", first),
            format!("{:?}", second),
            "debug output should be deterministic"
        );
    }
}

