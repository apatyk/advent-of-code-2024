use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Debug)]
struct PerimeterRegion {
    _name: char, // included for debugging
    area: u32,
    perimeter: u32,
}

#[derive(Debug)]
struct SideRegion {
    _name: char, // included for debugging
    area: u32,
    sides: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = garden.len();
    let n_cols = garden.first().expect("first row exists").len();

    // segment 2D vector into regions based on matching characters and
    // collect a vector of Regions containing area and perimeter
    let mut regions: Vec<PerimeterRegion> = Vec::new();
    let mut visited_plots: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            if visited_plots.contains(&(i, j)) {
                continue;
            }
            let current_plot = garden[i][j];
            let mut region = PerimeterRegion {
                _name: current_plot,
                area: 0,
                perimeter: 0,
            };

            // depth-first search for regions in garden
            let mut stack = vec![(i, j)];
            while let Some((r, c)) = stack.pop() {
                // skip plots if they are not valid
                if r >= n_rows
                    || c >= n_cols
                    || garden[r][c] != current_plot
                    || visited_plots.contains(&(r, c))
                {
                    continue;
                }

                // add neighbor plots to stack for checking and determine number
                // of matching neighbors to calculate perimeter
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                let mut matching_neighbor_count = 0;
                for (nr, nc) in neighbors {
                    let relative_r = (r as isize + nr) as usize;
                    let relative_c = (c as isize + nc) as usize;
                    if relative_r < n_rows && relative_c < n_cols {
                        if garden[relative_r][relative_c] == current_plot {
                            matching_neighbor_count += 1;
                        }
                        stack.push((relative_r, relative_c));
                    }
                }

                visited_plots.insert((r, c));
                region.area += 1;
                region.perimeter += 4 - matching_neighbor_count;
            }
            regions.push(region);
        }
    }

    let total_cost = regions.iter().map(|r| r.area * r.perimeter).sum::<u32>();

    Some(total_cost)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }
}
