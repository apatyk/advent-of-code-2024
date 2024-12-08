use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(6);

fn determine_distinct_guard_positions(
    bounds: (usize, usize),
    mut guard_pos: (usize, usize),
    mut guard_dir: char,
    obstacle_positions: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    // move guard throughout the grid according to movement and rotation rules
    let mut distinct_positions: HashSet<(usize, usize)> = HashSet::new();
    while guard_pos.0 < bounds.0 && guard_pos.1 < bounds.1 {
        distinct_positions.insert(guard_pos);
        // up
        if guard_dir == '^' {
            let next_pos = (guard_pos.0, guard_pos.1 - 1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '>';
            } else {
                guard_pos = next_pos;
            }
        // right
        } else if guard_dir == '>' {
            let next_pos = (guard_pos.0 + 1, guard_pos.1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = 'v';
            } else {
                guard_pos = next_pos;
            }
        // down
        } else if guard_dir == 'v' {
            let next_pos = (guard_pos.0, guard_pos.1 + 1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '<';
            } else {
                guard_pos = next_pos;
            }
        // left
        } else {
            let next_pos = (guard_pos.0 - 1, guard_pos.1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '^';
            } else {
                guard_pos = next_pos;
            }
        }
    }

    distinct_positions
}

fn check_path_loop(
    bounds: (usize, usize),
    guard_pos: (usize, usize),
    mut guard_dir: char,
    obstacle_positions: &HashSet<(usize, usize)>,
) -> bool {
    // cast to safe versions for negative bounds spillover
    let bounds: (i32, i32) = (bounds.0 as i32, bounds.1 as i32);
    let mut guard_pos: (i32, i32) = (guard_pos.0 as i32, guard_pos.1 as i32);
    let obstacle_positions: HashSet<(i32, i32)> = obstacle_positions
        .iter()
        .map(|(pos_x, pos_y)| (*pos_x as i32, *pos_y as i32))
        .collect();

    // move guard throughout the grid according to movement and rotation rules
    // check if guard gets stuck in a loop by checking if the iterations exceed
    // a reasonable vaue extrapolated from part 1
    let mut distinct_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut is_loop = false;
    let mut n_iterations = 0;
    while guard_pos.0 >= 0 && guard_pos.0 < bounds.0 && guard_pos.1 >= 0 && guard_pos.1 < bounds.1 {
        if n_iterations > 10_000 {
            // twice length of entire path from part 1
            // in a loop
            is_loop = true;
            break;
        }
        n_iterations += 1;
        distinct_positions.insert(guard_pos);
        // up
        if guard_dir == '^' {
            let next_pos = (guard_pos.0, guard_pos.1 - 1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '>';
            } else {
                guard_pos = next_pos;
            }
        // right
        } else if guard_dir == '>' {
            let next_pos = (guard_pos.0 + 1, guard_pos.1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = 'v';
            } else {
                guard_pos = next_pos;
            }
        // down
        } else if guard_dir == 'v' {
            let next_pos = (guard_pos.0, guard_pos.1 + 1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '<';
            } else {
                guard_pos = next_pos;
            }
        // left
        } else {
            let next_pos = (guard_pos.0 - 1, guard_pos.1);
            if obstacle_positions.contains(&next_pos) {
                guard_dir = '^';
            } else {
                guard_pos = next_pos;
            }
        }
    }

    is_loop
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"^|>|<|v").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    // determine positions of obstacles, position & direction of guard
    let mut guard_pos: (usize, usize) = (0, 0);
    let mut guard_dir: char = 'x';
    let mut obstacle_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] != '.' {
                if grid[i][j] == '#' {
                    obstacle_positions.insert((j, i));
                } else if re.is_match(&grid[i][j].to_string()) {
                    guard_dir = grid[i][j];
                    guard_pos = (j, i);
                }
            }
        }
    }

    let positions = determine_distinct_guard_positions(
        (n_cols, n_rows),
        guard_pos,
        guard_dir,
        &obstacle_positions,
    );

    // debug printing
    // for i in 0..n_rows {
    //     for j in 0..n_cols {
    // if positions.contains(&(j, i)) {
    //     print!("X");
    //         } else if obstacle_positions.contains(&(j, i)) {
    //             print!("#");
    //         } else if guard_pos == (j, i) {
    //             print!("{}", guard_dir);
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    Some(positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"^|>|<|v").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    // determine positions of obstacles, position & direction of guard
    let mut guard_pos: (usize, usize) = (0, 0);
    let mut guard_dir: char = 'x';
    let mut obstacle_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] != '.' {
                if grid[i][j] == '#' {
                    obstacle_positions.insert((j, i));
                } else if re.is_match(&grid[i][j].to_string()) {
                    guard_dir = grid[i][j];
                    guard_pos = (j, i);
                }
            }
        }
    }
    let initial_guard_pos = guard_pos;
    let initial_guard_dir = guard_dir;

    // use possible guard positions from part one as potential obstacle locations
    let guard_positions = determine_distinct_guard_positions(
        (n_cols, n_rows),
        guard_pos,
        guard_dir,
        &obstacle_positions,
    );

    let mut num_obstruction_positions: u32 = 0;
    // try adding an obstacle to each location and checking for a loop
    for position in guard_positions {
        let mut test_positions = obstacle_positions.clone();
        test_positions.insert(position);

        let creates_loop = check_path_loop(
            (n_cols, n_rows),
            initial_guard_pos,
            initial_guard_dir,
            &test_positions,
        );
        if creates_loop {
            num_obstruction_positions += 1
        }
    }

    Some(num_obstruction_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
