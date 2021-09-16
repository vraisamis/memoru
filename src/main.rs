mod command;

use clap::Clap;

use command::{Command, Opts};

fn main() {
    let opts: Opts = Opts::parse();

    match &opts.command {
        Command::New(new) => new.run(&opts),
    }
}
