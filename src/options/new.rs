use anyhow::Result;
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
            r.readline("Title is not set.\nType the title: ")
                .unwrap_or_else(|_| "NO_TITLE".to_string())
        });
        println!("THE TITLE IS [{}]", title);
        Ok(())
    }
}
