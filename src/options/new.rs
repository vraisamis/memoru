use std::fs::File;
use std::io::Write;

use anyhow::Result;
use chrono::Local;
use clap::Clap;

use super::Opts;

#[derive(Clap)]
pub struct New {
    // #[clap(short, long)]
    title: Option<String>,
}

impl New {
    pub fn run(&self, _opts: &Opts) -> Result<()> {
        let title: String = self.title.clone().unwrap_or_else(|| {
            let mut r = crate::READ_LINE.lock().unwrap();
            println!("Title is not set.");
            r.readline("Type the title: ").map_or_else(
                |_| "NO_TITLE".to_string(),
                |s| {
                    if s.len() > 0 {
                        s
                    } else {
                        "NO_TITLE".to_string()
                    }
                },
            )
        });
        dbg!(&title);
        let local = Local::now().format("%Y-%m-%d").to_string();
        dbg!(&local);

        let filename = format!("{}-{}.md", local, title);
        let mut file = File::create(filename)?;
        file.write_all(format!("# {}", title).as_bytes())?;
        Ok(())
    }
}
