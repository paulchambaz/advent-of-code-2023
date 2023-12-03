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
    
    let task_1 = task_1(file.clone());
    let task_2 = task_2(file.clone());

    println!("Task 1: {}", task_1);
    println!("Task 2: {}", task_2);
}

fn task_1(file: String) -> u32 {
    let mut sum = 0;
    for line in file.lines() {
        let begin = line.find(':').expect("Could not find ':'");
        let end = line.find(' ').expect("Could not find ' '");
        let game = line.get(end + 1..begin).expect("Slice out of bound").parse::<u32>().expect("Could not parse game");
        let parts = line.get(begin + 1..).expect("Slice out of bound");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for game in parts.split(';') {
            for draw in game.split(',') {
                let subparts: Vec<&str> = draw.split(' ').collect();
                let value = subparts[1].parse::<u32>().expect("Could not parse value");
                match subparts[2] {
                    "red" => if value > red { red = value },
                    "green" => if value > green { green = value },
                    "blue" => if value > blue { blue = value },
                    _ => panic!("Could not match color")
                };
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            sum += game;
        }
    }

    sum
}

fn task_2(file: String) -> u32 {
    let mut sum = 0;
    for line in file.lines() {
        let begin = line.find(':').expect("Could not find ':'");
        let parts = line.get(begin + 1..).expect("Slice out of bound");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for game in parts.split(';') {
            for draw in game.split(',') {
                let subparts: Vec<&str> = draw.split(' ').collect();
                let value = subparts.get(1).expect("Could not get value string").parse::<u32>().expect("Could not parse value");
                let color = subparts.get(2).expect("Could not get color string");
                match *color {
                    "red" => if value > red { red = value },
                    "green" => if value > green { green = value },
                    "blue" => if value > blue { blue = value },
                    _ => panic!("Could not match color"),
                };
            }
        }

        let power = red * green * blue;

        sum += power;
    }

    sum
}
