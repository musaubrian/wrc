use clap::Parser;
use wrc::wrc::{Wrc, WrcMode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WrsArgs {
    /// prints byte count
    #[arg(short, long)]
    bytes: bool,

    /// prints the character count
    #[arg(short, long)]
    char: bool,

    /// prints line count
    #[arg(short, long, default_value_t = true)]
    lines: bool,

    /// prints word count
    #[arg(short, long)]
    words: bool,

    /// input file
    #[arg(short, long)]
    file: String,
}

fn main() {
    // defaults to lines
    let args = WrsArgs::parse();
    let filename = &args.file;

    let mode = if args.words {
        WrcMode::Words
    } else if args.bytes {
        WrcMode::Bytes
    } else if args.char {
        WrcMode::Chars
    } else {
        WrcMode::Lines
    };

    let config = Wrc::new(filename.to_string(), mode);

    match config.count() {
        Ok(count) => println!("{}: {}", count, config.filename),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
