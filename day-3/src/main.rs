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

#[derive(Clone, Copy)]
struct Engine {
    number: i32,
    start: i32,
    end: i32,
}

#[derive(Clone, Copy, Debug)]
struct Power {
    x: i32,
}

fn aabb(a_start: i32, a_end: i32, b_x: i32) -> bool {
    a_start - 1 <= b_x && a_end + 1 >= b_x
}

fn merge_items<T: Clone>(items: &[Vec<T>], y: usize) -> Vec<T> {
    let mut merged = Vec::new();
    if y as i32 - 1 >= 0 {
        if let Some(prev) = items.get(y - 1) {
            merged.extend_from_slice(prev);
        }
    }
    if let Some(current) = items.get(y) {
        merged.extend_from_slice(current);
    }
    if y + 1 < items.len() {
        if let Some(next) = items.get(y + 1) {
            merged.extend_from_slice(next);
        }
    }
    merged
}

fn task_1(file: String) -> u32 {
    let n = file.lines().count();
    let mut engines: Vec<Vec<Engine>> = Vec::with_capacity(n);
    for _ in 0..n { engines.push(Vec::new()); }
    let mut powers: Vec<Vec<Power>> = Vec::with_capacity(n);
    for _ in 0..n { powers.push(Vec::new()); }
    for (y, line) in file.lines().enumerate() {
        let mut str = String::new();
        let mut start = -1;
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if start == -1 {
                    start = x as i32;
                }
                str.push(char);
            } else if "#%&*+-/=@$".contains(char) {
                let power = Power {
                    x: x as i32,
                };
                powers[y].push(power);
                if start != -1 {
                    let engine = Engine {
                        number: str.parse::<i32>().expect("Error trying to parse number"),
                        start,
                        end: (x - 1) as i32,
                    };
                    engines[y].push(engine);
                    str.clear();
                    start = -1;
                }
            } else if start != -1 {
                let engine = Engine {
                    number: str.parse::<i32>().expect("Error trying to parse number"),
                    start,
                    end: (x - 1) as i32,
                };
                engines[y].push(engine);
                str.clear();
                start = -1;
            }
        }

        if start != -1 {
            let engine = Engine {
                number: str.parse::<i32>().expect("Error trying to parse number"),
                start,
                end: (line.len() - 1) as i32,
            };
            engines[y].push(engine);
        }
    }

    let mut sum = 0;
    for (y, engine_group) in engines.into_iter().enumerate() {
        let merged = merge_items::<Power>(&powers, y);
        for engine in engine_group {
            for power in &merged {
                if aabb(engine.start, engine.end, power.x) {
                    sum += engine.number;
                    break;
                }
            }
        }
    }

    sum as u32
}

fn task_2(file: String) -> u32 {
    let n = file.lines().count();
    let mut engines: Vec<Vec<Engine>> = Vec::with_capacity(n);
    for _ in 0..n { engines.push(Vec::new()); }
    let mut powers: Vec<Vec<Power>> = Vec::with_capacity(n);
    for _ in 0..n { powers.push(Vec::new()); }
    for (y, line) in file.lines().enumerate() {
        let mut str = String::new();
        let mut start = -1;
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if start == -1 {
                    start = x as i32;
                }
                str.push(char);
            } else if char == '*' {
                let power = Power {
                    x: x as i32,
                };
                powers[y].push(power);
                if start != -1 {
                    let engine = Engine {
                        number: str.parse::<i32>().expect("Error trying to parse number"),
                        start,
                        end: (x - 1) as i32,
                    };
                    engines[y].push(engine);
                    str.clear();
                    start = -1;
                }
            } else if start != -1 {
                let engine = Engine {
                    number: str.parse::<i32>().expect("Error trying to parse number"),
                    start,
                    end: (x - 1) as i32,
                };
                engines[y].push(engine);
                str.clear();
                start = -1;
            }
        }

        if start != -1 {
            let engine = Engine {
                number: str.parse::<i32>().expect("Error trying to parse number"),
                start,
                end: (line.len() - 1) as i32,
            };
            engines[y].push(engine);
        }
    }

    let mut sum = 0;
    for (y, power_group) in powers.into_iter().enumerate() {
        let merged = merge_items::<Engine>(&engines, y);
        for power in power_group {
            let mut ratios: Vec<i32> = Vec::new();
            for engine in &merged {
                if aabb(engine.start, engine.end, power.x) {
                    ratios.push(engine.number);
                }
            }
            if ratios.len() == 2 {
                let product: i32 = ratios.iter().product();
                sum += product;
            }
        }
    }

    sum as u32
}
