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
    let file_1 = file.clone();
    let file_2 = file.clone();

    let start = Instant::now();

    let res_1 = task_1(file_1);
    let res_2 = task_2(file_2);

    let duration = start.elapsed();

    println!("Task 1: {}", res_1);
    println!("Task 2: {}", res_2);
    println!("Time: {} µs", duration.as_micros());
}

fn task_1(file: String) -> u32 {
    let mut sum = 0;
    for line in file.lines() {
        let (left, right) = line.split_at(line.find('|').expect("Could not find delimeter"));

        let winnings: Vec<u32> = left
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let numbers: Vec<u32> = right
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let mut score = 0;
        for number in numbers {
            if winnings.contains(&number) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }

        sum += score;
    }

    sum
}

fn task_2(file: String) -> u32 {
    let n = file.lines().count();
    let mut counter: Vec<u32> = vec![1; n];
    for (i, line) in file.lines().enumerate() {
        let (left, right) = line.split_at(line.find('|').expect("Could not find delimeter"));

        let winnings: Vec<u32> = left
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let numbers: Vec<u32> = right
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let mut score = 0;
        for number in numbers {
            if winnings.contains(&number) {
                score += 1;
            }
        }

        let value = counter[i];
        let slice = &mut counter[i + 1..n.min(score + i + 1)];
        for counter_val in slice {
            *counter_val += value;
        }
    }

    counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::{task_1, task_2};

    fn task_test(path: &str, task: fn(String) -> u32, result: u32) {
        let file = fs::read_to_string(path).expect("Error, could not read file");
        let res = task(file);
        assert_eq!(res, result);
    }

    #[test]
    fn task_1_test() {
        task_test("test", task_1, 13);
        task_test("input", task_1, 21138);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 30);
        task_test("input", task_2, 7185540);
    }
}
