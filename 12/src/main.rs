use clap::Parser;
use std::io::{self, Write};
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
struct Shape {
    orientations: Vec<Vec<Vec<bool>>>,
}

impl Shape {
    fn new(shape: &Vec<Vec<bool>>) -> Shape {
        Shape {
            orientations: Self::orientations(shape),
        }
    }

    fn orientations(shape: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
        let mut shape = shape.clone();
        let mut orientations = vec![];

        for _ in 0..4 {
            shape = rotate(shape);
            orientations.push(shape.clone());
        }

        shape = flip(shape);
        for _ in 0..4 {
            shape = rotate(shape);
            orientations.push(shape.clone());
        }

        orientations.sort();
        orientations.dedup();

        orientations
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    targets: Vec<usize>,
}

impl Region {
    // turns out I have some prior experience with this type of problem!
    // https://github.com/truist/puzzle/blob/master/solver.js
    fn can_fit(&self, shapes: &Vec<Shape>) -> bool {
        let board = vec![vec![false; self.width]; self.height];
        let placed = vec![0; self.targets.len()];
        self.try_shapes(&board, shapes, 0, 0, &placed)
    }

    fn try_shapes(
        &self,
        board: &Vec<Vec<bool>>,
        shapes: &Vec<Shape>,
        r: usize,
        c: usize,
        placed: &Vec<usize>,
    ) -> bool {
        // println!("{r},{c}");
        println!("{placed:?}");
        for shape_index in 0..shapes.len() {
            if placed[shape_index] < self.targets[shape_index] {
                if self.try_orientations(board, shapes, shape_index, r, c, placed) {
                    return true;
                }
            }
        }

        false
    }

    fn try_orientations(
        &self,
        board: &Vec<Vec<bool>>,
        shapes: &Vec<Shape>,
        shape_index: usize,
        r: usize,
        c: usize,
        placed: &Vec<usize>,
    ) -> bool {
        'orientations: for orientation in &shapes[shape_index].orientations {
            for orientation_r in 0..orientation.len() {
                for orientation_c in 0..orientation[0].len() {
                    if orientation[orientation_r][orientation_c] {
                        if r + orientation_r >= self.height
                            || c + orientation_c >= self.width
                            || board[r + orientation_r][c + orientation_c]
                        {
                            continue 'orientations;
                        }
                    }
                }
            }

            // if we got here, it fit!
            let mut board = board.clone();

            for orientation_r in 0..orientation.len() {
                for orientation_c in 0..orientation[0].len() {
                    if orientation[orientation_r][orientation_c] {
                        board[r + orientation_r][c + orientation_c] = true;
                    }
                }
            }

            let mut placed = placed.clone();
            placed[shape_index] += 1;
            if placed == self.targets {
                return true;
            }

            if self.try_next_location(&board, shapes, r, c, &placed) {
                return true;
            }
        }

        false
    }

    fn try_next_location(
        &self,
        board: &Vec<Vec<bool>>,
        shapes: &Vec<Shape>,
        mut r: usize,
        mut c: usize,
        placed: &Vec<usize>,
    ) -> bool {
        loop {
            c += 1;
            if c == self.width {
                c = 0;
                r += 1;
                if r == self.height {
                    return false;
                }
            }

            if !board[r][c] {
                if self.try_shapes(board, shapes, r, c, placed) {
                    return true;
                }
            }
        }
    }
}

fn process(data: String) {
    let mut lines = data.lines();
    let shapes: Vec<Shape> = (0..6).map(|_| parse_present(&mut lines)).collect();
    let regions: Vec<Region> = lines.map(|line| parse_region(line)).collect();

    let mut can_fit = 0;
    for region in regions {
        print!("{region:?}: ");
        io::stdout().flush().unwrap();
        if region.can_fit(&shapes) {
            can_fit += 1;
            println!("yes");
        } else {
            println!("no");
        }
    }

    println!("{can_fit}");
}

fn rotate(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rotated = vec![vec![false; 3]; 3];

    for r in 0..3 {
        for c in 0..3 {
            rotated[2 - c][r] = shape[r][c];
        }
    }

    rotated
}

fn flip(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut flipped = vec![vec![false; 3]; 3];

    for r in 0..3 {
        for c in 0..3 {
            flipped[r][2 - c] = shape[r][c];
        }
    }

    flipped
}

fn parse_present<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Shape {
    let _skip = lines.next();
    let bools = (0..3)
        .map(|_| lines.next().unwrap())
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect();
    let _skip = lines.next();

    Shape::new(&bools)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_bools(row1: &str, row2: &str, row3: &str) -> Vec<Vec<bool>> {
        vec![row1, row2, row3]
            .iter()
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect()
    }

    #[test]
    #[rustfmt::skip]
    fn test_rotate() {
        let original = str_to_bools(
            "##.",
            "...",
            "..#",
        );

        let expected = str_to_bools(
            "..#",
            "#..",
            "#..",
        );

        assert_eq!(expected, rotate(original), "rotation works");
    }

    #[test]
    #[rustfmt::skip]
    fn test_flip() {
        let original = str_to_bools(
            "##.",
            "...",
            "..#",
        );

        let expected = str_to_bools(
            ".##",
            "...",
            "#..",
        );

        assert_eq!(expected, flip(original), "rotation works");
    }

    #[test]
    #[rustfmt::skip]
    fn test_orientations() {
        let original = str_to_bools(
            "##.",
            "...",
            "..#",
        );
        let expected_orientations = vec![
            str_to_bools(
                "##.",
                "...",
                "..#",
            ),
            str_to_bools(
                ".##",
                "...",
                "#..",
            ),

            str_to_bools(
                "..#",
                "..#",
                "#..",
            ),
            str_to_bools(
                "#..",
                "..#",
                "..#",
            ),

            str_to_bools(
                "#..",
                "...",
                ".##",
            ),
            str_to_bools(
                "..#",
                "...",
                "##.",
            ),

            str_to_bools(
                "..#",
                "#..",
                "#..",
            ),
            str_to_bools(
                "#..",
                "#..",
                "..#",
            ),
        ];
        check(expected_orientations, Shape::orientations(&original), "all possible");


        let original = str_to_bools(
            ".#.",
            "..#",
            "...",
        );
        let expected_orientations = vec![
            str_to_bools(
                ".#.",
                "..#",
                "...",
            ),
            str_to_bools(
                "...",
                "..#",
                ".#.",
            ),
            str_to_bools(
                "...",
                "#..",
                ".#.",
            ),
            str_to_bools(
                ".#.",
                "#..",
                "...",
            ),
        ];
        check(expected_orientations, Shape::orientations(&original), "flips match rotations");

        let original = str_to_bools(
            "#..",
            ".#.",
            "..#",
        );
        let expected_orientations = vec![
            str_to_bools(
                "#..",
                ".#.",
                "..#",
            ),
            str_to_bools(
                "..#",
                ".#.",
                "#..",
            ),
        ];
        check(expected_orientations, Shape::orientations(&original), "only two");

    }

    fn check(expecteds: Vec<Vec<Vec<bool>>>, actuals: Vec<Vec<Vec<bool>>>, desc: &str) {
        let mut match_count = 0;
        'expected: for expected in &expecteds {
            for actual in &actuals {
                if *expected == *actual {
                    match_count += 1;
                    continue 'expected;
                }
            }
        }
        assert_eq!(
            expecteds.len(),
            match_count,
            "Got {desc} orientations; here are the actuals: {actuals:#?}"
        );
    }
}
