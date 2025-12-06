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

#[derive(Clone, Debug)]
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

    for line in data.lines() {
        if line == "" {
            break;
        } else {
            let (min, max) = line.split_once("-").unwrap();
            ranges.push(Range {
                min: min.parse().unwrap(),
                max: max.parse().unwrap(),
            });
        }
    }

    loop {
        let (new_ranges, merge_count) = merge_ranges(&ranges);
        ranges = new_ranges;
        if merge_count == 0 {
            break;
        }
    }

    let fresh: u64 = ranges.iter().map(|range| range.max - range.min + 1).sum();
    println!("{fresh}");
}

fn merge_ranges(ranges: &Vec<Range>) -> (Vec<Range>, usize) {
    let mut merged: Vec<Range> = vec![];
    let mut merged_count = 0;
    'range_loop: for range in ranges.iter() {
        let merged_len = merged.len();
        for m in 0..merged_len {
            let merge = &merged[m];
            if merge.includes(range.min) {
                if merge.includes(range.max) {
                    // do nothing; this range is already covered
                    merged_count += 1;
                    continue 'range_loop;
                } else {
                    // merge range and merge
                    merged.push(Range {
                        min: merge.min,
                        max: range.max,
                    });
                    merged.swap_remove(m);
                    merged_count += 1;
                    continue 'range_loop;
                }
            } else if merge.includes(range.max) {
                // merge range and merge
                merged.push(Range {
                    min: range.min,
                    max: merge.max,
                });
                merged.swap_remove(m);
                merged_count += 1;
                continue 'range_loop;
            }
        }
        // if we got here, this range is distinct
        merged.push((*range).clone());
    }
    merged.sort_by(|a, b| a.min.cmp(&b.min));
    (merged, merged_count)
}
