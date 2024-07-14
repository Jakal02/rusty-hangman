use hangman;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Hangman")]
#[command(about = "CLI Hangman game.")]
#[command(version, long_about = None)]
struct RawArgs {
    /// Length of secret word
    length: Option<usize>,
}

pub struct Args {
    pub length: usize
}

fn main() {
    let args: Args = parse_args();

    hangman::run(args.length);
}

fn parse_args() -> Args {
    let raw: RawArgs = RawArgs::parse();

    let raw_len: usize = raw.length.unwrap_or(4);

    if raw_len < 3 {
        panic!("Hidden word length must be at least 3 characters.");
    }

    Args {
        length: raw_len,
    }
}