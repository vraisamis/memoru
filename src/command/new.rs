use clap::Clap;

use super::Opts;

#[derive(Clap)]
pub struct New {
    // #[clap(short, long)]
    title: Option<String>,
}

impl New {
    pub fn run(&self, _opts: &Opts) {
        if let Some(title) = &self.title {
            println!("title is '{}'.", title);
        } else {
            println!("title IS NOT SET.");
        }
    }
}
