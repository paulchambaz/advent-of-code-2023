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

struct Engine {
    number: i32,
    start: i32,
    end: i32,
    y: i32,
}

struct Power {
    x: i32,
    y: i32,
}

fn create_engine(str: String, start: i32, end: usize, y: usize) -> Engine {
    Engine {
        number: str.parse::<i32>().expect("Error trying to parse number"),
        start,
        end: (end - 1) as i32,
        y: y as i32,
    }
}

fn aabb(a_start: i32, a_end: i32, a_y: i32, b_x: i32, b_y: i32) -> bool {
    a_y - 1 <= b_y && a_y + 1 >= b_y && a_start - 1 <= b_x && a_end + 1 >= b_x
}

fn task_1(file: String) -> u32 {
    let mut engines: Vec<Engine> = Vec::new();
    let mut powers: Vec<Power> = Vec::new();
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
                    y: y as i32,
                };
                powers.push(power);
                if start != -1 {
                    engines.push(create_engine(str, start, x, y));
                    str = String::new();
                    start = -1;
                }
            } else if start != -1 {
                engines.push(create_engine(str, start, x, y));
                str = String::new();
                start = -1;
            }
        }

        if start != -1 {
            engines.push(create_engine(str, start, line.len(), y));
        }
    }

    let mut sum = 0;
    for engine in &engines {
        for power in &powers {
            if aabb(engine.start, engine.end, engine.y, power.x, power.y) {
                sum += engine.number;
                break;
            }
        }
    }

    sum as u32
}

fn task_2(file: String) -> u32 {
    let mut engines: Vec<Engine> = Vec::new();
    let mut powers: Vec<Power> = Vec::new();
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
                    y: y as i32,
                };
                powers.push(power);
                if start != -1 {
                    engines.push(create_engine(str, start, x, y));
                    str = String::new();
                    start = -1;
                }
            } else if start != -1 {
                engines.push(create_engine(str, start, x, y));
                str = String::new();
                start = -1;
            }
        }

        if start != -1 {
            engines.push(create_engine(str, start, line.len(), y));
        }
    }

    // validate the engines
    let mut sum = 0;
    for power in &powers {
        let mut ratios: Vec<i32> = Vec::new();
        for engine in &engines {
            if aabb(engine.start, engine.end, engine.y, power.x, power.y) {
                ratios.push(engine.number);
            }
        }

        if ratios.len() == 2 {
            let product: i32 = ratios.iter().product();
            sum += product;
        }
    }

    sum as u32
}
