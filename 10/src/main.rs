use std::collections::{HashSet, VecDeque};
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
    joltages: Vec<usize>,
    press_count: usize,
}

fn find_min(machine: &Machine) -> usize {
    let mut seen: HashSet<Vec<usize>> = HashSet::new();

    let start_state = State {
        joltages: vec![0; machine.joltages.len()],
        press_count: 0,
    };

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(start_state);

    let mut max_press = 0;
    while let Some(state) = queue.pop_front() {
        if state.press_count > max_press {
            max_press += 1;
            println!(
                "max_press: {max_press}; queue size: {}; cache size: {}",
                queue.len(),
                seen.len()
            );
        }

        for next_state in push_buttons(&state, &machine) {
            match next_state {
                NextState::Answer(val) => return val,
                NextState::Queue(state) => {
                    if seen.insert(state.joltages.clone()) {
                        queue.push_back(state);
                    }
                }
            }
        }
    }

    panic!("It shouldn't be possible to get here without finding an answer first");
}

enum NextState {
    Answer(usize),
    Queue(State),
}

fn push_buttons(state: &State, machine: &Machine) -> Vec<NextState> {
    let mut next_states = vec![];
    for button in machine.buttons.iter() {
        let mut state = state.clone();

        state.press_count += 1;

        let mut overflow = false;
        for i in button.iter() {
            state.joltages[*i] += 1;

            if state.joltages[*i] > machine.joltages[*i] {
                overflow = true;
            }
        }
        if overflow {
            continue;
        }

        if machine.joltages == state.joltages {
            println!("one machine done: {}", state.press_count);
            next_states.push(NextState::Answer(state.press_count));
        } else {
            next_states.push(NextState::Queue(state));
        }
    }
    next_states
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
