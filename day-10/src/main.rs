use std::env;
use std::fs;
use std::process;
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

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    a: Direction,
    b: Direction,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Searcher {
    searching: bool,
    location: Point,
    coming_from: Direction,
    count: u32,
}

struct MazeSearcher {
    searching: bool,
    location: Point,
    coming_from: Direction,
    count: u32,
    graph: Vec<Point>,
}

fn get_node(
    point: Point,
    maze: &[Vec<Option<Node>>],
    width: usize,
    height: usize,
) -> Option<Node> {

    if point.y < 0 || point.y >= height as i32 || point.x < 0 || point.x >= width as i32 {
        return None;
    }

    maze[point.y as usize][point.x as usize]
}

fn segment_intersect(vertical: Segment, horizontal: Segment) -> bool {
    // println!("{:?} {:?}", vertical, horizontal);
    let ax = vertical.start_x;
    let asy;
    let aey;
    if vertical.start_y < vertical.end_y {
        asy = vertical.start_y;
        aey = vertical.end_y;
    } else {
        asy = vertical.end_y;
        aey = vertical.start_y;
    }
    let bsx;
    let bex;
    if horizontal.start_x < horizontal.end_x {
        bsx = horizontal.start_x;
        bex = horizontal.end_x;
    } else {
        bsx = horizontal.end_x;
        bex = horizontal.start_x;
    }
    let by = horizontal.start_y;


    if by < asy || by > aey {
        return false;
    }

    if ax < bsx || ax > bex {
        return false;
    }

    true
}

fn task_1(file: String) -> u32 {
    let height = file.lines().count();
    let mut width = 0;
    let mut maze: Vec<Vec<Option<Node>>> = Vec::with_capacity(height);
    let mut start: Option<Point> = None;
    for (y, line) in file.lines().enumerate() {
        // println!("{:?}", line);
        width = line.len();
        let mut maze_line: Vec<Option<Node>> = Vec::with_capacity(height);
        for (x, char) in line.chars().enumerate() {
            match char {
                '|' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::South,
                })),
                '-' => maze_line.push(Some(Node {
                    a: Direction::West,
                    b: Direction::East,
                })),
                'L' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::East,
                })),
                'J' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::West,
                })),
                '7' => maze_line.push(Some(Node {
                    a: Direction::South,
                    b: Direction::West,
                })),
                'F' => maze_line.push(Some(Node {
                    a: Direction::South,
                    b: Direction::East,
                })),
                'S' => {
                    maze_line.push(None);
                    start = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '.' => maze_line.push(None),
                _ => continue,
            };
        }
        maze.push(maze_line);
    }
    let start = match start {
        Some(s) => s,
        _ => panic!("Could not find the start"),
    };

    let mut searchers: Vec<Searcher> = vec![
        Searcher {
            searching: true,
            location: Point {
                x: start.x,
                y: start.y + 1,
            },
            coming_from: Direction::North,
            count: 1,
        },
        Searcher {
            searching: true,
            location: Point {
                x: start.x,
                y: start.y - 1,
            },
            coming_from: Direction::South,
            count: 1,
        },
        Searcher {
            searching: true,
            location: Point {
                x: start.x + 1,
                y: start.y,
            },
            coming_from: Direction::West,
            count: 1,
        },
        Searcher {
            searching: true,
            location: Point {
                x: start.x - 1,
                y: start.y,
            },
            coming_from: Direction::East,
            count: 1,
        },
    ];

    loop {
        for searcher in searchers.iter_mut() {
            if !searcher.searching {
                continue;
            }

            if searcher.location.x == start.x && searcher.location.y == start.y {
                return searcher.count / 2;
            }

            let node = match get_node(searcher.location, &maze, width, height) {
                Some(n) => n,
                _ => {
                    searcher.searching = false;
                    continue;
                }
            };

            let mut new_direction = if searcher.coming_from == node.a {
                node.b
            } else if searcher.coming_from == node.b {
                node.a
            } else {
                searcher.searching = false;
                continue;
            };
            new_direction = match new_direction {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            };

            let new_location = match new_direction {
                Direction::North => Point {
                    x: searcher.location.x,
                    y: searcher.location.y + 1,
                },
                Direction::South => Point {
                    x: searcher.location.x,
                    y: searcher.location.y - 1,
                },
                Direction::West => Point {
                    x: searcher.location.x + 1,
                    y: searcher.location.y,
                },
                Direction::East => Point {
                    x: searcher.location.x - 1,
                    y: searcher.location.y,
                },
            };

            searcher.location = new_location;
            searcher.coming_from = new_direction;
            searcher.count += 1;
        }
    }
}

fn task_2(file: String) -> u32 {
    let height = file.lines().count();
    let mut width = 0;
    let mut maze: Vec<Vec<Option<Node>>> = Vec::with_capacity(height);
    let mut start: Option<Point> = None;
    for (y, line) in file.lines().enumerate() {
        // println!("{:?}", line);
        width = line.len();
        let mut maze_line: Vec<Option<Node>> = Vec::with_capacity(height);
        for (x, char) in line.chars().enumerate() {
            match char {
                '|' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::South,
                })),
                '-' => maze_line.push(Some(Node {
                    a: Direction::West,
                    b: Direction::East,
                })),
                'L' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::East,
                })),
                'J' => maze_line.push(Some(Node {
                    a: Direction::North,
                    b: Direction::West,
                })),
                '7' => maze_line.push(Some(Node {
                    a: Direction::South,
                    b: Direction::West,
                })),
                'F' => maze_line.push(Some(Node {
                    a: Direction::South,
                    b: Direction::East,
                })),
                'S' => {
                    maze_line.push(None);
                    start = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '.' => maze_line.push(None),
                _ => continue,
            };
        }
        maze.push(maze_line);
    }
    let start = match start {
        Some(s) => s,
        _ => panic!("Could not find the start"),
    };

    let mut searchers: Vec<MazeSearcher> = vec![
        MazeSearcher {
            searching: true,
            location: Point {
                x: start.x,
                y: start.y + 1,
            },
            coming_from: Direction::North,
            count: 1,
            graph: Vec::new(),
        },
        MazeSearcher {
            searching: true,
            location: Point {
                x: start.x,
                y: start.y - 1,
            },
            coming_from: Direction::South,
            count: 1,
            graph: Vec::new(),
        },
        MazeSearcher {
            searching: true,
            location: Point {
                x: start.x + 1,
                y: start.y,
            },
            coming_from: Direction::West,
            count: 1,
            graph: Vec::new(),
        },
        MazeSearcher {
            searching: true,
            location: Point {
                x: start.x - 1,
                y: start.y,
            },
            coming_from: Direction::East,
            count: 1,
            graph: Vec::new(),
        },
    ];

    for searcher in searchers.iter_mut() {
        searcher.graph.push(start);
        searcher.graph.push(searcher.location);
    }

    let working_index;
    'main_loop: loop {
        for (i, searcher) in searchers.iter_mut().enumerate() {
            if !searcher.searching {
                continue;
            }

            if searcher.location.x == start.x && searcher.location.y == start.y {
                working_index = i;
                break 'main_loop;
            }

            let node = match get_node(searcher.location, &maze, width, height) {
                Some(n) => n,
                _ => {
                    searcher.searching = false;
                    continue;
                }
            };

            let mut new_direction = if searcher.coming_from == node.a {
                node.b
            } else if searcher.coming_from == node.b {
                node.a
            } else {
                searcher.searching = false;
                continue;
            };
            new_direction = match new_direction {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            };

            let new_location = match new_direction {
                Direction::North => Point {
                    x: searcher.location.x,
                    y: searcher.location.y + 1,
                },
                Direction::South => Point {
                    x: searcher.location.x,
                    y: searcher.location.y - 1,
                },
                Direction::West => Point {
                    x: searcher.location.x + 1,
                    y: searcher.location.y,
                },
                Direction::East => Point {
                    x: searcher.location.x - 1,
                    y: searcher.location.y,
                },
            };

            searcher.location = new_location;
            searcher.coming_from = new_direction;
            searcher.count += 1;

            searcher.graph.push(new_location);
        }
    }

    let graph = (searchers[working_index].graph).clone();
    // println!("{:?}", graph);
    let mut lines = Vec::new();
    for window in graph.windows(2) {
        // println!();
        // println!("{:?} {:?}", window[0], window[1]);
        if window[1].x != window[0].x {
            continue;
        }
        // println!("is valid");
        let line = Segment {
            start_x: window[0].x * 2 + 1,
            start_y: window[0].y * 2 + 1,
            end_x: window[1].x * 2 + 1,
            end_y: window[1].y * 2 + 1,
        };
        // println!("{:?}", line);
        lines.push(line);
    }

    let mut lines_col = Vec::with_capacity(width);
    for _ in 0..width {
        lines_col.push(Vec::new());
    }
    for line in &lines {
        let x = ((line.start_x - 1) / 2) as usize;
        lines_col[x].push(line);
    }

    let mut maze_matrix: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for point in graph {
        maze_matrix[point.y as usize][point.x as usize] = true;
    }

    let mut counter = 0;
    for y in 0..height {
        let mut inside = false;
        for x in 0..width {
            let step = Segment {
                start_x: x as i32 * 2,
                start_y: y as i32 * 2,
                end_x: (x as i32 + 1) * 2,
                end_y: y as i32 * 2,
            };
        
            let mut intersect = false;
            for line in &lines_col[x] {
                if segment_intersect(**line, step) {
                    intersect = true;
                    break;
                }
            }

            if intersect {
                inside = !inside;
                continue;
            }

            if inside {
                if maze_matrix[y][x] {
                    continue;
                }
                counter += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};
    use std::fs;

    fn task_test(path: &str, task: fn(String) -> u32, result: u32) {
        let file = fs::read_to_string(path).expect("Error, could not read file");
        let res = task(file);
        assert_eq!(res, result);
    }

    #[test]
    fn task_1_test() {
        task_test("test1", task_1, 8);
        task_test("input", task_1, 6831);
    }

    #[test]
    fn task_2_test() {
        task_test("test2", task_2, 10);
        task_test("input", task_2, 305);
    }
}
