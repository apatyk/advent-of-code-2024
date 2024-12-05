advent_of_code::solution!(5);

fn median(x: &[u32]) -> u32 {
    x[x.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");
    let ordering_rules_input = split.next().expect("first section exists");
    let page_numbers_input = split.next().expect("second section exists");

    // parse input sections into vectors of ordering rules and page number updates
    let ordering_rules: Vec<(u32, u32)> = ordering_rules_input
        .lines()
        .map(|line| {
            let pair: Vec<u32> = line
                .split("|")
                .map(|n| n.parse().expect("integer exists"))
                .collect();

            (pair[0], pair[1])
        })
        .collect();
    let page_numbers: Vec<Vec<u32>> = page_numbers_input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse().expect("integer exist"))
                .collect()
        })
        .collect();

    // iterate over page number updates and check all ordering rules for correct order
    // sum medians of all correctly ordered page number updates
    let mut total: u32 = 0;
    for update in page_numbers {
        let mut correct_order = true;
        for rule in ordering_rules.clone() {
            if let (Some(first_pos), Some(second_pos)) = (
                update.iter().position(|&page_num| page_num == rule.0),
                update.iter().position(|&page_num| page_num == rule.1),
            ) {
                if second_pos < first_pos {
                    correct_order = false;
                    break;
                }
            }
        }
        if correct_order {
            total += median(&update);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
