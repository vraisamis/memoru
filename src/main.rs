use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    New(New),
}

#[derive(Clap)]
struct New {
    #[clap(short, long)]
    title: Option<String>,
}

impl New {
    fn run(&self, opts: &Opts) {
        if let Some(title) = &self.title {
            println!("title is '{}'", title);
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match &opts.command {
        Command::New(new) => new.run(&opts),
    }
}
