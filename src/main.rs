mod options;

use std::sync::Mutex;

use anyhow::Result;
use clap::Clap;
use once_cell::sync::Lazy;
use rustyline::Editor;

use options::{Command, Opts};

static READ_LINE: Lazy<Mutex<Editor<()>>> = Lazy::new(|| Mutex::new(Editor::new()));

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match &opts.command {
        Command::New(new) => {
            new.run(&opts)?;
        }
    };
    Ok(())
}
