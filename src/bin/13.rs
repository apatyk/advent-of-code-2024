use std::{fmt::Error, str::FromStr};

use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl FromStr for ClawMachine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let re = Regex::new(r".*A:\sX\+(?<ax>\d+),\sY\+(?<ay>\d+)\s.*B:\sX\+(?<bx>\d+),\sY\+(?<by>\d+)\s.*Prize:\sX=(?<px>\d+),\sY=(?<py>\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        Ok(Self {
            button_a: (
                captures["ax"].parse().expect("unable to parse number"),
                captures["ay"].parse().expect("unable to parse number")
            ), 
            button_b: (
                captures["bx"].parse().expect("unable to parse number"),
                captures["by"].parse().expect("unable to parse number")
            ), 
            prize: (
                captures["px"].parse().expect("unable to parse number"),
                captures["py"].parse().expect("unable to parse number")
            )
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let claw_machines: Vec<ClawMachine> = split_input.iter().map(|&i| ClawMachine::from_str(i).unwrap()).collect();

    // solve system of equations and choose minimal solution
    // not possible if number of tokens exceeds 100
    let mut num_tokens: u32 = 0;
    for claw_machine in claw_machines {
        let a_tokens: u32 = 0;
        let b_tokens: u32 = 0;

        // not implemented yet

        num_tokens += a_tokens + b_tokens;
    }

    Some(num_tokens)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
