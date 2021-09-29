use anyhow::Result;
use clap::Clap;
use duct::cmd;
use walkdir::WalkDir;

use super::Opts;
use crate::util::{get_editor, open_editor};

#[derive(Clap)]
pub struct Edit {
    options: Vec<String>,
}

impl Edit {
    pub fn run(&self, _opts: &Opts) -> Result<()> {
        // メモファイルをリストアップ
        let memos: String = WalkDir::new(".")
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| {
                e.ok().and_then(|e| {
                    if e.path().extension().map_or(false, |ext| ext == "md") {
                        Some(e.path().to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
            .join("\n");
        let result = cmd!("peco").stdin_bytes(memos).read()?;
        println!("{}", result);

        let editor = get_editor();
        open_editor(editor, &result)?;
        Ok(())
    }
}
