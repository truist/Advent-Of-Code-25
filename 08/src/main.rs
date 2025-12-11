use clap::Parser;
use std::{fs, path::PathBuf};

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
struct Distance {
    left_idx: usize,
    right_idx: usize,
    distance: f64,
}

#[derive(Debug)]
struct Circuit {
    boxes: Vec<usize>,
}

impl Circuit {
    fn append(&mut self, circuit: &mut Circuit) {
        self.boxes.append(&mut circuit.boxes)
    }

    fn contains(&self, jbox_idx: &usize) -> bool {
        self.boxes.contains(&jbox_idx)
    }

    fn len(&self) -> usize {
        self.boxes.len()
    }

    fn push(&mut self, jbox_idx: usize) {
        self.boxes.push(jbox_idx);
    }
}

fn process(data: String) {
    let boxes: Vec<JunctionBox> = data
        .lines()
        .map(|line| {
            let mut split = line.splitn(3, ",");
            JunctionBox {
                x: split.next().unwrap().parse().unwrap(),
                y: split.next().unwrap().parse().unwrap(),
                z: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut distances: Vec<Distance> = vec![];
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            distances.push(Distance {
                left_idx: i,
                right_idx: j,
                distance: boxes[i].distance_to(&boxes[j]),
            });
        }
    }
    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut circuits: Vec<Circuit> = vec![];
    for distance in distances {
        let left_circuit_idx = circuits.iter().position(|c| c.contains(&distance.left_idx));
        let right_circuit_idx = circuits
            .iter()
            .position(|c| c.contains(&distance.right_idx));
        match (left_circuit_idx, right_circuit_idx) {
            (Some(lci), Some(rci)) => {
                if lci != rci {
                    let (earlier, later) = if lci < rci { (lci, rci) } else { (rci, lci) };
                    let (first_part, last_part) = circuits.split_at_mut(later);
                    let left_circuit = &mut first_part[earlier];
                    let right_circuit = &mut last_part[0];
                    left_circuit.append(right_circuit);

                    circuits.remove(later);
                }
            }
            (Some(lci), None) => {
                circuits[lci].push(distance.right_idx);
            }
            (None, Some(rci)) => {
                circuits[rci].push(distance.left_idx);
            }
            (None, None) => {
                let new_circuit = Circuit {
                    boxes: vec![distance.left_idx, distance.right_idx],
                };
                circuits.push(new_circuit);
            }
        };
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            println!(
                "{}",
                boxes[distance.left_idx].x * boxes[distance.right_idx].x
            );
            break;
        }
    }
}
