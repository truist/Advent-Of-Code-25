use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 06
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

#[derive(Debug)]
struct Problem {
    nums: Vec<usize>,
    op: char,
}

impl Problem {
    fn result(&self) -> usize {
        match self.op {
            '+' => self.nums.iter().sum(),
            '*' => self.nums.iter().product(),
            _ => panic!("Unexpected operator: {}", self.op),
        }
    }
}

fn process(data: String) {
    let mut lines = data.lines();
    let row1: Vec<char> = lines.next().unwrap().chars().collect();
    let cols = row1.len();

    let mut char_grid: Vec<Vec<char>> = vec![row1];
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        char_grid.push(line.unwrap().chars().collect());
    }

    let mut problems: Vec<Problem> = vec![];

    let mut nums: Vec<usize> = vec![];
    for c in (0..cols).rev() {
        let mut val = "".to_string();
        for r in 0..char_grid.len() {
            let each_char = char_grid[r][c];
            match each_char {
                ' ' => continue,
                '*' | '+' => {
                    nums.push(val.parse().unwrap());
                    problems.push(Problem {
                        nums: nums,
                        op: each_char,
                    });

                    nums = vec![];
                    val = "".to_string();
                }
                _ => val.push(each_char),
            };
        }
        if val.len() > 0 {
            nums.push(val.parse().unwrap());
        }
    }

    let total: usize = problems.iter().map(|problem| problem.result()).sum();
    println!("{total}");
}
