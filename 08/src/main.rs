use clap::Parser;
use std::{fmt, fs, path::PathBuf};

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

    fn find_containing_circuit<'a>(
        &'a self,
        circuits: &'a mut [Circuit<'a>],
    ) -> Option<&'a mut Circuit> {
        for circuit in circuits.iter_mut() {
            if circuit.contains(self) {
                return Some(circuit);
            }
        }
        return None;
    }
}

struct Distance<'a> {
    left: &'a JunctionBox,
    right: &'a JunctionBox,
    distance: f64,
}

impl<'a> fmt::Display for Distance<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.distance, self.left, self.right)
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
        write!(f, ") ({})", self.len())
    }
}

impl<'a> Circuit<'a> {
    fn append(&mut self, circuit: &mut Circuit<'a>) {
        self.boxes.append(&mut circuit.boxes)
    }

    fn contains(&self, jbox: &JunctionBox) -> bool {
        self.boxes.contains(&jbox)
    }

    fn len(&self) -> usize {
        self.boxes.len()
    }

    fn push(&mut self, jbox: &'a JunctionBox) {
        self.boxes.push(jbox);
    }
}

fn process(data: String, connections: usize) {
    let mut boxes: Vec<JunctionBox> = data
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
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            let distance = boxes[i].distance_to(&boxes[j]);
            distances.push(Distance {
                left: &boxes[i],
                right: &boxes[j],
                distance,
            });
        }
    }
    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    for distance in &distances {
        println!("{distance}");
    }

    let mut circuits: Vec<Circuit> = vec![];

    let mut count = 0;
    for distance in &distances {
        println!("count: {count}");
        print_circuits(&circuits);
        if count == connections {
            break;
        }

        // check if we need to add it to an existing circuit
        if let Some(&mut left_circuit) = distance.left.find_containing_circuit(&mut circuits) {
            println!(
                "circuit {} already contains left {}",
                left_circuit, distance.left
            );

            if let Some(&mut right_circuit) = distance.right.find_containing_circuit(&mut circuits)
            {
                println!(
                    "circuit {} already contains right {}",
                    right_circuit, distance.right
                );

                if std::ptr::eq(&left_circuit, &right_circuit) {
                    println!("the two circuits are the same, so there's nothing to do");

                    continue;
                } else {
                    println!("the two circuits are different, and have to be merged");

                    left_circuit.append(&mut right_circuit);
                    circuits.remove(
                        circuits
                            .iter()
                            .position(|circuit| std::ptr::eq(circuit, &right_circuit))
                            .unwrap(),
                    );
                    println!("merged: {}", left_circuit);
                    count += 1;
                }
            } else {
                println!("adding {} to {}", distance.right, left_circuit);

                left_circuit.push(distance.right);
                count += 1;
            }
        } else if let Some(&mut right_circuit) =
            distance.right.find_containing_circuit(&mut circuits)
        {
            println!(
                "circuit {} already contains right {}",
                right_circuit, distance.right
            );

            println!("adding {} to {}", distance.left, right_circuit);
            right_circuit.push(distance.left);
            count += 1;
        } else {
            let new_circuit = Circuit {
                boxes: vec![distance.left, distance.right],
            };
            println!(
                "both boxes are outside a circuit; making one: {}",
                new_circuit
            );
            circuits.push(new_circuit);
            count += 1;
        }
    }

    print_circuits(&circuits);
}

fn print_circuits(circuits: &Vec<Circuit>) {
    // circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    for circuit in circuits.iter() {
        println!("{circuit}");
    }
    let answer: usize = circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product();
    println!("answer: {answer}");
}
