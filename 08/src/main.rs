use clap::Parser;
use std::{fs, path::PathBuf};

/// AOC 25 day 08
#[derive(Parser)]
#[command()]
struct Args {
    /// Path to the input file
    input: PathBuf,

    /// Number of connections to make
    connections: usize,
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

    process(contents, args.connections);
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

// impl fmt::Display for JunctionBox {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[{},{},{}]", self.x, self.y, self.z)
//     }
// }

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

fn process(data: String, connections: usize) {
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
    // for (i, jbox) in boxes.iter().enumerate() {
    //     println!("{i}: {jbox}");
    // }

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
    // for distance in &distances {
    //     println!("{distance:?}");
    // }

    let mut circuits: Vec<Circuit> = vec![];

    for i in 0..connections {
        let distance = &distances[i];

        // println!(
        //     "Evaluating boxes {} and {}...",
        //     distance.left_idx, distance.right_idx
        // );
        let left_circuit_idx = circuits.iter().position(|c| c.contains(&distance.left_idx));
        let right_circuit_idx = circuits
            .iter()
            .position(|c| c.contains(&distance.right_idx));
        match (left_circuit_idx, right_circuit_idx) {
            (Some(lci), Some(rci)) => {
                if lci == rci {
                    // println!("they're in the same circuit already",);
                } else {
                    // println!(
                    //     "the two circuits are different ({} and {}), and have to be merged",
                    //     lci, rci
                    // );

                    let (earlier, later) = if lci < rci { (lci, rci) } else { (rci, lci) };
                    let (first_part, last_part) = circuits.split_at_mut(later);
                    let left_circuit = &mut first_part[earlier];
                    let right_circuit = &mut last_part[0];
                    left_circuit.append(right_circuit);

                    circuits.remove(later);
                }
            }
            (Some(lci), None) => {
                // println!("adding box {} to circuit {}", distance.right_idx, lci);

                circuits[lci].push(distance.right_idx);
            }
            (None, Some(rci)) => {
                // println!("adding box {} to circuit {}", distance.left_idx, rci);
                circuits[rci].push(distance.left_idx);
            }
            (None, None) => {
                // println!("making a new circuit",);
                let new_circuit = Circuit {
                    boxes: vec![distance.left_idx, distance.right_idx],
                };
                circuits.push(new_circuit);
            }
        };
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let answer: usize = circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product();
    println!("{answer}");
}
