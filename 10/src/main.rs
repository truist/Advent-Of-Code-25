use std::collections::{HashSet, VecDeque};
use std::io::{self, Write};
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
    // lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn process(data: String) {
    let machines = parse_machines(data);

    let mut answer = 0;
    for m in 0..machines.len() {
        let machine = &machines[m];
        println!("Starting machine {m} ({:?}) ", machine.joltages);
        answer += find_min(machine);
    }
    println!("{answer}");
}

#[derive(Clone, Debug)]
struct State {
    joltages: Vec<usize>,
    press_count: usize,
}

fn find_min(machine: &Machine) -> usize {
    let maps = map_joltages_to_buttons(machine);

    let mut start_states = vec![State {
        joltages: vec![0; machine.joltages.len()],
        press_count: 0,
    }];
    for map in maps {
        let new_starts =
            find_min_targeted(machine, start_states, map.joltage_index, map.button_indexes);
        print!("{}", new_starts.len());

        // let's assume/hope that only the shortest path to the intermediate states are needed...
        // let min_presses = new_starts
        //     .iter()
        //     .min_by_key(|state| state.press_count)
        //     .unwrap()
        //     .press_count;
        // start_states = new_starts
        //     .into_iter()
        //     .filter(|state| state.press_count == min_presses)
        //     .collect();
        start_states = new_starts;
    }

    let min = start_states
        .iter()
        .filter_map(|state| {
            if state.joltages == machine.joltages {
                Some(state.press_count)
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("\n{min}");
    min
}

fn find_min_targeted(
    machine: &Machine,
    start_states: Vec<State>,
    target_joltage: usize,
    button_set: Vec<usize>,
) -> Vec<State> {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.extend(start_states.clone());

    let mut seen: HashSet<Vec<usize>> = HashSet::new();

    let mut partial_states: Vec<State> = vec![];

    let mut max_press = 0;
    while let Some(state) = queue.pop_front() {
        if state.press_count > max_press {
            max_press = state.press_count;
            print!(".");
            io::stdout().flush().unwrap();
        }

        for next_state in push_buttons(&state, &machine, &target_joltage, &button_set) {
            match next_state {
                NextState::OneTarget(state) => {
                    if seen.insert(state.joltages.clone()) {
                        partial_states.push(state);
                    }
                }
                NextState::Queue(state) => {
                    if seen.insert(state.joltages.clone()) {
                        queue.push_back(state);
                    }
                }
            }
        }
    }

    if partial_states.len() == 0 {
        start_states
    } else {
        partial_states
    }
}

enum NextState {
    OneTarget(State),
    Queue(State),
}

fn push_buttons(
    state: &State,
    machine: &Machine,
    target_joltage: &usize,
    button_filter: &Vec<usize>,
) -> Vec<NextState> {
    let mut next_states = vec![];

    for button_index in 0..machine.buttons.len() {
        if !button_filter.contains(&button_index) {
            continue;
        }
        let button = &machine.buttons[button_index];

        let mut new_state = state.clone();

        new_state.press_count += 1;

        let mut overflow = false;
        for joltage_index in button.iter() {
            new_state.joltages[*joltage_index] += 1;

            if new_state.joltages[*joltage_index] > machine.joltages[*joltage_index] {
                overflow = true;
            }
        }
        if overflow {
            continue;
        }

        if machine.joltages[*target_joltage] == new_state.joltages[*target_joltage] {
            next_states.push(NextState::OneTarget(new_state));
        } else {
            next_states.push(NextState::Queue(new_state));
        }
    }
    next_states
}

#[derive(Debug)]
struct JoltageButtonMap {
    joltage_index: usize,
    button_indexes: Vec<usize>,
}

// idea credit to michelkraemer at https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt9h7qw/
fn map_joltages_to_buttons(machine: &Machine) -> Vec<JoltageButtonMap> {
    let mut maps: Vec<JoltageButtonMap> = vec![];
    for joltage_index in 0..machine.joltages.len() {
        maps.push(JoltageButtonMap {
            joltage_index,
            button_indexes: vec![],
        });
    }

    for button_index in 0..machine.buttons.len() {
        for joltage_index in machine.buttons[button_index].iter() {
            maps[*joltage_index].button_indexes.push(button_index);
        }
    }

    maps.sort_by(|a, b| {
        a.button_indexes
            .len()
            .cmp(&b.button_indexes.len())
            .then_with(|| machine.joltages[b.joltage_index].cmp(&machine.joltages[a.joltage_index]))
    });

    maps
}

fn parse_machines(data: String) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    for line in data.lines() {
        let spec: Vec<&str> = line.split(" ").collect();

        // let lights: Vec<bool> = extract_values(spec[0], false)
        //     .iter()
        //     .map(|s| if s == "#" { true } else { false })
        //     .collect();

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
            // lights,
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
