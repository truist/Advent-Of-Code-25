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

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Area {
    area: usize,
    tile_a_idx: usize,
    tile_b_idx: usize,
}

fn process(data: String) {
    let red_tiles: Vec<Tile> = data
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut areas: Vec<Area> = vec![];
    for i in 0..red_tiles.len() - 1 {
        for j in i + 1..red_tiles.len() {
            let area = (red_tiles[i].x.abs_diff(red_tiles[j].x) + 1)
                * (red_tiles[i].y.abs_diff(red_tiles[j].y) + 1);
            areas.push(Area {
                area,
                tile_a_idx: i,
                tile_b_idx: j,
            })
        }
    }

    areas.sort_by_key(|area| area.area);
    println!("{}", areas.pop().unwrap().area);
}
