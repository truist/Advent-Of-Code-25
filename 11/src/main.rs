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

    // dac -> fft
    // fft -> dac
    // hopefully one has none
    // svr -> whichever one comes first
    // whichever one comes second -> out
    // multiply the three together?

    let dac_fft = count_with_cache("dac", "fft", &devices, true);
    println!("dac_fft: {dac_fft}");

    let fft_dac = count_with_cache("fft", "dac", &devices, true);
    println!("fft_dac: {fft_dac}");

    let (first, second, mid_count) = match (dac_fft, fft_dac) {
        (0, _) => ("fft", "dac", fft_dac),
        (_, 0) => ("dac", "fft", dac_fft),
        _ => panic!("this shouldn't happen"),
    };

    let svr_first = count_with_cache("svr", first, &devices, true);
    println!("svr_first: {svr_first}");

    let second_out = count_with_cache(second, "out", &devices, true);
    println!("second_out: {second_out}");

    println!("{}", svr_first * mid_count * second_out);
}

fn count_with_cache(
    start: &str,
    destination: &str,
    devices: &HashMap<String, Vec<String>>,
    totals_only: bool,
) -> usize {
    let seen_already = Seen {
        dac: false,
        fft: false,
    };
    let mut path_cache: HashMap<String, CacheRecord> = HashMap::new();
    let (seen_count, total_count) = count_paths_between(
        start,
        destination,
        devices,
        &seen_already,
        &mut path_cache,
        totals_only,
    );

    if totals_only {
        total_count
    } else {
        seen_count
    }
}

#[derive(Clone, Debug)]
struct Seen {
    dac: bool,
    fft: bool,
}

#[derive(Debug)]
struct CacheRecord {
    // seen_count: usize,
    total_count: usize,
    dac_downstream: Option<bool>,
    fft_downstream: Option<bool>,
}

fn count_paths_between(
    device: &str,
    destination: &str,
    devices: &HashMap<String, Vec<String>>,
    seen_already: &Seen,
    path_cache: &mut HashMap<String, CacheRecord>,
    totals_only: bool,
) -> (usize, usize) {
    let mut seen_already = seen_already.clone();
    match device {
        "dac" => seen_already.dac = true,
        "fft" => seen_already.fft = true,
        _ => (),
    }
    // println!("{device}: {seen_already:?}");

    let mut seen_count = 0;
    let mut total_count = 0;
    let maybe_outputs = devices.get(device);
    if maybe_outputs.is_none() {
        return (seen_count, total_count);
    }
    for output in maybe_outputs.unwrap() {
        match output.as_str() {
            any if any == destination => {
                if seen_already.dac && seen_already.fft {
                    seen_count += 1;
                }
                total_count += 1;
            }
            _ => {
                if let Some(cache_record) = path_cache.get(output) {
                    // println!(" Got record from cache for {}: {:?}", output, cache_record);
                    if totals_only
                        || ((seen_already.dac
                            || cache_record.dac_downstream.is_some_and(|val| val))
                            && (seen_already.fft
                                || cache_record.fft_downstream.is_some_and(|val| val)))
                    {
                        println!("Using cache for {output}: {}", cache_record.total_count);
                        // println!(
                        //     "  and we're using the total count (second case)! ({})",
                        //     // cache_record.seen_count
                        //     cache_record.total_count
                        // );
                        // seen_count += cache_record.seen_count;
                        seen_count += cache_record.total_count;
                        total_count += cache_record.total_count;
                        continue;
                    }
                }

                let (downstream_seen_count, downstream_total_count) = count_paths_between(
                    output,
                    destination,
                    devices,
                    &seen_already,
                    path_cache,
                    totals_only,
                );
                seen_count += downstream_seen_count;
                total_count += downstream_total_count;
                // println!(
                //     "  got counts for {output}: {downstream_seen_count}, {downstream_total_count}"
                // );

                if !totals_only && let Some(cache_record) = path_cache.get_mut(output) {
                    // downstream_seen_count: 0 or non-zero
                    //  if 0, that means we didn't get enough additional variables
                    //   ...or we didn't reach the end, but that's probably a redundant case
                    //   so if only one var was 'already' true then the other can be cached as false
                    //  if non-zero, if any 'already' vars were false, we can cache them as true
                    if downstream_seen_count == 0 {
                        if seen_already.dac != seen_already.fft {
                            if !seen_already.dac {
                                cache_record.dac_downstream = Some(false);
                            } else {
                                cache_record.fft_downstream = Some(false);
                            }
                        }
                    } else {
                        if !seen_already.dac {
                            cache_record.dac_downstream = Some(true);
                        }
                        if !seen_already.fft {
                            cache_record.fft_downstream = Some(true);
                        }
                    }
                    // println!("  and we might have altered it!: {:?}", cache_record);
                    // assert_eq!(
                    //     cache_record.seen_count, downstream_seen_count,
                    //     "previously-cached record matches new seen count"
                    // );
                    assert_eq!(
                        cache_record.total_count, downstream_total_count,
                        "previousy-cached record matches new total count"
                    );
                } else {
                    let cache_record = CacheRecord {
                        // seen_count: downstream_seen_count,
                        total_count: downstream_total_count,
                        dac_downstream: if !seen_already.dac {
                            Some(downstream_seen_count > 0)
                        } else {
                            None
                        },
                        fft_downstream: if !seen_already.fft {
                            Some(downstream_seen_count > 0)
                        } else {
                            None
                        },
                    };
                    println!(" Adding new cache record for {output}: {:?}", cache_record);
                    path_cache.insert(output.to_string(), cache_record);
                }
            }
        }
    }

    (seen_count, total_count)
}
