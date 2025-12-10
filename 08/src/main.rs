use clap::Parser;
use std::{fmt, fs, path::PathBuf};

/// AOC 25 day 08
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
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
    closest_index: usize,
    closest_distance: f64,
}

impl fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
struct Circuit<'a> {
    box_indices: Vec<usize>,
    boxes: &'a Vec<JunctionBox>,
}

impl<'a> fmt::Display for Circuit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for i in self.box_indices.iter() {
            write!(f, "{}", self.boxes[*i])?;
        }
        write!(f, ")")
    }
}

impl<'a> Circuit<'a> {
    fn contains(&self, jb: &usize) -> bool {
        self.box_indices.contains(jb)
    }

    fn push(&mut self, jb: usize) {
        self.box_indices.push(jb);
    }
}

fn process(data: String) {
    let mut boxes: Vec<JunctionBox> = data
        .lines()
        .map(|line| {
            let mut split = line.splitn(3, ",");
            JunctionBox {
                x: split.next().unwrap().parse().unwrap(),
                y: split.next().unwrap().parse().unwrap(),
                z: split.next().unwrap().parse().unwrap(),
                closest_index: usize::MAX,
                closest_distance: f64::MAX,
            }
        })
        .collect();

    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            let distance = boxes[i].distance_to(&boxes[j]);
            if distance < boxes[i].closest_distance {
                boxes[i].closest_distance = distance;
                boxes[i].closest_index = j;
            }
        }
    }

    // 'clever' way to avoid sorting `boxes` itself
    let mut sorted_box_indices: Vec<usize> = (0..boxes.len()).collect();
    sorted_box_indices.sort_by(|&i, &j| {
        boxes[i]
            .closest_distance
            .total_cmp(&boxes[j].closest_distance)
    });

    let mut circuits: Vec<Circuit> = vec![];

    'outer: for i in sorted_box_indices {
        let closest_index = boxes[i].closest_index;

        // check if we need to add it to an existing circuit
        for circuit in circuits.iter_mut() {
            if circuit.contains(&i) {
                // println!("circuit {} already contains {}", circuit, boxes[i]);
                continue 'outer;
            }
            if circuit.contains(&closest_index) {
                println!("circuit {} is closest to {}", circuit, boxes[i]);
                circuit.push(i);
                continue 'outer;
            }
        }

        // nope, so make a new circuit
        let new_circuit = Circuit {
            box_indices: vec![i, closest_index],
            boxes: &boxes,
        };
        println!("made a new circuit {} for {}", new_circuit, boxes[i]);
        circuits.push(new_circuit);
    }

    // println!("{circuits:?}");
}
