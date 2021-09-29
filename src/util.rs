use std::env::var;

use anyhow::Result;
use duct::cmd;

pub fn get_editor() -> String {
    get_editor_or("vim")
}

pub fn get_editor_or(default_editor: &str) -> String {
    var("EDITOR").unwrap_or_else(|_| default_editor.to_string())
}

pub fn open_editor(editor: String, path: &str) -> Result<()> {
    cmd!(editor, path).run()?;
    Ok(())
}
