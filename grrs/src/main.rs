// call this program with the following command:
// cargo run -- --path src/main.rs -r ::BufReader -vv
// cargo build && ./target/debug/grrs -h
use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context, Result};
use std::io::{self, Write};
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use grrs::find_matches;

// resource: https://rust-cli.github.io/book/tutorial/errors.html

// define what clap displays if we run the program with the --help flag
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'r', long = "pattern", default_value = "::File")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'p', long = "path", default_value = "src/main.rs")]
    path: std::path::PathBuf,
    /// How verbose do you want the program to be?
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

// by using the `anyhow` crate, we can return a `Result` from the main function
fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Cli::parse();

    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&args.verbose.log_level_filter().to_string()))
    .target(env_logger::Target::Stdout)
    .init();

    info!("Welcome to grrs 🐻");

    let stdout = io::stdout(); // get the global stdout entity
    // let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let mut handle = io::BufWriter::new(stdout.lock()); // acquire a lock on it

    // the "?" operator can be used to propagate errors
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    let reader = BufReader::new(&file);

    // get total number of lines in the file
    let total_lines: u64 = reader.lines().count() as u64;

    // Re-open the file to read lines again
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    let mut reader = BufReader::new(&file);
    let mut line = String::new();

    // Initialize the progress bar
    let pb = ProgressBar::new(total_lines);
    pb.set_style(ProgressStyle::default_bar()
        // this template configures what is displayed in the progress bar
        .template("Elapsed [{elapsed_precise}] {bar:20.green} (ETA:{eta})")
        .unwrap()
        .progress_chars("█▓▒░  "));

    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            // Writes to handle only get printed when they reach 8KB or when they get flushed
            writeln!(handle, "End of file reached")?;
            break;
        }
        find_matches(&line, &args.pattern, &mut handle);
        // Pause for a while to simulate a slow program
        std::thread::sleep(std::time::Duration::from_millis(20));
        pb.inc(1);
        line.clear();
    }

    pb.finish_with_message("Done");
    
    handle.flush()?;
    // The `Ok(())` syntax is a shorthand for returning `Ok(())`
    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches(&"lorem ipsum", &"lorem", &mut result);
    assert_eq!(result, b"lorem ipsum");
}
