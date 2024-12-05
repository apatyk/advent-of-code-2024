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

    let mut count: u32 = 0;
    let all_lines = [
        horizontal_lines,
        vertical_lines,
        rl_diagonal_lines,
        lr_diagonal_lines,
    ]
    .concat();

    // count instances of 'XMAS' in each line forward and backward
    for line in all_lines {
        count += re.find_iter(&line).count() as u32;
        count += re
            .find_iter(&line.chars().rev().collect::<String>())
            .count() as u32;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    // collapse multi-line input into a single character slice
    let input_str = input.lines().collect::<String>();
    let input_chars = input_str.chars();

    let width = input.lines().next().expect("first line exists").len();

    // find each occurrence of 'A' and check each corner for an 'M' or 'S'
    //
    //  TL  |  *  |  TR
    // -----|-----|-----
    //   *  |  A  |  *
    // -----|-----|-----
    //  BL  |  *  |  BR
    //
    // use 1-D representation of grid and corresponding notation
    let mut count: u32 = 0;
    for (i, char) in input_chars.enumerate() {
        if char != 'A' {
            continue;
        }

        let safe_sub_check = (i as i32 - width as i32) > 0;
        let on_edge_check = i % width == 0 || i % width == width - 1;
        if !safe_sub_check || on_edge_check {
            continue;
        }

        let tl = input_str.chars().nth(i - width - 1).unwrap_or(' ');
        let br = input_str.chars().nth(i + width + 1).unwrap_or(' ');
        let tr = input_str.chars().nth(i - width + 1).unwrap_or(' ');
        let bl = input_str.chars().nth(i + width - 1).unwrap_or(' ');

        let lr_mas = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
        let rl_mas = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');
        if lr_mas && rl_mas {
            count += 1;
        }
    }

    Some(count)
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
