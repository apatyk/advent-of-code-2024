advent_of_code::solution!(1);

fn process_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut first_col: Vec<u32> = vec![];
    let mut second_col: Vec<u32> = vec![];

    // parse integers from each row into vectors by column
    let lines = process_input(input);
    for line in lines {
        let values: Vec<u32> = line
            .split("   ")
            .map(|val| val.parse::<u32>().expect("value exists"))
            .collect();
        first_col.push(values[0]);
        second_col.push(values[1]);
    }

    // sort columns independently
    first_col.sort();
    second_col.sort();

    // calculate sum of differences between pairs of sorted values
    let value_sets = first_col.into_iter().zip(second_col);
    let differences = value_sets.map(|(val1, val2)| val1.abs_diff(val2));

    Some(differences.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first_col: Vec<u32> = vec![];
    let mut second_col: Vec<u32> = vec![];

    // parse integers from each row into vectors by column
    let lines = process_input(input);
    for line in lines {
        let values: Vec<u32> = line
            .split("   ")
            .map(|val| val.parse::<u32>().expect("value exists"))
            .collect();
        first_col.push(values[0]);
        second_col.push(values[1]);
    }

    let similarity_scores = first_col
        .into_iter()
        .map(|val| val * second_col.iter().filter(|&n| *n == val).count() as u32);

    Some(similarity_scores.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
