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

fn task_1(file: String) -> u32 {
    const SPECIAL_CHARS: [char; 10] = ['#', '%', '&', '*', '+', '-', '/', '=', '@', '$'];
    let n = file.lines().count();
    let mut engines: Vec<Vec<Engine>> = Vec::with_capacity(n);
    let mut powers: Vec<Vec<Power>> = Vec::with_capacity(n);
    let mut str = String::with_capacity(4);

    for _ in 0..n {
        engines.push(Vec::with_capacity(16));
        powers.push(Vec::with_capacity(16));
    }

    for (y, line) in file.lines().enumerate() {
        str.clear();
        let mut start = -1;
        for (x, char) in line.chars().enumerate() {
            let x = x as i32;

            if char.is_numeric() {
                if start == -1 {
                    start = x;
                }
                str.push(char);
            } else {
                if start != -1 {
                    match str.parse::<i32>() {
                        Ok(number) => engines[y].push(Engine { number, start, end: x - 1 }),
                        Err(_) => { eprintln!("Error trying to parse number"); process::exit(1); },
                    };
                    str.clear();
                    start = -1;
                }

                if SPECIAL_CHARS.contains(&char) {
                    powers[y].push(Power { x });
                }
            }
        }

        if start != -1 {
            match str.parse::<i32>() {
                Ok(number) => engines[y].push(Engine { number, start, end: (line.len() as i32) - 1 }),
                Err(_) => { eprintln!("Error trying to parse number"); process::exit(1); },
            };
        }
    }

    let mut sum = 0;
    for (y, engine_group) in engines.into_iter().enumerate() {
        for engine in engine_group {
            if y as i32 > 0 {
                for power in &powers[y - 1] {
                    if aabb(engine.start, engine.end, power.x) {
                        sum += engine.number;
                        break;
                    }
                }
            }

            for power in &powers[y] {
                if aabb(engine.start, engine.end, power.x) {
                    sum += engine.number;
                    break;
                }
            }


            if y + 1 < powers.len() {
                for power in &powers[y + 1] {
                    if aabb(engine.start, engine.end, power.x) {
                        sum += engine.number;
                        break;
                    }
                }
            }
        }
    }

    sum as u32
}

fn task_2(file: String) -> u32 {
    const SPECIAL_CHARS: [char; 10] = ['#', '%', '&', '*', '+', '-', '/', '=', '@', '$'];
    let n = file.lines().count();
    let mut engines: Vec<Vec<Engine>> = Vec::with_capacity(n);
    let mut powers: Vec<Vec<Power>> = Vec::with_capacity(n);
    let mut str = String::with_capacity(4);

    for _ in 0..n {
        engines.push(Vec::with_capacity(16));
        powers.push(Vec::with_capacity(16));
    }

    for (y, line) in file.lines().enumerate() {
        str.clear();
        let mut start = -1;
        for (x, char) in line.chars().enumerate() {
            let x = x as i32;

            if char.is_numeric() {
                if start == -1 {
                    start = x;
                }
                str.push(char);
            } else {
                if start != -1 {
                    match str.parse::<i32>() {
                        Ok(number) => engines[y].push(Engine { number, start, end: x - 1 }),
                        Err(_) => { eprintln!("Error trying to parse number"); process::exit(1); },
                    };
                    str.clear();
                    start = -1;
                }

                if SPECIAL_CHARS.contains(&char) {
                    powers[y].push(Power { x });
                }
            }
        }

        if start != -1 {
            match str.parse::<i32>() {
                Ok(number) => engines[y].push(Engine { number, start, end: (line.len() as i32) - 1 }),
                Err(_) => { eprintln!("Error trying to parse number"); process::exit(1); },
            };
        }
    }

    let mut sum = 0;
    let mut ratios: [i32; 2] = [0; 2];
    for (y, power_group) in powers.into_iter().enumerate() {
        'power_loop: for power in power_group {
            let mut len = 0;

            if y as i32 > 0 {
                for engine in &engines[y - 1] {
                    if aabb(engine.start, engine.end, power.x) {
                        if len == 2 {
                            continue 'power_loop
                        }
                        ratios[len] = engine.number;
                        len += 1;
                    }
                }
            }

            for engine in &engines[y] {
                if aabb(engine.start, engine.end, power.x) {
                    if len == 2 {
                        continue 'power_loop
                    }
                    ratios[len] = engine.number;
                    len += 1;
                }
            }

            if y + 1 < engines.len() {
                for engine in &engines[y + 1] {
                    if aabb(engine.start, engine.end, power.x) {
                        if len == 2 {
                            continue 'power_loop
                        }
                        ratios[len] = engine.number;
                        len += 1;
                    }
                }
            }

            if len == 2 {
                sum += ratios[0] * ratios[1];
            }
        }
    }

    sum as u32
}
