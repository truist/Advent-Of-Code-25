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
            let strval = id.to_string();
            let len = strval.len();
            if len % 2 != 0 {
                continue;
            }

            let left = &mut strval[0..len / 2].chars();
            let right = &mut strval[len / 2..].chars();

            while let Some(lchar) = left.next() {
                let rchar = right.next().unwrap();
                if lchar.ne(&rchar) {
                    continue 'outer;
                }
            }
            invalids += id;
        }
    }

    println!("{invalids}");
}
