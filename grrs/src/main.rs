use clap::Parser;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context, Result};

// resource: https://rust-cli.github.io/book/tutorial/errors.html

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'r', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();

    // the "?" operator can be used to propagate errors
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            println!("End of file reached");
            break;
        }
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
        line.clear();

    }
    return Ok(())
}
