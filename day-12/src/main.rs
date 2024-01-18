extern crate num_integer;

use std::env;
use std::process;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use num_integer::binomial;

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

fn is_pure(elements: &[char], start: usize) -> bool {
    for char in elements.iter().skip(start) {
        if *char != '?' {
            return false;
        }
    }
    true
}

fn count_potential(elements: &[char], start: usize) -> u64 {
    let mut counter = 0;
    for char in elements.iter().skip(start) {
        if *char != '.' {
            counter += 1;
        }
    }
    counter
}

fn fits(elements: &[char], start: usize, end: usize, n: usize) -> bool {
    for i in 0..n {
        if i + start < end {
            match elements[i + start] {
                '#' => continue,
                '?' => continue,
                _ => return false,
            }
        }
    }
    if n + start < end {
        match elements[n + start] {
            '#' => return true,
            '?' => return true,
            _ => return false,
        }
    }
    true
}

fn find_occurence(elements: &[char], start: usize, end: usize, n: usize) -> (usize, usize) {
    let mut j = start;
    let mut count = 0;
    while j + n + 1 < end {
        if fits(elements, start, end, n) {
            count += 1;
        } else if count != 0 {
            break
        }
        j += 1;
    }

    (count, j)
}

fn formal_solve(elements: &[char], numbers: &[u64], start: usize, end: usize, num_start: usize, num_end: usize) -> Option<u64> {
    let mut n = num_start;
    let mut start = start;
    while n < num_end {
        start = trim(elements, start);
        let num = if n < num_end { numbers[n] + 1 } else { numbers[n] } as usize;
        let (count, j) = find_occurence(elements, start, end, num);
        if count == 1 {
            // start = j - 1 + num;
            n += 1
        } else {
            return None
        }
    }
    Some(1)
}

fn trim(elements: &[char], start: usize) -> usize {
    let mut start = start;
    for element in elements.iter().skip(start) {
        if *element != '.' {
            break;
        } else {
            start += 1;
        }
    }
    start
}

fn create_key(elements: &[char], numbers: &[u64], start: usize, end: usize, num_start: usize, num_end: usize) -> String {
    format!("{}{:?}", elements[start..end].iter().collect::<String>().trim_matches('.'), &numbers[num_start..num_end])
}

fn compute_combinations(elements: &[char], numbers: &[u64], start: usize, end: usize, num_start: usize, num_end: usize, cache: &mut HashMap<String, u64>) -> u64 {
    // recursive stop - no more elements
    if start >= end {
        return if num_start >= num_end { 1 } else { 0 };
    }

    // recursive stop - no more numbers
    if num_start >= num_end {
        return if elements[start..end].contains(&'#') { 0 } else { 1 };
    }

    // sum total of elements to place
    let sum = numbers[num_start..].iter().sum::<u64>();

    // if there are more elements to place than there is space for we can early exit
    if end - start < sum as usize + num_end - num_start - 1 {
        return 0;
    }
    
    // if there are more elements to place than there are potential space for we can early exit
    if count_potential(elements, start) < sum {
        return 0;
    }

    // trim over start
    let start = trim(elements, start);
    
    // access to the cache
    let key = create_key(elements, numbers, start, end, num_start, num_end);
    // if we have already computed this value in a previous iteration we can early exit
    if let Some(&cached_result) = cache.get(&key) {
        return cached_result;
    }

    // if the list is solvable formally, we can early exit
    if let Some(result) = formal_solve(elements, numbers, start, end, num_start, num_end) {
        cache.insert(key, result);
        return result;
    }

    // if we are left with a list of '?' we can compute the binomial coheficient and early exit
    if is_pure(elements, start) {
        let mut n = (end - start) as u64;
        let k = (num_end - num_start) as u64;

        for number in numbers.iter().skip(num_start) {
            n -= number - 1;
        }

        n -= k - 1;

        let result = binomial(n, k);
        cache.insert(key, result);
        return result;
    }

    let mut result = 0;

    let first = elements[start];

    // a point here is not possible
    // if the first character is . or ? we can recurse and ignore it
    if ".?".contains(first) {
        result += compute_combinations(elements, numbers, start + 1, end, num_start, num_end, cache);
    }

    // if the first character is . or ? we can recurse search on the condition
    if "#?".contains(first) {
        let group = numbers[num_start] as usize;

        if group <= end - start && !elements[start..start + group].contains(&'.') && (group == end - start || elements[start + group] != '#') {
            result += compute_combinations(elements, numbers, start + group + 1, end, num_start + 1, num_end, cache)
        }
    }

    // now that we have the result, lets save it for later
    cache.insert(key, result);

    result
}

fn task_1(file: String) -> u64 {
    let mut sum = 0;
    for line in file.lines() {
        let mut cache = HashMap::with_capacity(32);
        let parts: Vec<&str> = line.split_whitespace().collect();
        let elements: Vec<char> = parts[0].trim_matches('.').chars().collect();
        let numbers: Vec<u64>= parts[1].split(',').filter_map(
            |str| str.parse::<u64>().ok()
        ).collect();

        println!("{:?} {:?}", elements, numbers);
        let len = elements.len();
        let num_len = numbers.len();
        let combinations = compute_combinations(&elements, &numbers, 0, len, 0, num_len, &mut cache);
        println!("{}", combinations);

        sum += combinations;
    }
    sum
}

fn task_2(file: String) -> u64 {
    let mut sum = 0;
    for line in file.lines() {
        let mut cache = HashMap::with_capacity(128);
        let parts: Vec<&str> = line.split_whitespace().collect();
        let elements_str: &str = parts[0];
        let numbers: Vec<u64> = parts[1]
            .split(',')
            .filter_map(|str| str.parse::<u64>().ok())
            .collect();
        let numbers = numbers.iter().cloned().cycle().take(numbers.len() * 5).collect::<Vec<u64>>();

        let elements: Vec<char> = std::iter::repeat(elements_str)
            .take(5)
            .collect::<Vec<&str>>()
            .join("?")
            .trim_matches('.')
            .chars()
            .collect();

        let len = elements.len();
        let num_len = numbers.len();
        let combinations = compute_combinations(&elements, &numbers, 0, len, 0, num_len, &mut cache);

        sum += combinations;
    }
    sum
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::{task_1, task_2};

    fn task_test(path: &str, task: fn(String) -> u64, result: u64) {
        let file = fs::read_to_string(path).expect("Error, could not read file");
        let res = task(file);
        assert_eq!(res, result);
    }

    #[test]
    fn task_1_test() {
        task_test("test", task_1, 21);
        task_test("input", task_1, 7716);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 525152);
        task_test("input", task_2, 18716325559999);
    }
}
