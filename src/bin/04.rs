advent_of_code::solution!(4);

use regex::Regex;

fn process_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"XMAS").unwrap();

    let input_lines = process_input(input);

    let orthogonal_len = input_lines[0].len();
    let diagonal_len = (orthogonal_len * 2) - 1;

    let horizontal_lines: Vec<String> = input_lines.clone();
    let mut vertical_lines: Vec<String> = vec![String::new(); orthogonal_len];
    let mut lr_diagonal_lines: Vec<String> = vec![String::new(); diagonal_len];
    let mut rl_diagonal_lines: Vec<String> = vec![String::new(); diagonal_len];

    // collect vertical, L->R diagonal, and R->L diagonal lines of characters
    for (n, line) in input_lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            vertical_lines[i].push(c);
            if n + i < diagonal_len {
                rl_diagonal_lines[n + i].push(c);
            }
        }
        for (i, c) in line.chars().rev().enumerate() {
            if n + i < diagonal_len {
                lr_diagonal_lines[n + i].push(c);
            }
        }
    }

    let mut total_count = 0;
    let all_lines = [
        horizontal_lines,
        vertical_lines,
        rl_diagonal_lines,
        lr_diagonal_lines,
    ]
    .concat();

    // count instances of 'XMAS' in each line forward and backward
    for line in all_lines {
        total_count += re.find_iter(&line).count();
        total_count += re
            .find_iter(&line.chars().rev().collect::<String>())
            .count();
    }

    Some(total_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"MAS").unwrap();

    let input_lines = process_input(input);

    let diagonal_len = (input_lines[0].len() * 2) - 1;
    let mut lr_diagonal_lines: Vec<String> = vec![String::new(); diagonal_len];
    let mut rl_diagonal_lines: Vec<String> = vec![String::new(); diagonal_len];

    // collect vertical, L->R diagonal, and R->L diagonal lines of characters
    for (n, line) in input_lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if n + i < diagonal_len {
                rl_diagonal_lines[n + i].push(c);
            }
        }
        for (i, c) in line.chars().rev().enumerate() {
            if n + i < diagonal_len {
                lr_diagonal_lines[n + i].push(c);
            }
        }
    }

    let mut total_count = 0;
    let all_lines = [rl_diagonal_lines, lr_diagonal_lines].concat();

    // count instances of 'MAS' in each line forward and backward
    // WIP: need to find intersections
    for line in all_lines {
        total_count += re.find_iter(&line).count();
        total_count += re
            .find_iter(&line.chars().rev().collect::<String>())
            .count();
    }

    Some(total_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
