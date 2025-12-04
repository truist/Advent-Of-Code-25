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

const JOLT_LEN: usize = 12;

fn process(data: String) {
    let mut total = 0;
    for bank in data.lines() {
        let chars: Vec<char> = bank.chars().collect();

        let mut vals = vec![];
        let mut next_start = 0;
        for i in 0..JOLT_LEN {
            let stopping_point = chars.len() - (JOLT_LEN - 1 - i);
            let mut max_val = 0;
            let mut max_index = chars.len();
            for i in next_start..stopping_point {
                let val = chars[i].to_digit(10).unwrap();
                if val > max_val {
                    max_val = val;
                    max_index = i;
                }
            }

            vals.push(max_val);
            next_start = max_index + 1;
        }

        let combined_val: u64 = vals
            .iter()
            .map(|val| val.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        total += combined_val;
    }
    println!("{total}");
}
