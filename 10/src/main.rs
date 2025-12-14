use std::collections::HashMap;
// use std::io::{self, Write};
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
        answer += find_min_for_machine(machine);
    }
    println!("{answer}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    joltages: Vec<usize>,
    press_count: usize,
}

fn find_min_for_machine(machine: &Machine) -> usize {
    // XXX next step: kill off the state Vecs; just go fully recursive
    let maps = map_joltages_to_buttons(machine);

    let mut buttons_used: Vec<usize> = vec![];

    let mut start_states = vec![State {
        joltages: vec![0; machine.joltages.len()],
        press_count: 0,
    }];
    for map in maps {
        let mut filtered_buttons: Vec<usize> = map
            .button_indexes
            .into_iter()
            .filter(|&index| !buttons_used.contains(&index))
            .collect();

        let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
        println!(
            " Starting joltage #{} with {} buttons and {} states and {} cache",
            map.joltage_index,
            filtered_buttons.len(),
            start_states.len(),
            seen.len(),
        );

        let new_states = find_paths_to_joltage(
            machine,
            &start_states,
            &mut seen,
            &map.joltage_index,
            &filtered_buttons,
        );

        let min = new_states
            .iter()
            .map(|state| state.press_count)
            .min()
            .unwrap();
        for new_state in &new_states {
            if new_state.press_count == min && new_state.joltages == machine.joltages {
                println!("Min pressses: {min}\n");
                return min;
            }
        }

        start_states = new_states;
        buttons_used.append(&mut filtered_buttons);
    }

    panic!("It shouldn't be possible to get here");
}

fn find_paths_to_joltage(
    machine: &Machine,
    start_states: &Vec<State>,
    seen: &mut HashMap<Vec<usize>, usize>,
    target_joltage: &usize,
    button_set: &Vec<usize>,
) -> Vec<State> {
    // Permute all possible combinations of button presses.
    // Order doesn't matter; just counts (per button).
    // Never any need to press a button more than (target - current).
    // A button might be pressed anywhere from 0..(target-current).
    //
    // So e.g. 5 buttons with a target of 200 gives us 200^5 = 320B possible combinations.
    // But since we know we're evaluating all possibilities,
    // and not trying to find the shortest path first, recursive DFS would work.
    // Or that's easy to convert to a queue later.
    // We don't really need to know the "path" - just the button count that gets us there.
    //
    // Keep the "seen" idea... but of what? What is it safe to skip?
    //
    // Idea credit to michelkraemer at https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt9h7qw/

    let mut final_states = vec![];
    for state in start_states {
        // println!("  Starting: {:?} with {} buttons", state, button_set.len());

        final_states.append(&mut find_all_button_combos(
            state,
            seen,
            machine,
            &target_joltage,
            &button_set,
        ));
    }

    if final_states.len() == 0 {
        start_states
            .clone()
            .into_iter()
            .filter(|state| state.joltages[*target_joltage] == machine.joltages[*target_joltage])
            .collect()
    } else {
        final_states
    }
}

fn find_all_button_combos(
    state: &State,
    seen: &mut HashMap<Vec<usize>, usize>,
    machine: &Machine,
    target_joltage: &usize,
    button_set: &Vec<usize>,
) -> Vec<State> {
    let mut final_states = vec![];
    if button_set.len() == 0 {
        return final_states;
    }
    // println!("{button_set:#?}");

    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut remaining_buttons = button_set.clone();
    let current_button = &machine.buttons[remaining_buttons.remove(0)];

    let mut press_start = 0;
    let joltage_diff = machine.joltages[*target_joltage] - state.joltages[*target_joltage];
    if remaining_buttons.len() == 0 {
        press_start = joltage_diff;
    }
    for press_count in press_start..=joltage_diff {
        if let Some(new_state) = do_press(machine, state, &mut seen, current_button, press_count) {
            if machine.joltages[*target_joltage] == new_state.joltages[*target_joltage] {
                final_states.push(new_state);
            } else {
                let downstream_states = find_all_button_combos(
                    &new_state,
                    &mut seen,
                    machine,
                    target_joltage,
                    &remaining_buttons,
                );
                for downstream in downstream_states {
                    if downstream.joltages[*target_joltage] == machine.joltages[*target_joltage] {
                        final_states.push(downstream);
                    }
                }
            }
        }
    }

    final_states
}

fn do_press(
    machine: &Machine,
    state: &State,
    seen: &mut HashMap<Vec<usize>, usize>,
    button: &Vec<usize>,
    presses: usize,
) -> Option<State> {
    let mut new_state = state.clone();

    new_state.press_count += presses;

    for joltage_index in button.iter() {
        new_state.joltages[*joltage_index] += presses;

        if new_state.joltages[*joltage_index] > machine.joltages[*joltage_index] {
            return None;
        }
    }

    // if let Some(seen_press_count) = seen.get_mut(&new_state.joltages) {
    //     if new_state.press_count > *seen_press_count {
    //         // print!(".");
    //         // io::stdout().flush().unwrap();
    //         return None;
    //     }
    //     *seen_press_count = new_state.press_count;
    // } else {
    //     seen.insert(new_state.joltages.clone(), new_state.press_count);
    // }

    Some(new_state)
}

#[derive(Debug)]
struct JoltageButtonMap {
    joltage_index: usize,
    button_indexes: Vec<usize>,
}

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
    // println!("{maps:#?}");

    maps.sort_by(|a, b| {
        a.button_indexes
            .len()
            .cmp(&b.button_indexes.len())
            .then_with(|| machine.joltages[b.joltage_index].cmp(&machine.joltages[a.joltage_index]))
    });
    // maps.sort_by(|a, b| machine.joltages[a.joltage_index].cmp(&machine.joltages[b.joltage_index]));

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
