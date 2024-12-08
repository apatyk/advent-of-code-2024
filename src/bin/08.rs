use std::collections::HashSet;

advent_of_code::solution!(8);

fn check_bounds(i: i32, j: i32, n_rows: usize, n_cols: usize) -> bool {
    let (n_rows, n_cols) = (n_rows as i32, n_cols as i32);

    i >= 0 && i < n_rows && j >= 0 && j < n_cols
}

fn process_antinode_positions(
    bounds: (usize, usize),
    positions: &[(i32, i32)],
    antinode_positions: &mut HashSet<(usize, usize)>,
) {
    for &(i_pos, j_pos) in positions {
        if check_bounds(i_pos, j_pos, bounds.0, bounds.1) {
            antinode_positions.insert((i_pos as usize, j_pos as usize));
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] == '.' {
                continue;
            }
            // for all characters that are not '.' search for others like it
            // propagate out on each diagonal within n_rows x n_cols grid
            let c = grid[i][j];
            // safe cast for bounds checking
            let (i, j) = (i as i32, j as i32);
            for c_dist in 1..n_cols {
                for r_dist in 1..n_rows {
                    // safe cast for bounds checking
                    let (r_dist, c_dist) = (r_dist as i32, c_dist as i32);
                    // ↘️
                    if check_bounds(i + r_dist, j + c_dist, n_rows, n_cols)
                        && c == grid[(i + r_dist) as usize][(j + c_dist) as usize]
                    {
                        // check both antinode positions
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &[(i - r_dist, j - c_dist), (i + 2 * r_dist, j + 2 * c_dist)],
                            &mut antinode_positions,
                        );
                    }
                    // ↖
                    if check_bounds(i - r_dist, j - c_dist, n_rows, n_cols)
                        && c == grid[(i - r_dist) as usize][(j - c_dist) as usize]
                    {
                        // check both antinode positions
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &[(i + r_dist, j + c_dist), (i - 2 * r_dist, j - 2 * c_dist)],
                            &mut antinode_positions,
                        );
                    }
                    // ↙️
                    if check_bounds(i + r_dist, j - c_dist, n_rows, n_cols)
                        && c == grid[(i + r_dist) as usize][(j - c_dist) as usize]
                    {
                        // check both antinode positions
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &[(i - r_dist, j + c_dist), (i + 2 * r_dist, j - 2 * c_dist)],
                            &mut antinode_positions,
                        );
                    }
                    // ↗️
                    if check_bounds(i - r_dist, j + c_dist, n_rows, n_cols)
                        && c == grid[(i - r_dist) as usize][(j + c_dist) as usize]
                    {
                        // check both antinode positions
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &[(i + r_dist, j - c_dist), (i - 2 * r_dist, j + 2 * c_dist)],
                            &mut antinode_positions,
                        );
                    }
                }
            }
        }
    }

    // debug printing
    // for i in 0..n_rows {
    //     for j in 0..n_cols {
    //         print!("{}", grid[i][j]);
    //     }
    //     println!();
    // }

    let antinode_count = antinode_positions.len() as u32;
    Some(antinode_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            if grid[i][j] == '.' {
                continue;
            }
            let c = grid[i][j];
            // for all characters that are not '.' search for others like it
            // propagate out on each diagonal within n_rows x n_cols grid
            // safe cast for bounds checking
            let (i, j) = (i as i32, j as i32);
            for c_dist in 1..n_cols {
                for r_dist in 1..n_rows {
                    // safe cast for bounds checking
                    let (r_dist, c_dist) = (r_dist as i32, c_dist as i32);
                    // ↘️
                    if check_bounds(i + r_dist, j + c_dist, n_rows, n_cols)
                        && c == grid[(i + r_dist) as usize][(j + c_dist) as usize]
                    {
                        // check all antinode positions
                        let all_possible_positions: Vec<(i32, i32)> = [
                            (0..n_rows)
                                .map(|scale| (i - scale as i32 * r_dist, j - scale as i32 * c_dist))
                                .collect::<Vec<(i32, i32)>>(),
                            (0..n_rows)
                                .map(|scale| (i + scale as i32 * r_dist, j + scale as i32 * c_dist))
                                .collect(),
                        ]
                        .concat();
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &all_possible_positions,
                            &mut antinode_positions,
                        );
                    }
                    // ↖
                    if check_bounds(i - r_dist, j - c_dist, n_rows, n_cols)
                        && c == grid[(i - r_dist) as usize][(j - c_dist) as usize]
                    {
                        // check all antinode positions
                        let all_possible_positions: Vec<(i32, i32)> = [
                            (1..n_rows)
                                .map(|scale| (i + scale as i32 * r_dist, j + scale as i32 * c_dist))
                                .collect::<Vec<(i32, i32)>>(),
                            (1..n_rows)
                                .map(|scale| (i - scale as i32 * r_dist, j - scale as i32 * c_dist))
                                .collect(),
                        ]
                        .concat();
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &all_possible_positions,
                            &mut antinode_positions,
                        );
                    }
                    // ↙️
                    if check_bounds(i + r_dist, j - c_dist, n_rows, n_cols)
                        && c == grid[(i + r_dist) as usize][(j - c_dist) as usize]
                    {
                        // check all antinode positions
                        let all_possible_positions: Vec<(i32, i32)> = [
                            (1..n_rows)
                                .map(|scale| (i - scale as i32 * r_dist, j + scale as i32 * c_dist))
                                .collect::<Vec<(i32, i32)>>(),
                            (1..n_rows)
                                .map(|scale| (i + scale as i32 * r_dist, j - scale as i32 * c_dist))
                                .collect(),
                        ]
                        .concat();
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &all_possible_positions,
                            &mut antinode_positions,
                        );
                    }
                    // ↗️
                    if check_bounds(i - r_dist, j + c_dist, n_rows, n_cols)
                        && c == grid[(i - r_dist) as usize][(j + c_dist) as usize]
                    {
                        // check all antinode positions
                        let all_possible_positions: Vec<(i32, i32)> = [
                            (1..n_rows)
                                .map(|scale| (i + scale as i32 * r_dist, j - scale as i32 * c_dist))
                                .collect::<Vec<(i32, i32)>>(),
                            (1..n_rows)
                                .map(|scale| (i - scale as i32 * r_dist, j + scale as i32 * c_dist))
                                .collect(),
                        ]
                        .concat();
                        process_antinode_positions(
                            (n_rows, n_cols),
                            &all_possible_positions,
                            &mut antinode_positions,
                        );
                    }
                }
            }
        }
    }

    // debug printing
    // for i in 0..n_rows {
    //     for j in 0..n_cols {
    //         if antinode_positions.contains(&(i, j)) {
    //             print!("#");
    //         }
    //         else {
    //             print!("{}", grid[i][j]);
    //         }
    //     }
    //     println!();
    // }

    let antinode_count = antinode_positions.len() as u32;
    Some(antinode_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
