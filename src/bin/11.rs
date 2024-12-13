use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("integers exist"))
        .collect();

    // run for 25 blinks (iterations)
    for _ in 0..25 {
        let len = stones.len();
        let mut new_stones = Vec::with_capacity(len * 3 / 2);
        for stone in stones {
            // rule 1 - if the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
            if stone == 0 {
                new_stones.push(1);
            // rule 2 - if the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let half_len = (stone.ilog10() + 1) / 2;
                new_stones.push(stone / 10_u64.pow(half_len));
                new_stones.push(stone % 10_u64.pow(half_len));
            // rule 3 - if none of the other rules apply, the stone is replaced by a stone with the old stone's number multiplied by 2024.
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    let num_stones = stones.len() as u32;
    Some(num_stones)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("integers exist"))
        .collect();

    // keep a hashmap keyed on stone numbers storing the count of each
    let mut stones_map: HashMap<u64, u64> = HashMap::with_capacity(5000);
    for stone in stones.clone() {
        stones_map.insert(stone, 1);
    }

    // run for 75 blinks (iterations)
    // update or insert count for each stone
    for _ in 0..75 {
        let mut updates: HashMap<u64, u64> = HashMap::with_capacity(stones_map.len());
        for stone in stones.clone() {
            let count = *stones_map.get(&stone).unwrap();

            // rule 1 - if the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
            if stone == 0 {
                *updates.entry(1).or_insert(0) += count;
            // rule 2 - if the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let half_len = (stone.ilog10() + 1) / 2;
                let old_stone = stone / 10_u64.pow(half_len);
                let new_stone = stone % 10_u64.pow(half_len);
                *updates.entry(old_stone).or_insert(0) += count;
                *updates.entry(new_stone).or_insert(0) += count;
            // rule 3 - if none of the other rules apply, the stone is replaced by a stone with the old stone's number multiplied by 2024.
            } else {
                *updates.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stones_map = updates;
        stones = stones_map.keys().cloned().collect();
    }

    let num_stones = stones_map.into_values().sum();
    Some(num_stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
