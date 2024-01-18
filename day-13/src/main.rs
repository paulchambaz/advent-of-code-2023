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

fn find_symmetry_point(arr: &[u32]) -> usize {
    for (i, _) in arr.iter().enumerate() {
        let mut left = &arr[..i+1];
        let mut right = &arr[i+1..];

        let llen = left.len();
        let rlen = right.len();

        if llen == 0 || rlen == 0 {
            return 0;
        }

        if llen > rlen {
            left = &left[llen - rlen..];
        } else if rlen > llen {
            right = &right[..llen];
        }

        if left.iter().rev().eq(right.iter()) {
            return i + 1;
        }
    }
    0
}

fn reflects(arr: &[u32], index: i32) -> bool {
    let len = arr.len() as i32;
    for i in 0..len {
        let ni = index - i;
        let ri = index + i + 1;
        if ni < 0 || ri >= len {
            break;
        }
        if arr[ni as usize] != arr[ri as usize] {
            return false;
        }
    }
    true
}

fn analyse_matrix_1(matrix: Vec<Vec<bool>>) -> usize {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut columns = vec![0; width];
    for x in 0..width {
        let mut column_value = 0;
        for y in 0..height {
            column_value |= (matrix[y][x] as u32) << y;
        }
        columns[x] = column_value;
    }

    let mut rows = vec![0; height];
    for y in 0..height {
        let mut row_value = 0;
        for x in 0..width {
            row_value |= (matrix[y][x] as u32) << x;
        }
        rows[y] = row_value;
    }

    for i in 0..columns.len() - 1 {
        if reflects(&columns, i as i32) {
            return i + 1;
        }
    }

    for i in 0..rows.len() - 1 {
        if reflects(&rows, i as i32) {
            return (i + 1) * 100;
        }
    }

    0
}

fn reflects_tries(arr: &[u32], index: i32) -> bool {
    let mut tries = false;
    let len = arr.len() as i32;
    for i in 0..len {
        let ni = index - i;
        let ri = index + i + 1;
        if ni < 0 || ri >= len {
            break;
        }
        // for i in len()
        if arr[ni as usize] != arr[ri as usize] {
            if tries {
                return false;
            } else {
                tries = true;
            }
        }
    }
    tries
}

fn analyse_matrix_2(matrix: Vec<Vec<bool>>) -> usize {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut columns = vec![0; width];
    for x in 0..width {
        let mut column_value = 0;
        for y in 0..height {
            column_value |= (matrix[y][x] as u32) << y;
        }
        columns[x] = column_value;
    }

    let mut rows = vec![0; height];
    for y in 0..height {
        let mut row_value = 0;
        for x in 0..width {
            row_value |= (matrix[y][x] as u32) << x;
        }
        rows[y] = row_value;
    }

    for i in 0..columns.len() - 1 {
        if reflects_tries(&columns, i as i32) {
            println!("{:?} {}", columns, i);
            return i + 1;
        }
    }

    for i in 0..rows.len() - 1 {
        if reflects_tries(&rows, i as i32) {
            println!("{:?} {}", rows, i);
            return (i + 1) * 100;
        }
    }

    0
}

fn task_1(file: String) -> u32 {
    let width = file.lines().count();
    let height = file.lines().next().map_or(0, |line| line.len());
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut start = 0;
    let mut sum = 0;
    for (y, line) in file.lines().enumerate() {
        if line.is_empty() {
            let res = analyse_matrix_1(matrix.clone());
            sum += res;
            matrix.clear();
            start = y + 1;
            continue;
        }

        matrix.push(Vec::new());

        for char in line.chars() {
            matrix[y - start].push(char == '#');
        }

    }

    let res = analyse_matrix_1(matrix);
    sum += res;

    sum as u32
}

fn task_2(file: String) -> u32 {
    let width = file.lines().count();
    let height = file.lines().next().map_or(0, |line| line.len());
    let mut matrix: Vec<Vec<bool>> = Vec::new();
    let mut start = 0;
    let mut sum = 0;
    for (y, line) in file.lines().enumerate() {
        if line.is_empty() {
            let res = analyse_matrix_2(matrix.clone());
            sum += res;
            matrix.clear();
            start = y + 1;
            continue;
        }

        matrix.push(Vec::new());

        for char in line.chars() {
            matrix[y - start].push(char == '#');
        }

    }

    let res = analyse_matrix_2(matrix);
    sum += res;

    sum as u32
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
        task_test("test", task_1, 0);
        // task_test("input", task_1, 0);
    }

    #[test]
    fn task_2_test() {
        task_test("test", task_2, 0);
        // task_test("input", task_2, 0);
    }
}
