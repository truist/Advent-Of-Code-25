use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 09
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

#[derive(Debug, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
}

fn process(data: String) {
    let reds: Vec<Tile> = data
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut largest = 0;
    for i in 0..reds.len() - 2 {
        for j in i + 2..reds.len() {
            let (first, second) = (&reds[i], &reds[j]);
            if !detect_obstacles(first, second, &reds) {
                let area = (second.x.abs_diff(first.x) + 1) * (second.y.abs_diff(first.y) + 1);
                if area > largest {
                    println!("new largest ({area}): {first:?}, {second:?}");
                    largest = area;
                }
            }
        }
    }
}

fn detect_obstacles(first: &Tile, second: &Tile, reds: &Vec<Tile>) -> bool {
    let leftest = first.x.min(second.x);
    let rightest = first.x.max(second.x);
    let highest = first.y.min(second.y);
    let lowest = first.y.max(second.y);

    for i in 0..reds.len() {
        let first = &reds[i];
        let second = &reds[(i + 1) % reds.len()];

        if leftest < first.x && first.x < rightest && highest < first.y && first.y < lowest {
            return true;
        }

        if first.x == second.x
            && between(leftest, first.x, rightest)
            && outside(first.y, second.y, highest, lowest)
        {
            return true;
        }

        if first.y == second.y
            && between(highest, first.y, lowest)
            && outside(first.x, second.x, leftest, rightest)
        {
            return true;
        }
    }

    false
}

fn between(a: usize, b: usize, c: usize) -> bool {
    (a < b && b < c) || (a > b && b > c)
}

fn outside(first: usize, second: usize, edge1: usize, edge2: usize) -> bool {
    (first <= edge1 && second >= edge2) || (second <= edge1 && first >= edge2)
}
