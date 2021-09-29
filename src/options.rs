use clap::{AppSettings, Clap};

mod edit;
mod new;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    New(new::New),
    Edit(edit::Edit),
}
