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
enum Corner {
    Unknown,
    NW,
    NE,
    SE,
    SW,
}

#[derive(Debug, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
    corner: Corner,
}

fn process(data: String) {
    let mut reds: Vec<Tile> = data
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                corner: Corner::Unknown,
            }
        })
        .collect();

    // I ran this first to verify that input.txt corners are clockwise, because the sum was positive,
    // per ChatGPT
    // let mut sum: isize = 0;
    // for i in 0..reds.len() {
    //     let j = (i + 1) % reds.len();
    //     sum += (reds[i].x * reds[j].y) as isize - (reds[j].x * reds[i].y) as isize;
    // }
    // println!("{sum}");

    for i in 0..reds.len() {
        let prev_idx = if i == 0 { reds.len() - 1 } else { i - 1 };
        let next_idx = (i + 1) % reds.len();
        let [prev, tile, next] = reds.get_disjoint_mut([prev_idx, i, next_idx]).unwrap();
        let (px, py, nx, ny) = (
            sigdiff(tile.x, prev.x),
            sigdiff(tile.y, prev.y),
            sigdiff(next.x, tile.x),
            sigdiff(next.y, tile.y),
        );
        // these are correct for clockwise; they would be wrong for counter-clockwise
        tile.corner = match (px, py, nx, ny) {
            (1, 0, 0, 1) => Corner::NE,   // right then down
            (1, 0, 0, -1) => Corner::NW,  // right then up
            (-1, 0, 0, 1) => Corner::SE,  // left then down
            (-1, 0, 0, -1) => Corner::SW, // left then up
            (0, 1, 1, 0) => Corner::NE,   // down then right
            (0, 1, -1, 0) => Corner::SE,  // down then left
            (0, -1, 1, 0) => Corner::NW,  // up then right
            (0, -1, -1, 0) => Corner::SW, // up then left
            _ => panic!("Unexpected tile relationship! {px}, {py}, {nx}, {ny}"),
        };
    }
    // println!("{reds:#?}");

    let mut largest = 0;
    for i in 0..reds.len() - 2 {
        for j in i + 2..reds.len() {
            let (first, second) = (&reds[i], &reds[j]);
            if !detect_inside_corners(first, second, &reds)
                && !detect_intersections(first, second, &reds)
            {
                let area = (second.x.abs_diff(first.x) + 1) * (second.y.abs_diff(first.y) + 1);
                // println!("no inside corners: {first:?}, {second:?}: {area}");
                if area > largest {
                    println!("new largest ({area}): {first:?}, {second:?}");
                    if area == DEBUG_AREA {
                        println!(
                            "new largest debug: {:#?}->{:#?}->{:#?}->{:#?}->{:#?}, {:#?}->{:#?}->{:#?}->{:#?}->{:#?}",
                            &reds[i - 2],
                            &reds[i - 1],
                            first,
                            &reds[i + 1],
                            &reds[i + 2],
                            &reds[j - 2],
                            &reds[j - 1],
                            second,
                            &reds[j + 1],
                            &reds[j + 2],
                        );
                    }
                    largest = area;
                }
            }
        }
    }
}

fn sigdiff(next: usize, prev: usize) -> isize {
    (next as isize - prev as isize).signum()
}

const DEBUG_AREA: usize = 4647960552;
fn detect_inside_corners(first: &Tile, second: &Tile, reds: &Vec<Tile>) -> bool {
    let area = (second.x.abs_diff(first.x) + 1) * (second.y.abs_diff(first.y) + 1);
    if area == DEBUG_AREA {
        println!("considering {first:?} and {second:?}");
    }
    let leftest = first.x.min(second.x);
    let rightest = first.x.max(second.x);
    let highest = first.y.min(second.y);
    let lowest = first.y.max(second.y);
    if area == DEBUG_AREA {
        println!("{leftest}-{rightest}, {highest}-{lowest}");
    }

    if leftest == rightest || highest == lowest {
        return false;
    }

    for each in reds {
        if each == first || each == second {
            // println!("self");
            continue;
        }
        if each.x < leftest || each.x > rightest || each.y < highest || each.y > lowest {
            // println!("outside");
            continue;
        }
        if area == DEBUG_AREA {
            println!("{each:?}");
        }

        if leftest < each.x && each.x < rightest && highest < each.y && each.y < lowest {
            // println!("inside: {each:?}");
            return true;
        }

        if leftest == each.x && (each.corner == Corner::SE || each.corner == Corner::NE) {
            // println!("west: {each:?}");
            return true;
        }
        if rightest == each.x && (each.corner == Corner::SW || each.corner == Corner::NW) {
            // println!("east: {each:?}");
            return true;
        }
        if highest == each.y && (each.corner == Corner::SW || each.corner == Corner::SE) {
            // println!("north: {each:?}");
            return true;
        }
        if lowest == each.y && (each.corner == Corner::NW || each.corner == Corner::NE) {
            // println!("south: {each:?}");
            return true;
        }

        if area == DEBUG_AREA {
            println!("one got through!: {each:?}");
        }
    }
    false
}
fn detect_intersections(first: &Tile, second: &Tile, reds: &Vec<Tile>) -> bool {
    let leftest = first.x.min(second.x);
    let rightest = first.x.max(second.x);
    let highest = first.y.min(second.y);
    let lowest = first.y.max(second.y);

    for i in 0..reds.len() {
        let first = &reds[i];
        let second = &reds[(i + 1) % reds.len()];

        if first.x == second.x
            && between(leftest, first.x, rightest)
            && first.y <= highest
            && second.y >= lowest
        {
            println!("vertical edge");
            return true;
        }

        if first.y == second.y
            && between(highest, first.y, lowest)
            && first.x <= leftest
            && second.x >= rightest
        {
            println!("horizontal edge");
            return true;
        }
    }

    false
}
fn between(a: usize, b: usize, c: usize) -> bool {
    (a < b && b < c) || (a > b && b > c)
}
