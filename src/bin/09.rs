advent_of_code::solution!(9);

#[derive(Clone, Debug, PartialEq)]
enum BlockType {
    File,
    FreeSpace,
}

#[derive(Clone, Debug)]
struct Block {
    id: usize,
    kind: BlockType,
    size: u32,
}

fn decompress(disk_map: &str) -> Vec<Block> {
    // expand compressed disk map into full file blocks
    let mut output_map: Vec<Block> = vec![];

    let disk_map_vec: Vec<u32> = disk_map
        .chars()
        .map(|c| c.to_digit(10).expect("single line of 0-9 digits"))
        .collect();

    for (i, &size) in disk_map_vec.iter().enumerate() {
        // alternate between file & free space sizes
        if i % 2 == 0 {
            let id = i / 2;
            for _ in 0..size {
                output_map.push(Block {
                    id,
                    kind: BlockType::File,
                    size,
                })
            }
        } else {
            for _ in 0..size {
                output_map.push(Block {
                    id: 0,
                    kind: BlockType::FreeSpace,
                    size,
                })
            }
        }
    }

    output_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut all_blocks = decompress(input);

    // collect reversed list of indices for file blocks to consume
    let rev_file_block_idxs: Vec<usize> = all_blocks
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if b.kind == BlockType::File {
                Some(i)
            } else {
                None
            }
        })
        .rev()
        .collect();

    // fill in free space with file blocks from end of map
    let mut rev_pointer: usize = 0;
    for (i, block) in all_blocks.clone().into_iter().enumerate() {
        if block.kind != BlockType::FreeSpace {
            continue;
        }
        if all_blocks[i..]
            .iter()
            .all(|b| b.kind == BlockType::FreeSpace)
        {
            break;
        }

        // swap FreeSpace block with File Block
        all_blocks.swap(i, rev_file_block_idxs[rev_pointer]);
        rev_pointer += 1;
    }

    let mut checksum: u64 = 0;
    for (i, block) in all_blocks.into_iter().enumerate() {
        checksum += (block.id * i) as u64;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut all_blocks = decompress(input);

    // collect list of indices for file blocks to consume
    let mut file_block_idxs: Vec<usize> = all_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if b.kind == BlockType::File {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // TODO: Fix sweep from beginning, not progressive
    for (i, block) in all_blocks.clone().into_iter().enumerate() {
        if block.kind != BlockType::FreeSpace {
            continue;
        }

        if file_block_idxs.is_empty() {
            break;
        }

        let mut next_file_block = *file_block_idxs.last().expect("next file block exists");
        let file_block_size = all_blocks[next_file_block].size;
        let free_space_size = block.size;

        if file_block_size > free_space_size {
            // no FreeSpace block fits File Block - skip File Block entirely
            if all_blocks
                .iter()
                .filter(|&b| b.kind == BlockType::FreeSpace)
                .all(|b| b.size < file_block_size)
            {
                for _ in 1..file_block_size {
                    file_block_idxs.pop();
                }
            }
            while file_block_idxs.len() > 1 && all_blocks[next_file_block].size > free_space_size {
                file_block_idxs.pop();
                next_file_block = *file_block_idxs.last().expect("next file block exists");
            }
        // swap FreeSpace block with File Block if it fits
        } else {
            all_blocks.swap(i, next_file_block);
            file_block_idxs.pop();
        }

        // debug printing
        // for b in &all_blocks {
        //     match b.kind {
        //         BlockType::File => print!("{}", b.id),
        //         BlockType::FreeSpace => print!("."),
        //     }
        // }
        // println!();
    }

    let mut checksum: u64 = 0;
    for (i, block) in all_blocks.into_iter().enumerate() {
        checksum += (block.id * i) as u64;
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
