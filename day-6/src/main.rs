use std::env;
use std::process;
use std::fs;
use std::time::Instant;
use std::cmp::Ordering;

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

fn count(time: u64, distance: u64) -> u64 {
    fn binary_search_first_occurrence(low: u64, high: u64, key: u64) -> u64 {
        let mut l = low;
        let mut h = (high + 1) / 2;

        while l < h {
            let mid = l + (h - l) / 2;
            let val = (high - mid) * mid;
            match val.cmp(&key) {
                Ordering::Less => {
                    let next = mid + 1;
                    let val_next = (high - next) * next;
                    if val_next > key {
                        return mid;
                    }
                    l = mid + 1;
                },
                Ordering::Greater => {
                    let prev = mid - 1;
                    let val_prev = (high - prev) * prev;
                    if val_prev < key {
                        return prev;
                    }
                    h = mid;
                },
                Ordering::Equal => {
                    return mid + 1;
                },
            }
        }

        0
    }

    let start = binary_search_first_occurrence(0, time, distance);
    let end = time - start - 1;

    end - start
}

fn task_1(file: String) -> u32 {
    let mut lines = file.lines();
    let times: Vec<u32> = lines.next().expect("Missing line")
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

    let distances: Vec<u32> = lines.next().expect("Missing line")
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

    times.iter().zip(distances.iter())
          .map(|(&time, &distance)| count(time as u64, distance as u64))
          .product::<u64>() as u32
}

fn task_2(file: String) -> u32 {
    let mut lines = file.lines();
    let time = lines.next().expect("Missing line")
        .chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<u64>().expect("Could not parse string");

    let distance = lines.next().expect("Missing line")
        .chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<u64>().expect("Could not parse string");

    count(time, distance) as u32
}
