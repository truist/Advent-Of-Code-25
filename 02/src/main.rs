use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 02
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
    let mut invalids = 0;

    for range in data.split(',') {
        let mut ids = range.trim().split('-');
        let start: usize = ids.next().unwrap().parse().unwrap();
        let end: usize = ids.next().unwrap().parse().unwrap();

        'outer: for id in start..=end {
            let strval: Vec<char> = id.to_string().chars().collect();
            let len = strval.len();

            'div: for divisor in [2, 3, 5, 7] {
                if len % divisor != 0 {
                    continue;
                }
                let segment_len = len / divisor;

                for segment in 1..divisor {
                    for i in 0..len / divisor {
                        if strval[i] != strval[segment * segment_len + i] {
                            continue 'div;
                        }
                    }
                }

                invalids += id;
                continue 'outer;
            }
        }
    }

    println!("{invalids}");
}
