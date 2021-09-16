mod options;

use clap::Clap;

use options::{Command, Opts};

fn main() {
    let opts: Opts = Opts::parse();

    match &opts.command {
        Command::New(new) => new.run(&opts),
    }
}
