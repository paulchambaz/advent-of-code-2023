use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [path]", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let file = fs::read_to_string(path).expect("Error, could not read file");

    let sum_1 = task_1(file.clone());
    let sum_2 = task_2(file.clone());

    println!("{}", sum_1);
    println!("{}", sum_2);
}

fn task_1(file: String) -> u32 {
    let mut sum: u32 = 0;
    for line in file.lines() {
        let mut first: u32 = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                first = c.to_digit(10).unwrap();
                break;
            }
        }

        let mut last: u32 = 0;
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last = c.to_digit(10).unwrap();
                break;
            }
        }

        let n = first * 10 + last;
        sum += n;
    }

    sum
}

fn task_2(file: String) -> u32 {
    let numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let numbers: Vec<String> = numbers.iter().map(|&s| s.to_string()).collect();
    let reverse_numbers: Vec<String> = numbers.iter().map(|s| s.chars().rev().collect()).collect();

    let mut sum: u32 = 0;
    for line in file.lines() {
        
        let mut first_digit: u32 = 0;
        let mut first_digit_location: usize = line.len();
        for (n, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                first_digit = c.to_digit(10).unwrap();
                first_digit_location = n;
                break;
            }
        }
        
        let mut first_number: u32 = 0;
        let mut first_number_location: usize = line.len();
        for (n, number) in numbers.iter().enumerate() {
            match line.find(number) {
                Some(loc) => {
                    if loc < first_number_location {
                        first_number = (n + 1) as u32;
                        first_number_location = loc;
                    }
                }
                None => continue,
            }
        }

        let first = if first_digit_location < first_number_location { first_digit } else { first_number };

        let reverse_line = line.chars().rev().collect::<String>();

        let mut last_digit: u32 = 0;
        let mut last_digit_location: usize = line.len();
        for (n, c) in reverse_line.chars().enumerate() {
            if c.is_ascii_digit() {
                last_digit = c.to_digit(10).unwrap();
                last_digit_location = n;
                break;
            }
        }

        let mut last_number: u32 = 0;
        let mut last_number_location: usize = line.len();
        for (n, number) in reverse_numbers.iter().enumerate() {
            match reverse_line.find(number) {
                Some(loc) => {
                    if loc < last_number_location {
                        last_number = (n + 1) as u32;
                        last_number_location = loc;
                    }
                }
                None => continue,
            }
        }

        let last = if last_digit_location < last_number_location { last_digit } else { last_number };

        let n = first * 10 + last;
        sum += n;
    }

    sum
}
