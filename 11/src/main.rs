use clap::Parser;
use std::collections::HashMap;
use std::{fs, path::PathBuf};

/// AOC 25 day 11
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

fn process(data: String) {
    let mut devices: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        let mut fields = line.split_whitespace();
        let id = fields.next().unwrap().trim_end_matches(":");
        devices.insert(id.to_string(), fields.map(|s| s.to_string()).collect());
    }

    let dac_fft = count_paths_between("dac", "fft", &devices, &mut HashMap::new());
    let fft_dac = count_paths_between("fft", "dac", &devices, &mut HashMap::new());
    let (first, second, mid_count) = match (dac_fft, fft_dac) {
        (0, _) => ("fft", "dac", fft_dac),
        (_, 0) => ("dac", "fft", dac_fft),
        _ => panic!("this shouldn't happen"),
    };

    let svr_first = count_paths_between("svr", first, &devices, &mut HashMap::new());
    let second_out = count_paths_between(second, "out", &devices, &mut HashMap::new());

    println!("{}", svr_first * mid_count * second_out);
}

fn count_paths_between(
    device: &str,
    destination: &str,
    devices: &HashMap<String, Vec<String>>,
    path_cache: &mut HashMap<String, usize>,
) -> usize {
    let mut total_count = 0;

    let maybe_outputs = devices.get(device);
    if maybe_outputs.is_none() {
        return total_count;
    }

    for output in maybe_outputs.unwrap() {
        match output.as_str() {
            any if any == destination => {
                total_count += 1;
            }
            _ => {
                if let Some(cached_total) = path_cache.get(output) {
                    total_count += cached_total;
                    continue;
                }

                let downstream_total_count =
                    count_paths_between(output, destination, devices, path_cache);
                total_count += downstream_total_count;

                path_cache.insert(output.to_string(), downstream_total_count);
            }
        }
    }

    total_count
}
