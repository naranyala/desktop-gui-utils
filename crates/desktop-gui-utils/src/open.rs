use arboard::Clipboard;

pub fn copy_text(value: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|err| err.to_string())?;
    clipboard.set_text(value.to_owned()).map_err(|err| err.to_string())?;
    Ok(())
}

pub fn paste_text() -> Result<String, String> {
    let mut clipboard = Clipboard::new().map_err(|err| err.to_string())?;
    clipboard.get_text().map_err(|err| err.to_string())
}
