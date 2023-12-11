use std::env;
use std::process;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::fmt;
use num::integer::lcm;

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

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

struct NodeStr {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Node {
    left: usize,
    right: usize,
}

fn apply_direction(start: usize, directions: &Vec<Direction>, nodes: &[Node]) -> usize {
    let mut next = start;
    for direction in directions {
        match direction {
            Direction::Left => {
                next = nodes[next].left;
            },
            Direction::Right => {
                next = nodes[next].right;
            },
        };
    }
    next
}

            // nicewigg
fn task_1(file: String) -> u32 {
    let n = file.lines().count();
    let n = n - 2;
    let mut labels: HashMap<String, usize> = HashMap::with_capacity(n);
    let mut directions: Vec<Direction> = Vec::new();
    let mut nodes_str: Vec<NodeStr> = Vec::with_capacity(n);
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            for char in line.chars() {
                let direction = match char {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => Direction::Left,
                };
                directions.push(direction);
            }
        } else if i == 1 {
            continue;
        } else {
            labels.insert(line[0..3].to_string(), i - 2);
            nodes_str.push(NodeStr { left: line[7..10].to_string(), right: line[12..15].to_string() });
        }
    }
    let mut nodes: Vec<Node> = Vec::with_capacity(n);
    for node in &nodes_str {
        let left = match labels.get(&node.left) {
            Some(left) => *left,
            _ => 0,
        };
        let right = match labels.get(&node.right) {
            Some(right) => *right,
            _ => 0,
        };
        nodes.push(Node {
            left,
            right,
        });
    }

    let mut iterations: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        iterations.push(apply_direction(i, &directions, &nodes));
    }

    let start = match labels.get("AAA") {
        Some(start) => *start,
        _ => 0,
    };

    let end = match labels.get("ZZZ") {
        Some(end) => *end,
        _ => 0,
    };

    let mut count = 0;
    let mut current = start;
    while current != end {
        current = iterations[current];
        count += 1;
    }

    (count * directions.len()) as u32
}

fn task_2(file: String) -> u64 {
    let n = file.lines().count();
    let n = n - 2;
    let mut labels: HashMap<String, usize> = HashMap::with_capacity(n);
    let mut directions: Vec<Direction> = Vec::new();
    let mut nodes_str: Vec<NodeStr> = Vec::with_capacity(n);
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            for char in line.chars() {
                let direction = match char {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => Direction::Left,
                };
                directions.push(direction);
            }
        } else if i == 1 {
            continue;
        } else {
            labels.insert(line[0..3].to_string(), i - 2);
            nodes_str.push(NodeStr { left: line[7..10].to_string(), right: line[12..15].to_string() });
        }
    }
    let mut nodes: Vec<Node> = Vec::with_capacity(n - 2);
    for node in &nodes_str {
        let left = match labels.get(&node.left) {
            Some(left) => *left,
            _ => 0,
        };
        let right = match labels.get(&node.right) {
            Some(right) => *right,
            _ => 0,
        };
        nodes.push(Node {
            left,
            right,
        });
    }

    let mut iterations: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        iterations.push(apply_direction(i, &directions, &nodes));
    }

    let ends: Vec<usize> = labels
        .iter()
        .filter_map(|(key, &value)| {
            if key.chars().nth(2) == Some('Z') {
                Some(value)
            } else {
                None
            }
        })
        .collect();

    let mut cycles = Vec::with_capacity(n);
    for end in &ends {
        let mut count = 0;
        let mut current = *end;
        loop {
            current = iterations[current];
            count += 1;
            if current == *end {
                break;
            }
        }
        cycles.push(count as u64);
    }

    let lcm_result = cycles.iter().fold(1, |acc, &num| lcm(acc, num));

    lcm_result * directions.len() as u64
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
        task_test("test1a", task_1, 2);
        task_test("test1b", task_1, 6);
        task_test("input", task_1, 18727);
    }

    #[test]
    fn task_2_test() {
        let file = fs::read_to_string("test2").expect("Error, could not read file");
        let res = task_2(file);
        assert_eq!(res, 6);

        let file = fs::read_to_string("input").expect("Error, could not read file");
        let res = task_2(file);
        assert_eq!(res, 18024643846273);
    }
}
