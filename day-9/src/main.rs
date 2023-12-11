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

fn find_next(numbers: Vec<i32>) -> i32 {
    let len = numbers.len();
    // TODO: let sequences: Vec<Vec<u32>> = Vec::with_capacity(len/ 2);
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(numbers);
    let mut i = 1;
    loop {
        // computing the next line
        let mut sequence: Vec<i32> = Vec::with_capacity(len - i);
        for j in 0..len - i {
            sequence.push(sequences[i - 1][j + 1] - sequences[i - 1][j]);
        }
        // checking the line is not full 0
        let mut void = true;
        for number in &sequence {
            if *number != 0 {
                void = false;
                break;
            }
        }
        if void {
            // we have reached the final line
            break;
        }
        sequences.push(sequence);
        i += 1;
    };
    let count = i - 1;
    let mut result = vec![0; i];
    result[count] = sequences[count][len - count - 1];
    for j in 1..i {
        result[count - j] = sequences[count - j][len - count + j - 1] + result[count - j + 1];
    }
    result[0]
}

fn find_prev(numbers: Vec<i32>) -> i32 {
    let len = numbers.len();
    // TODO: let sequences: Vec<Vec<u32>> = Vec::with_capacity(len/ 2);
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(numbers);
    let mut i = 1;
    loop {
        // computing the next line
        let mut sequence: Vec<i32> = Vec::with_capacity(len - i);
        for j in 0..len - i {
            sequence.push(sequences[i - 1][j + 1] - sequences[i - 1][j]);
        }
        // checking the line is not full 0
        let mut void = true;
        for number in &sequence {
            if *number != 0 {
                void = false;
                break;
            }
        }
        if void {
            // we have reached the final line
            break;
        }
        sequences.push(sequence);
        i += 1;
    };
    let count = i - 1;
    let mut result = vec![0; i];
    result[count] = sequences[count][0];
    for j in 1..i {
        result[count - j] = sequences[count - j][0] - result[count - j + 1];
    }
    result[0]
}

fn task_1(file: String) -> i32 {
    let mut sum = 0;
    for line in file.lines() {
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|e| e.parse::<i32>().ok()).collect();
        let res = find_next(numbers);
        sum += res;
    }
    sum
}

fn task_2(file: String) -> i32 {
    let mut sum = 0;
    for line in file.lines() {
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|e| e.parse::<i32>().ok()).collect();
        let res = find_prev(numbers);
        sum += res;
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::{task_1, task_2};

    fn task_test(path: &str, task: fn(String) -> i32, result: i32) {
        let file = fs::read_to_string(path).expect("Error, could not read file");
        let res = task(file);
        assert_eq!(res, result);
    }

    #[test]
    fn task_1_test() {
        task_test("test", task_1, 114);
        task_test("input", task_1, 1969958987);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 2);
        // task_test("input", task_2, 0);
    }
}
