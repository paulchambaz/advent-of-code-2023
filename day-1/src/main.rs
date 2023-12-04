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
    let sum_2 = task_2(file);

    println!("Sum: {}", sum_1);
    println!("Sum: {}", sum_2);
}

fn task_1(file: String) -> u32 {
    let mut sum: u32 = 0;
    for line in file.lines() {
        let mut first = 0;
        let mut last = 0;

        for c in line.chars() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).expect("Could not parse digit");
                if first == 0 {
                    first = digit;
                }
                last = digit;
            }
        }

        let n = first * 10 + last;
        sum += n;
    }

    sum
}

fn task_2(file: String) -> u32 {
    let numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let mut sum: u32 = 0;
    for line in file.lines() {
        
        let mut first: u32 = 0;
        let mut first_location: usize = usize::MAX;
        let mut last: u32 = 0;
        let mut last_location: usize = 0;

        for (n, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).expect("Could not parse digit");
                if n < first_location {
                    first = digit;
                    first_location = n;
                }
                last = digit;
                last_location = n;
            }
        }

        for (n, number) in numbers.iter().enumerate() {
            if let Some(i) = line.find(number) {
                let num = (n + 1) as u32;
                if i < first_location {
                    first = num;
                    first_location = i;
                }
            }
            if let Some(i) = line.rfind(number) {
                let num = (n + 1) as u32;
                if i + 1 > last_location {
                    last = num;
                    last_location = i + 1;
                }
            }
        }

        let n = first * 10 + last;
        sum += n;
    }

    sum
}
