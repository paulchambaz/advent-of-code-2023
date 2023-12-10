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
    for (i, line) in file.lines().enumerate() {
        let begin = line.find(':').expect("Could not find ':'");
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
            sum += i + 1;
        }
    }

    sum as u32
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
                let value = subparts[1].parse::<u32>().expect("Could not parse value");
                match subparts[2] {
                    "red" => if value > red { red = value },
                    "green" => if value > green { green = value },
                    "blue" => if value > blue { blue = value },
                    _ => panic!("Could not parse color"),
                };
            }
        }

        sum += red * green * blue;
    }
    sum
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
        task_test("test", task_1, 8);
        task_test("input", task_1, 2149);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 2286);
        task_test("input", task_2, 71274);
    }
}
