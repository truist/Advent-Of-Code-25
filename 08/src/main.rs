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

#[derive(Debug, PartialEq)]
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
    boxes: Vec<&'a JunctionBox>,
}

impl<'a> fmt::Display for Circuit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for jbox in self.boxes.iter() {
            write!(f, "{}", jbox)?;
        }
        write!(f, ")")
    }
}

impl<'a> Circuit<'a> {
    fn contains(&self, jbox: &JunctionBox) -> bool {
        self.boxes.contains(&jbox)
    }

    fn push(&mut self, jbox: &'a JunctionBox) {
        self.boxes.push(jbox);
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

    let mut circuits: Vec<Circuit> = vec![];

    'outer: for jbox in boxes.iter() {
        // check if we need to add it to an existing circuit
        let closest = &boxes[jbox.closest_index];
        for circuit in circuits.iter_mut() {
            if circuit.contains(closest) {
                println!("circuit {} is closest to {}", circuit, jbox);
                circuit.push(jbox);
                continue 'outer;
            }
        }

        // nope, so make a new circuit
        let new_circuit = Circuit { boxes: vec![jbox] };
        println!("made a new circuit {} for {}", new_circuit, jbox);
        circuits.push(new_circuit);
    }

    for circuit in circuits {
        println!("{circuit}");
    }
}
