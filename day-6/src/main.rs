use std::env;
use std::process;
use std::fs;
use std::time::Instant;
use roots::Roots;
use roots::find_roots_quadratic;

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

fn count(time: u32, distance: u32) -> u32 {
    match find_roots_quadratic::<f32>(1., -(time as f32), distance as f32) {
        Roots::Two(r) => time - 2 * f32::ceil(r[0]) as u32 + 1,
        _ => 0,
    }
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
          .map(|(&time, &distance)| count(time, distance))
          .product()
}

fn task_2(file: String) -> u32 {
    let mut lines = file.lines();
    let time = lines.next().expect("Missing line")
        .chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<f64>().expect("Could not parse string");

    let distance = lines.next().expect("Missing line")
        .chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<f64>().expect("Could not parse string");

    let count = match find_roots_quadratic::<f64>(1., -(time), distance) {
        Roots::Two(r) => time - 2. * f64::ceil(r[0]) + 1.,
        _ => 0.,
    };

    count as u32
}
