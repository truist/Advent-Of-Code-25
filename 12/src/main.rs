use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 12
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
struct Present {
    shape: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    targets: Vec<usize>,
}

fn process(data: String) {
    let mut lines = data.lines();
    let presents: Vec<Present> = (0..6).map(|_| parse_present(&mut lines)).collect();
    let regions: Vec<Region> = lines.map(|line| parse_region(line)).collect();
}

fn parse_present<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Present {
    let _skip = lines.next();
    let bools = (0..3)
        .map(|_| lines.next().unwrap())
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect();
    let _skip = lines.next();

    Present { shape: bools }
}

fn parse_region(line: &str) -> Region {
    let mut vals = line.split_whitespace();

    let (width, height) = vals
        .next()
        .unwrap()
        .trim_end_matches(":")
        .split_once("x")
        .unwrap();
    let (width, height) = (width.parse().unwrap(), height.parse().unwrap());

    let targets = vals.map(|val| val.parse().unwrap()).collect();

    Region {
        width,
        height,
        targets,
    }
}
