use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 07
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

fn process(data: String) {
    let mut lines = data.lines();

    let mut chars: Vec<char> = lines.next().unwrap().chars().collect();
    let cols = chars.len();

    let mut beams = vec![false; cols];
    let mut splits = 0;

    beams[chars.iter().position(|&c| c == 'S').unwrap()] = true;

    while let Some(line) = lines.next() {
        chars = line.chars().collect();

        let splitter_indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| (c == '^').then_some(i))
            .collect();
        for splitter in splitter_indices {
            if beams[splitter] {
                beams[splitter - 1] = true;
                beams[splitter] = false;
                beams[splitter + 1] = true;

                splits += 1;
            }
        }
    }

    println!("{splits}");
}
