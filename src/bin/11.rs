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

pub fn part_two(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("integers exist"))
        .collect();

    // run for 75 blinks (iterations)
    for _ in 0..75 {
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
        assert_eq!(result, None);
    }
}
