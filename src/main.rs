use hangman;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Hangman")]
#[command(about = "CLI Hangman game.")]
#[command(version, long_about = None)]
struct RawArgs {
    /// Length of secret word
    length: Option<u8>,
}

pub struct Args {
    pub length: u8
}

fn main() {
    let args: Args = parse_args();

    hangman::run(args.length);
}

fn parse_args() -> Args {
    let raw: RawArgs = RawArgs::parse();

    let raw_len: u8 = raw.length.unwrap_or(4);

    if raw_len > 6 || raw_len < 3 {
        panic!("Hidden word length must be 3 <= x <= 6, not {} characters.", raw_len);
    }

    Args {
        length: raw_len,
    }
}