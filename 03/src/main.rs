use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 03
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
    let mut total = 0;
    for bank in data.lines() {
        let chars: Vec<char> = bank.chars().collect();

        let mut max_val = 0;
        let mut max_index = chars.len();
        for i in (0..chars.len() - 1).rev() {
            let val = chars[i].to_digit(10).unwrap();
            if val >= max_val {
                max_val = val;
                max_index = i;
            }
        }
        let first_val = max_val;

        max_val = 0;
        for i in max_index + 1..chars.len() {
            let val = chars[i].to_digit(10).unwrap();
            if val > max_val {
                max_val = val;
            }
        }
        let second_val = max_val;

        let combined_val: usize = format!("{}{}", first_val.to_string(), second_val.to_string())
            .parse()
            .unwrap();
        total += combined_val;
    }
    println!("{total}");
}
