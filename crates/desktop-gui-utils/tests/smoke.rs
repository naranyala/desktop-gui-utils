use std::path::Path;

pub fn open_url(url: &str) -> Result<(), String> {
    open::that(url).map_err(|err| err.to_string())
}

pub fn open_path<P: AsRef<Path>>(path: P) -> Result<(), String> {
    open::that(path.as_ref()).map_err(|err| err.to_string())
}

pub fn open_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
    open::that(path.as_ref()).map_err(|err| err.to_string())
}
