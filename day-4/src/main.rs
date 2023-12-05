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

fn task_1(file: String) -> u32 {
    let mut sum = 0;

    for line in file.lines() {
        let parts: Vec<&str> = line.split_once(':').map(|(_, rest)| rest.split('|').collect()).unwrap_or_default();
        
        let mut winnings: Vec<u32> = parts.first().expect("Could not parse winning numbers")
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Could not parse winning numbers"))
            .collect();
        winnings.sort();

        let mut numbers: Vec<u32> = parts.get(1).expect("Could not parse winning numbers")
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Could not parse winning numbers"))
            .collect();
        numbers.sort();

        let mut start = 0;
        let mut score = 0;
        for winning in winnings {
            if let Ok(index) =  numbers[start..].binary_search(&winning) {
                start = index;
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        sum += score;
    }

    sum
}

fn task_2(file: String) -> u32 {
    let n = file.lines().count() as u32;
    let mut scores: Vec<usize> = Vec::with_capacity(n as usize);
    for line in file.lines() {
        let parts: Vec<&str> = line.split_once(':').map(|(_, rest)| rest.split('|').collect()).unwrap_or_default();
        
        let mut winnings: Vec<u32> = parts.first().expect("Could not parse winning numbers")
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Could not parse winning numbers"))
            .collect();
        winnings.sort();

        let mut numbers: Vec<u32> = parts.get(1).expect("Could not parse winning numbers")
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Could not parse winning numbers"))
            .collect();
        numbers.sort();

        let mut start = 0;
        let mut score = 0;
        for winning in winnings {
            if let Ok(index) =  numbers[start..].binary_search(&winning) {
                start = index;
                score += 1;
            }
        }

        scores.push(score);
    }
    
    let mut counter: Vec<usize> = vec![1; n as usize];
    for (i, score) in scores.into_iter().enumerate() {
        for n in 0..score {
            let index: usize = n + i + 1;
            counter[index] += counter[i];
        }
        
    }

    let sum: usize = counter.iter().sum();

    sum as u32
}
