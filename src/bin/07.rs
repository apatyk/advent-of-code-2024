advent_of_code::solution!(7);

fn get_operator_patterns(operators: &[char], len: usize) -> Vec<Vec<char>> {
    // determine all combinations of operator patterns based on number of operators needed
    let mut result = operators
        .iter()
        .map(|&c| vec![c])
        .collect::<Vec<Vec<char>>>();
    for _ in 1..len {
        result = result
            .into_iter()
            .flat_map(|s| {
                operators
                    .iter()
                    .map(move |&c| [s.clone(), vec![c]].concat())
            })
            .collect();
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_calibration_result: u64 = 0;

    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let split_line = line.split_once(":").expect("line matches input");
            let test_value: u64 = split_line.0.parse().expect("integer test value exists");
            let numbers: Vec<u64> = split_line
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse().expect("integers exist"))
                .collect();
            (test_value, numbers)
        })
        .collect();

    for (test_value, equation_numbers) in equations {
        let mut match_found = false;
        // check each possible operator pattern to determine if it could produce the test value
        let operator_patterns = get_operator_patterns(&['+', '*'], equation_numbers.len() - 1);
        for ops in operator_patterns {
            let mut equation_total: u64 = equation_numbers[0];
            for (i, &num) in equation_numbers.iter().skip(1).enumerate() {
                if ops[i] == '+' {
                    equation_total += num;
                } else {
                    equation_total *= num;
                }
            }
            if equation_total == test_value {
                match_found = true;
                break;
            }
        }
        if match_found {
            total_calibration_result += test_value;
        }
    }

    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_calibration_result: u64 = 0;

    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let split_line = line.split_once(":").expect("line matches input");
            let test_value: u64 = split_line.0.parse().expect("integer test value exists");
            let numbers: Vec<u64> = split_line
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse().expect("integers exist"))
                .collect();
            (test_value, numbers)
        })
        .collect();

    for (test_value, equation_numbers) in equations {
        let mut match_found = false;
        // check each possible operator pattern to determine if it could produce the test value
        let operator_patterns = get_operator_patterns(&['+', '*', '|'], equation_numbers.len() - 1);
        for ops in operator_patterns {
            let mut equation_total: u64 = equation_numbers[0];
            for (i, &num) in equation_numbers.iter().skip(1).enumerate() {
                if ops[i] == '+' {
                    equation_total += num;
                } else if ops[i] == '*' {
                    equation_total *= num;
                // handle concatenation operator (|)
                } else {
                    let new_num: u64 = (equation_total.to_string() + &num.to_string())
                        .parse()
                        .expect("concatenation produces new number");
                    equation_total = new_num;
                }
            }
            if equation_total == test_value {
                match_found = true;
                break;
            }
        }
        if match_found {
            total_calibration_result += test_value;
        }
    }

    Some(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
