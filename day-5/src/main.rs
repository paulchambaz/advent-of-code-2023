use std::env;
use std::process;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [path]", args[0]);
        process::exit(1);
    }

    let path = &args[1];
    let file = fs::read_to_string(path).expect("Error, could not read file");

    let start = Instant::now();

    let task_1 = task_1(file.clone());
    let task_2 = task_2(file);

    let duration = start.elapsed();

    println!("Task 1: {}", task_1);
    println!("Task 2: {}", task_2);
    println!("Time: {} µs", duration.as_micros());
}

#[derive(Clone, Debug)]
struct Convert {
    start: i64,
    end: i64,
    convert: i64,
}

#[derive(Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn task_1(file: String) -> u32 {
    let mut maps: Vec<Vec<Convert>> = vec![Vec::new(); 7];
    let mut map_type = 0;
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            continue;

        } else if let Some(first_char) = line.chars().next() {
            if first_char.is_ascii_digit() {
                let map: Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();

                maps[map_type - 1].push(Convert {
                    start: map[1],
                    end: map[1] + map[2],
                    convert: map[0] - map[1],
                });
            } else {
                map_type += 1;
            }
        } else {
            continue;
        }
    }

    let seeds: Vec<i64> = file.lines()
        .next().expect("No first line")
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut min = i64::MAX;
    for seed in seeds {
        let mut next = seed;
        for map in &maps {
            for convert in map {
                if next >= convert.start && next < convert.end {
                    next += convert.convert;
                    break;
                }
            }
        }
        if next < min {
            min = next;
        }
    }

    min as u32
}

fn task_2(file: String) -> u32 {
    let mut maps: Vec<Vec<Convert>> = vec![Vec::new(); 7];
    let mut map_type = 0;
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            continue;
        } else if let Some(first_char) = line.chars().next() {
            if first_char.is_ascii_digit() {
                let map: Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();

                maps[map_type - 1].push(Convert {
                    start: map[1],
                    end: map[1] + map[2] - 1,
                    convert: map[0] - map[1],
                });
            } else {
                map_type += 1;
            }
        } else {
            continue;
        }
    }

    let seeds = file.lines().next().expect("No first line")
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut pairs = seeds.chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some(Range{ start: chunk[0], end: chunk[0] + chunk[1] - 1 })
            } else {
                None
            }
        })
        .collect::<Vec<Range>>();

    for map in maps {
        let mut working_pairs: Vec<Range> = Vec::with_capacity(pairs.len());
        let mut new_pairs: Vec<Range> = Vec::with_capacity(pairs.len());

        for pair in &pairs {
            working_pairs.push(pair.clone());
        }

        'pair_loop: while !working_pairs.is_empty() {
            if let Some(pair) = working_pairs.pop() {
                if pair.start > pair.end {
                    println!("ERROR: {} {}", pair.start, pair.end);
                    process::exit(0);
                }
                let mut modified = false;
                for convert in &map {
                    if pair.end < convert.start || pair.start > convert.end {
                    } else if pair.start >= convert.start && pair.end <= convert.end {
                        new_pairs.push(Range { start: pair.start + convert.convert, end: pair.end + convert.convert });
                        modified = true;
                        break;
                    } else if pair.start < convert.start {
                        working_pairs.push(Range { start: pair.start, end: convert.start - 1 });
                        working_pairs.push(Range { start: convert.start, end: pair.end });
                        continue 'pair_loop;
                    } else if pair.end > convert.end {
                        working_pairs.push(Range { start: pair.start, end: convert.end });
                        working_pairs.push(Range { start: convert.end + 1, end: pair.end });
                        continue 'pair_loop;
                    }
                }
                if !modified {
                    new_pairs.push(Range { start: pair.start, end: pair.end });
                }
            }
        }
        pairs = new_pairs;
    }

    let mut min = i64::MAX;
    for pair in pairs {
        if pair.start < min {
            min = pair.start;
        }
    }

    min as u32
}
