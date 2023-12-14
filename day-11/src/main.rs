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

fn distance(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    i64::abs(ax - bx) + i64::abs(ay - by)
}

fn task_1(file: String) -> i64 {
    let width = file.lines().count();
    let height = file.lines().next().map_or(0, |line| line.len());
    let mut galaxies_x: Vec<i64> = Vec::with_capacity(height * 4);
    let mut galaxies_y: Vec<i64> = Vec::with_capacity(height * 4);
    let mut empty_columns: Vec<bool> = vec![ true; width ];
    let mut empty_rows: Vec<bool> = vec![ true; height ];
    for (y, line) in file.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => { 
                    empty_columns[x] = false;
                    empty_rows[y] = false;
                    galaxies_x.push(x as i64);
                    galaxies_y.push(y as i64);
                },
                _ => continue,
            }
        }
    }

    let empty_rows: Vec<i64> = empty_rows.iter().scan(0, |count, &b| { if b { *count += 1; } Some(*count) }).collect();
    let empty_columns: Vec<i64> = empty_columns.iter().scan(0, |count, &b| { if b { *count += 1; } Some(*count) }).collect();

    let n_galaxy = galaxies_x.len();
    for i in 0..n_galaxy {
        galaxies_x[i] += empty_columns[galaxies_x[i] as usize];
    }
    for i in 0..n_galaxy {
        galaxies_y[i] += empty_rows[galaxies_y[i] as usize];
    }

    let mut sum = 0;
    for i in 0..n_galaxy {
        for j in i + 1..n_galaxy {
            sum += distance(galaxies_x[i], galaxies_y[i], galaxies_x[j], galaxies_y[j]);
        }
    }
    
    sum
}

fn task_2(file: String) -> i64 {
    let width = file.lines().count();
    let height = file.lines().next().map_or(0, |line| line.len());
    let mut galaxies_x: Vec<i64> = Vec::with_capacity(height * 4);
    let mut galaxies_y: Vec<i64> = Vec::with_capacity(height * 4);
    let mut empty_columns: Vec<bool> = vec![ true; width ];
    let mut empty_rows: Vec<bool> = vec![ true; height ];
    for (y, line) in file.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => { 
                    empty_columns[x] = false;
                    empty_rows[y] = false;
                    galaxies_x.push(x as i64);
                    galaxies_y.push(y as i64);
                },
                _ => continue,
            }
        }
    }

    let expansion = 1000000;
    let empty_rows: Vec<i64> = empty_rows.iter().scan(0, |count, &b| { if b { *count += expansion - 1; } Some(*count) }).collect();
    let empty_columns: Vec<i64> = empty_columns.iter().scan(0, |count, &b| { if b { *count += expansion - 1; } Some(*count) }).collect();

    let n_galaxy = galaxies_x.len();
    for i in 0..n_galaxy {
        galaxies_x[i] += empty_columns[galaxies_x[i] as usize];
    }
    for i in 0..n_galaxy {
        galaxies_y[i] += empty_rows[galaxies_y[i] as usize];
    }

    let mut sum = 0;
    for i in 0..n_galaxy {
        for j in i + 1..n_galaxy {
            sum += distance(galaxies_x[i], galaxies_y[i], galaxies_x[j], galaxies_y[j]);
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::{task_1, task_2};

    fn task_test(path: &str, task: fn(String) -> i64, result: i64) {
        let file = fs::read_to_string(path).expect("Error, could not read file");
        let res = task(file);
        assert_eq!(res, result);
    }

    #[test]
    fn task_1_test() {
        task_test("test", task_1, 374);
        task_test("input", task_1, 10885634);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 82000210);
        task_test("input", task_2, 707505470642);
    }
}
