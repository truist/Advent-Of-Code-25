use std::collections::VecDeque;
use std::{fs, path::PathBuf};

use clap::Parser;

/// AOC 25 day 10
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
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn process(data: String) {
    let machines = parse_machines(data);

    let answer: usize = machines.iter().map(|machine| find_min(machine)).sum();
    println!("{answer}");
}

#[derive(Clone)]
struct State {
    lights: Vec<bool>,
    press_count: usize,
}

fn find_min(machine: &Machine) -> usize {
    let start_state = State {
        lights: vec![false; machine.lights.len()],
        press_count: 0,
    };

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(start_state);

    while let Some(state) = queue.pop_front() {
        for button in machine.buttons.iter() {
            let mut state = state.clone();

            state.press_count += 1;
            for light in button.iter() {
                state.lights[*light] = !state.lights[*light];
            }

            if machine.lights == state.lights {
                return state.press_count;
            } else {
                queue.push_back(state);
            }
        }
    }

    0 // can't ever get here
}

fn parse_machines(data: String) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    for line in data.lines() {
        let spec: Vec<&str> = line.split(" ").collect();

        let lights: Vec<bool> = extract_values(spec[0], false)
            .iter()
            .map(|s| if s == "#" { true } else { false })
            .collect();

        let buttons: Vec<Vec<usize>> = spec[1..spec.len() - 1]
            .iter()
            .map(|val| {
                extract_values(val, true)
                    .iter()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        let joltages: Vec<usize> = extract_values(spec[spec.len() - 1], true)
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        machines.push(Machine {
            lights,
            buttons,
            joltages,
        })
    }
    machines
}

fn extract_values(val: &str, split: bool) -> Vec<String> {
    let inner: String = val.chars().skip(1).take(val.len() - 2).collect();
    if split {
        inner.split(",").map(|s| s.to_string()).collect()
    } else {
        inner.chars().map(|c| c.to_string()).collect()
    }
}
