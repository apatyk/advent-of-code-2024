advent_of_code::solution!(2);

fn process_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn vec_diff(vec: &[u32]) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::with_capacity(vec.len() - 1);

    for i in 1..vec.len() {
        differences.push(vec[i] as i32 - vec[i - 1] as i32);
    }

    differences
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = process_input(input);
    let mut safe_count = 0;

    for line in lines {
        let report_values: Vec<u32> = line
            .split(' ')
            .map(|n| n.parse().expect("ints exist"))
            .collect();

        let differences = vec_diff(&report_values);

        // count reports monotonically increasing  or decreasing by 1-3
        if differences.iter().all(|&d| (1..=3).contains(&d))
            || differences.iter().all(|&d| (-3..=-1).contains(&d))
        {
            safe_count += 1;
        };
    }

    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = process_input(input);
    let mut safe_count = 0;

    for line in lines {
        let mut report_values: Vec<u32> = line
            .split(' ')
            .map(|n| n.parse().expect("ints exist"))
            .collect();

        let differences = vec_diff(&report_values);

        // count reports monotonically increasing by 1-3
        if differences.iter().all(|&d| (1..=3).contains(&d)) {
            safe_count += 1;
        // count reports monotonically decreasing by 1-3
        } else if differences.iter().all(|&d| (-3..=-1).contains(&d)) {
            safe_count += 1;
        } else {
            // greedily check removals to tolerate a single level
            for i in 0..report_values.len() {
                let removed_value = report_values.remove(i);
                let intermediate_diffs = vec_diff(&report_values);

                if intermediate_diffs.iter().all(|&d| (1..=3).contains(&d))
                    || intermediate_diffs.iter().all(|&d| (-3..=-1).contains(&d))
                {
                    safe_count += 1;
                    break;
                };

                report_values.insert(i, removed_value);
            }
        };
    }

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
