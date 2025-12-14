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
    joltage_targets: Vec<usize>,
}

fn process(data: String) {
    let machines = parse_machines(data);

    let mut answer = 0;
    for m in 0..machines.len() {
        let machine = &machines[m];
        println!("Starting machine {m} ({:?}) ", machine.joltage_targets);

        let maps = map_joltages_to_button_sequence(machine);

        let start_state = State {
            joltages: vec![0; machine.joltage_targets.len()],
            press_count: 0,
        };
        let machine_answer = find_min_by_joltage(machine, &maps, &start_state);
        println!(" -> {machine_answer}\n");
        answer += machine_answer;
    }
    println!("{answer}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    joltages: Vec<usize>,
    press_count: usize,
}

// Permute all possible combinations of button presses.
// Order doesn't matter; just counts (per button).
// Never any need to press a button more than (target - current).
// A button might be pressed anywhere from 0..(target-current).
//
// So e.g. 5 buttons with a target of 200 gives us 200^5 = 320B possible combinations.
// But since we know we're evaluating all possibilities,
// and not trying to find the shortest path first, recursive DFS would work.
// We don't really need to know the "path" - just the button count that gets us there.
//
// Idea credit to michelkraemer at https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt9h7qw/
fn find_min_by_joltage(machine: &Machine, maps: &Vec<JoltageButtonMap>, state: &State) -> usize {
    if maps.len() == 0 {
        if state.joltages == machine.joltage_targets {
            print!(" {}", state.press_count);
            io::stdout().flush().unwrap();
            return state.press_count;
        } else {
            return usize::MAX;
        }
    }

    // println!(" state: {state:?}");
    // println!(" maps: {maps:?}");
    let mut remaining_maps = maps.clone();
    let current_map = remaining_maps.remove(0);

    if current_map.button_indexes.len() == 0 {
        find_min_by_joltage(machine, &remaining_maps, state)
    } else {
        find_min_by_buttons(
            machine,
            state,
            &current_map.button_indexes,
            &current_map.joltage_index,
            &remaining_maps,
        )
    }
}

fn find_min_by_buttons(
    machine: &Machine,
    state: &State,
    button_indexes: &Vec<usize>,
    joltage_index: &usize,
    remaining_maps: &Vec<JoltageButtonMap>,
) -> usize {
    // println!("  buttons: {button_indexes:?}");
    let joltage_target_value = machine.joltage_targets[*joltage_index];
    let joltage_diff = joltage_target_value - state.joltages[*joltage_index];

    let mut remaining_buttons = button_indexes.clone();
    let current_button = &machine.buttons[remaining_buttons.remove(0)];
    // println!("  current_button: {current_button:?}");

    // just an optimization; not strictly necessary
    let mut min_press_to_test = 0;
    if remaining_buttons.len() == 0 {
        min_press_to_test = joltage_diff;
    }

    let mut best_so_far = usize::MAX;
    for press_count in min_press_to_test..=joltage_diff {
        if let Some(new_state) = do_press(machine, state, current_button, press_count) {
            let answer = if remaining_buttons.len() == 0 {
                find_min_by_joltage(machine, remaining_maps, &new_state)
            } else {
                find_min_by_buttons(
                    machine,
                    &new_state,
                    &remaining_buttons,
                    joltage_index,
                    remaining_maps,
                )
            };
            if answer < best_so_far {
                best_so_far = answer;
                // println!("   best_so_far: {best_so_far}");
            }
        }
    }

    // println!("  returning");
    best_so_far
}

fn do_press(
    machine: &Machine,
    state: &State,
    button: &Vec<usize>,
    presses: usize,
) -> Option<State> {
    let mut new_state = state.clone();

    new_state.press_count += presses;

    for joltage_index in button.iter() {
        new_state.joltages[*joltage_index] += presses;

        if new_state.joltages[*joltage_index] > machine.joltage_targets[*joltage_index] {
            return None;
        }
    }

    Some(new_state)
}

#[derive(Clone, Debug)]
struct JoltageButtonMap {
    joltage_index: usize,
    button_indexes: Vec<usize>,
}

fn map_joltages_to_button_sequence(machine: &Machine) -> Vec<JoltageButtonMap> {
    let mut maps: Vec<JoltageButtonMap> = vec![];
    for joltage_index in 0..machine.joltage_targets.len() {
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
    // println!("{maps:#?}");

    maps.sort_by(|a, b| {
        a.button_indexes
            .len()
            .cmp(&b.button_indexes.len())
            .then_with(|| {
                machine.joltage_targets[b.joltage_index]
                    .cmp(&machine.joltage_targets[a.joltage_index])
            })
    });
    // maps.sort_by(|a, b| machine.joltage_targets[a.joltage_index].cmp(&machine.joltage_targets[b.joltage_index]));

    let mut buttons_used: Vec<usize> = Vec::new();
    for map in &mut maps {
        map.button_indexes.retain(|idx| {
            if buttons_used.contains(idx) {
                false
            } else {
                buttons_used.push(*idx);
                true
            }
        });
    }
    // println!("{maps:#?}");

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
            joltage_targets: joltages,
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
