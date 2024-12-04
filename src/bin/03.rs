advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<m1>\d+),(?<m2>\d+)\)").unwrap();

    // find all instances of mul(m1,m2) where m1 & m2 are integers
    let multiplicands: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|m| {
            let m1 = m
                .name("m1")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("first int");
            let m2 = m
                .name("m2")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("second int");

            (m1, m2)
        })
        .collect();

    let sum: i32 = multiplicands.iter().map(|(m1, m2)| m1 * m2).sum();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"mul\((?<m1>\d+),(?<m2>\d+)\)").unwrap();
    let split_re = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut extra_do_input = String::from("do()");
    extra_do_input.push_str(input);

    // find all instances of do()/don't() toggles
    let toggles: Vec<&str> = split_re
        .find_iter(&extra_do_input)
        .map(|toggle| toggle.as_str())
        .collect();
    // split input into sections starting with do() or don't()
    let splits: Vec<&str> = split_re
        .split(&extra_do_input)
        .filter(|&split| !split.is_empty())
        .collect();
    // prepend split segments with do()/don't() toggles
    let splits_with_toggles: Vec<String> = splits
        .into_iter()
        .enumerate()
        .map(|(i, part)| {
            let mut split_with_toggle = String::from(toggles[i]);
            split_with_toggle.push_str(part);

            split_with_toggle
        })
        .collect();

    // find all instances of mul(m1,m2) where m1 & m2 are integers
    // but only in segments that start with "do()"
    let mut sum = 0;
    for split in splits_with_toggles {
        if split.starts_with("do()") {
            let multiplicands: Vec<(i32, i32)> = mul_re
                .captures_iter(&split)
                .map(|m| {
                    let m1 = m
                        .name("m1")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .expect("first int");
                    let m2 = m
                        .name("m2")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .expect("second int");

                    (m1, m2)
                })
                .collect();
            sum += multiplicands.iter().map(|(m1, m2)| m1 * m2).sum::<i32>();
        };
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(52));
    }
}
