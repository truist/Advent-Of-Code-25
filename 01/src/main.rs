use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 01
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
    let mut pointer = 50 as isize;
    let mut zeros = 0;

    for line in data.lines() {
        let (i, first) = line.char_indices().next().unwrap();
        let distance: isize = line[i + first.len_utf8()..].parse::<isize>().unwrap() % 100;
        match first {
            'L' => {
                pointer -= distance;
                if pointer < 0 {
                    pointer += 100;
                }
            }
            'R' => {
                pointer += distance;
                if pointer > 99 {
                    pointer -= 100;
                }
            }
            _ => {
                panic!("Unexpected value in {line}");
            }
        }
        if 0 == pointer {
            zeros += 1;
        }
    }
    println!("{zeros}");
}
