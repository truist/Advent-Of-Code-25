use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 05
#[derive(Parser)]
#[command()]
struct Args {
    /// Path to the input file
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let contents = match fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read {}: {e}", args.input.display());
            std::process::exit(1);
        }
    };

    process(contents);
}

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn includes(&self, val: u64) -> bool {
        self.min <= val && val <= self.max
    }
}

fn process(data: String) {
    let mut ranges: Vec<Range> = vec![];
    let mut fresh = 0;

    let mut range_mode = true;
    for line in data.lines() {
        if range_mode {
            if line == "" {
                range_mode = false;
            } else {
                let (min, max) = line.split_once("-").unwrap();
                ranges.push(Range {
                    min: min.parse().unwrap(),
                    max: max.parse().unwrap(),
                });
            }
        } else {
            let val = line.parse().unwrap();
            if ranges.iter().any(|range| range.includes(val)) {
                fresh += 1;
            }
        }
    }
    println!("{fresh}");
}
