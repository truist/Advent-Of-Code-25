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
struct Problem<'a> {
    nums: Vec<usize>,
    op: &'a str,
}

impl<'a> Problem<'a> {
    fn result(&self) -> usize {
        match self.op {
            "+" => self.nums.iter().sum(),
            "*" => self.nums.iter().product(),
            _ => panic!("Unexpected operator: {}", self.op),
        }
    }
}

fn process(data: String) {
    let mut lines = data.lines();
    let row1: Vec<&str> = lines.next().unwrap().trim().split_whitespace().collect();
    let cols = row1.len();
    let mut rows: Vec<Vec<&str>> = vec![row1];
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        rows.push(line.unwrap().trim().split_whitespace().collect());
    }

    let mut problems: Vec<Problem> = vec![];
    for c in 0..cols {
        let mut nums: Vec<usize> = vec![];
        for r in 0..rows.len() - 1 {
            nums.push(rows[r][c].parse().unwrap());
        }
        problems.push(Problem {
            nums,
            op: rows[rows.len() - 1][c],
        });
    }

    let total: usize = problems.iter().map(|problem| problem.result()).sum();
    println!("{total}");
}
