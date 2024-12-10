use std::collections::HashSet;

advent_of_code::solution!(10);

fn check_trails_score(
    grid: &Vec<Vec<u32>>,
    pos: (i32, i32),
    bounds: (i32, i32),
) -> HashSet<(usize, usize)> {
    // recursively check if trails exist in each direction from a starting point
    // and return the set of reachable peak positions used to calculate trail score
    let (i, j) = (pos.0 as usize, pos.1 as usize);
    let height = grid[i][j];
    let mut unique_peaks = HashSet::new();

    // end of a trail detected - valid path
    if height == 9 {
        unique_peaks.insert((i, j));
        return unique_peaks;
    };
    // outside bounds without reaching height 9 - invalid path
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= bounds.0 || pos.1 >= bounds.1 {
        return HashSet::new();
    }

    // check each of the cardinal directions
    // north
    if pos.0 > 0 && grid[i - 1][j] == height + 1 {
        unique_peaks.extend(check_trails_score(grid, (pos.0 - 1, pos.1), bounds));
    }
    // south
    if pos.0 + 1 < bounds.0 && grid[i + 1][j] == height + 1 {
        unique_peaks.extend(check_trails_score(grid, (pos.0 + 1, pos.1), bounds));
    }
    // east
    if pos.1 + 1 < bounds.1 && grid[i][j + 1] == height + 1 {
        unique_peaks.extend(check_trails_score(grid, (pos.0, pos.1 + 1), bounds));
    }
    // west
    if pos.1 > 0 && grid[i][j - 1] == height + 1 {
        unique_peaks.extend(check_trails_score(grid, (pos.0, pos.1 - 1), bounds));
    }

    unique_peaks
}

fn check_trails_rating(grid: &Vec<Vec<u32>>, pos: (i32, i32), bounds: (i32, i32)) -> u32 {
    // recursively check if trails exist in each direction from a starting point
    // and return number of all possible trails (trail rating)
    let (i, j) = (pos.0 as usize, pos.1 as usize);
    let height = grid[i][j];
    let mut rating = 0;

    // end of a trail detected - valid path
    if height == 9 {
        return 1;
    };
    // outside bounds without reaching height 9 - invalid path
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= bounds.0 || pos.1 >= bounds.1 {
        return 0;
    }

    // check each of the cardinal directions
    // north
    if pos.0 > 0 && grid[i - 1][j] == height + 1 {
        rating += check_trails_rating(grid, (pos.0 - 1, pos.1), bounds);
    }
    // south
    if pos.0 + 1 < bounds.0 && grid[i + 1][j] == height + 1 {
        rating += check_trails_rating(grid, (pos.0 + 1, pos.1), bounds);
    }
    // east
    if pos.1 + 1 < bounds.1 && grid[i][j + 1] == height + 1 {
        rating += check_trails_rating(grid, (pos.0, pos.1 + 1), bounds);
    }
    // west
    if pos.1 > 0 && grid[i][j - 1] == height + 1 {
        rating += check_trails_rating(grid, (pos.0, pos.1 - 1), bounds);
    }

    rating
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(9)).collect())
        .collect();
    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    let mut total_trailhead_score: u32 = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            // start from possible trailheads (0)
            if grid[i][j] != 0 {
                continue;
            }
            let unique_peaks =
                check_trails_score(&grid, (i as i32, j as i32), (n_rows as i32, n_cols as i32));
            let trailhead_score = unique_peaks.len() as u32;
            total_trailhead_score += trailhead_score;
        }
    }

    Some(total_trailhead_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(9)).collect())
        .collect();
    let n_rows = grid.len();
    let n_cols = grid.first().expect("first row exists").len();

    let mut total_trailhead_score: u32 = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            // start from possible trailheads (0)
            if grid[i][j] != 0 {
                continue;
            }
            let rating =
                check_trails_rating(&grid, (i as i32, j as i32), (n_rows as i32, n_cols as i32));
            total_trailhead_score += rating;
        }
    }

    Some(total_trailhead_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
