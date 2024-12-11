advent_of_code::solution!(9);

#[derive(Debug, Clone, PartialEq)]
enum BlockType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    kind: BlockType,
}

#[derive(Debug, Clone)]
struct BlockSpan {
    data: Vec<usize>,
    kind: BlockType,
    free: usize,
}

impl BlockSpan {
    fn occupy(&mut self, data: &[usize]) {
        self.data.extend(data);
        self.free -= data.len();
        if self.free == 0 {
            self.kind = BlockType::File;
        }
    }
    fn free(&mut self) {
        self.free = self.data.len();
        self.data.clear();
        self.kind = BlockType::FreeSpace;
    }
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
                })
            }
        } else {
            for _ in 0..size {
                output_map.push(Block {
                    id: 0,
                    kind: BlockType::FreeSpace,
                })
            }
        }
    }

    output_map
}

fn decompress_spans(disk_map: &str) -> Vec<BlockSpan> {
    // expand compressed disk map into block spans
    let mut output_map: Vec<BlockSpan> = vec![];

    let disk_map_vec: Vec<u32> = disk_map
        .chars()
        .map(|c| c.to_digit(10).expect("single line of 0-9 digits"))
        .collect();

    for (i, &size) in disk_map_vec.iter().enumerate() {
        // alternate between file & free space sizes
        if i % 2 == 0 {
            let id = i / 2;
            output_map.push(BlockSpan {
                kind: BlockType::File,
                data: vec![id; size as usize],
                free: 0,
            })
        } else {
            output_map.push(BlockSpan {
                kind: BlockType::FreeSpace,
                data: Vec::new(),
                free: size as usize,
            })
        }
    }

    output_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = decompress(input);

    // collect reversed list of indices for file blocks to consume
    let rev_file_block_idxs: Vec<usize> = disk
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
    for (i, block) in disk.clone().into_iter().enumerate() {
        if block.kind != BlockType::FreeSpace {
            continue;
        }
        if disk[i..].iter().all(|b| b.kind == BlockType::FreeSpace) {
            break;
        }

        // swap FreeSpace block with File Block
        disk.swap(i, rev_file_block_idxs[rev_pointer]);
        rev_pointer += 1;
    }

    let checksum: u64 = disk
        .iter()
        .enumerate()
        .map(|(i, block)| (block.id * i) as u64)
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = decompress_spans(input);

    // iterate through files right to left
    let mut right = disk.len() - 1;
    while right > 0 {
        if disk[right].kind != BlockType::File {
            right -= 1;
            continue;
        }
        // iterate through free space left to right
        let mut left = 0;
        while left < right {
            // skip over files or free space that is too small
            if disk[left].kind != BlockType::FreeSpace || disk[right].data.len() > disk[left].free {
                left += 1;
                continue;
            }

            // if free space is found, move the file
            let file_data = disk[right].data.clone();
            disk[left].occupy(&file_data);
            disk[right].free();

            break;
        }
        right -= 1;

        // debug printing
        // for b in &disk {
        //     for i in &b.data {
        //         print!("{}", i);
        //     };
        //     for _ in 0..b.free {
        //         print!(".");
        //     }
        // }
        // println!()
    }

    let mut checksum: u64 = 0;
    let mut index: u64 = 0;
    for span in disk {
        for id in span.data {
            checksum += index * id as u64;
            index += 1;
        }
        for _ in 0..span.free {
            index += 1;
        }
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
