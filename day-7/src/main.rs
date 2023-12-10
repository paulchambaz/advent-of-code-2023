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

#[derive(Debug)]
struct Game {
    bid: u32,
    score: u32,
}

fn get_type_1(a: [char; 5]) -> u32 {
    let mut count: [u32; 13] = [0; 13];
    for &char in &a {
        match char {
            'A' => count[0] += 1,
            'K' => count[1] += 1,
            'Q' => count[2] += 1,
            'J' => count[3] += 1,
            'T' => count[4] += 1,
            '9' => count[5] += 1,
            '8' => count[6] += 1,
            '7' => count[7] += 1,
            '6' => count[8] += 1,
            '5' => count[9] += 1,
            '4' => count[10] += 1,
            '3' => count[11] += 1,
            '2' => count[12] += 1,
            _ => continue,
        };
    }

    let max = count.iter().max().expect("Could not find max");
    match max {
        5 => { return 6; },
        4 => { return 5; },
        3 => {
            if count.iter().any(|&x| x == 2) {
                return 4;
            }
            return 3;
        },
        2 => {
            let mut c = 0;
            for &value in &count[0..13] {
                if value == 2 {
                    c += 1;
                }
            }
            if c == 2 {
                return 2;
            } else if c == 1 {
                return 1;
            }
        },
        1 => { return 0; },
        _ => {},
    };

    0
}

fn get_score_1(a: [char; 5]) -> u32 {
    let mut sum = 0;
    let ids = [ 28561, 2197, 169, 13, 1 ];
    for i in 0..5 {
        sum += match a[i] {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => continue,
        } * ids[i];
    }
    sum
}

fn task_1(file: String) -> u32 {
    let n = file.lines().count();
    let mut games: Vec<Game> = Vec::with_capacity(n);
    for line in file.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand: [char; 5] = split[0].chars().collect::<Vec<char>>().try_into().expect("Could not get hand");
        let bid: u32 = split[1].parse::<u32>().expect("Could not get bid");
        let h_type = get_type_1(hand);
        let h_score = get_score_1(hand);
        let score = h_type * 4_826_809 + h_score;
        games.push(Game { bid, score });
    }

    games.sort_by_key(|game| game.score);

    games.iter().enumerate().map(|(rank, game)| (rank as u32 + 1) * game.bid).sum()
}
fn get_type_2(a: [char; 5]) -> u32 {
    let mut count: [u32; 12] = [0; 12];
    let mut joker_count = 0;
    for &char in &a {
        match char {
            'A' => count[0] += 1,
            'K' => count[1] += 1,
            'Q' => count[2] += 1,
            'J' => joker_count += 1,
            'T' => count[3] += 1,
            '9' => count[4] += 1,
            '8' => count[5] += 1,
            '7' => count[6] += 1,
            '6' => count[7] += 1,
            '5' => count[8] += 1,
            '4' => count[9] += 1,
            '3' => count[10] += 1,
            '2' => count[11] += 1,
            _ => continue,
        };
    }

    let (loc, max) = count.iter().enumerate().max_by_key(|&(_, value)| value).expect("Could not get max");
    match max + joker_count {
        6 => { return 6; },
        5 => { return 6; },
        4 => { return 5; },
        3 => {
            for (i, &value) in count.iter().enumerate() {
                if joker_count > 0 {
                    if i != loc && value == 2 {
                        return 4;
                    }
                } else if value == 2 {
                    return 4;
                }
            }
            return 3;
        },
        2 => {
            let mut c = 0;
            for (i, &value) in count.iter().enumerate() {
                if joker_count > 0 {
                    if i != loc && value == 2 {
                        c += 1;
                    }
                } else if value == 2 {
                    c += 1;
                }
            }
            if c  == 2 {
                return 2;
            } else if c == 1 || c == 0 {
                return 1;
            }
        },
        1 => { return 0; },
        _ => {},
    };

    10000
}

fn get_score_2(a: [char; 5]) -> u32 {
    let mut sum = 0;
    let ids = [ 28561, 2197, 169, 13, 1 ];
    for i in 0..5 {
        sum += match a[i] {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            'J' => 0,
            _ => continue,
        } * ids[i];
    }
    sum
}

fn task_2(file: String) -> u32 {
    let n = file.lines().count();
    let mut games: Vec<Game> = Vec::with_capacity(n);
    for line in file.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand: [char; 5] = split[0].chars().collect::<Vec<char>>().try_into().expect("Could not get hand");
        let bid: u32 = split[1].parse::<u32>().expect("Could not get bid");
        let h_type = get_type_2(hand);
        let h_score = get_score_2(hand);
        let score = h_type * 4_826_809 + h_score;
        games.push(Game { bid, score });
    }

    games.sort_by_key(|game| game.score);

    games.iter().enumerate().map(|(rank, game)| (rank as u32 + 1) * game.bid).sum()
}
