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

    let mut timelines = vec![0; cols];

    timelines[chars.iter().position(|&c| c == 'S').unwrap()] = 1;

    while let Some(line) = lines.next() {
        chars = line.chars().collect();

        let splitter_indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| (c == '^').then_some(i))
            .collect();
        for splitter in splitter_indices {
            if timelines[splitter] > 0 {
                timelines[splitter - 1] += timelines[splitter];
                timelines[splitter + 1] += timelines[splitter];
                timelines[splitter] = 0;
            }
        }
    }

    println!("{}", timelines.iter().sum::<usize>());
}

/*

.......S....... 1 timeline
.......1.......
......1^1...... adds 1 = 2
......1.1......
.....1^2^1..... left adds 1; right adds 1; = 4
.....1.2.1.....
....1^3^3^1.... left adds 1; middle gets hit twice so adds 2; right adds 1; = 8
....1.3.3.1....
...1^4^331^1... left adds 1; middle gets hit thrice so adds 3; right adds 1; 13
...1.4.331.1...
..1^5^434^2^1.. 20
..1.5.434.2.1..
.1^154^74.21^1. 26
.1.154.74.21.1.
1^2^0^1^1^211^1 40
...............

*/
