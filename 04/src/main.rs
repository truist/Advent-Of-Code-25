use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 04
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
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut total_rolls = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '@' && adjacent_rolls(&grid, r as isize, c as isize) < 4 {
                total_rolls += 1;
                // print!("x");
            } else {
                // print!("{}", grid[r][c]);
            }
        }
        // println!("");
    }
    println!("{total_rolls}");
}

fn adjacent_rolls(grid: &Vec<Vec<char>>, r: isize, c: isize) -> usize {
    // println!("Evaluating {r},{c}: {}", grid[r as usize][c as usize]);
    let mut adjacent: isize = -1; // -1 because we're going to end up counting ourselves
    for ro in -1isize..=1 {
        for co in -1isize..=1 {
            // println!("  checking {},{}", r + ro, c + co);
            if is_roll(grid, r + ro, c + co) {
                // println!("    {},{} is a roll", r + ro, c + co);
                adjacent += 1;
            }
        }
    }
    adjacent as usize
}

fn is_roll(grid: &Vec<Vec<char>>, r: isize, c: isize) -> bool {
    r >= 0
        && r < grid.len() as isize
        && c >= 0
        && c < grid[0].len() as isize
        && grid[r as usize][c as usize] == '@'
}
