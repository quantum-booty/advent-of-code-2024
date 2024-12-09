#![allow(unused)]
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Block {
    File { id: usize },
    Empty,
}

fn parse(input: &str) -> Vec<Block> {
    let digits = input
        .replace("\r\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    expand_disk(digits)
}

fn expand_disk(input: Vec<u32>) -> Vec<Block> {
    let mut res = Vec::new();
    let mut id = 0;
    for (position, digit) in input.iter().enumerate() {
        if position % 2 != 0 {
            for _ in 0..*digit {
                res.push(Block::Empty);
            }
        } else {
            for _ in 0..*digit {
                res.push(Block::File { id });
            }
            id += 1;
        }
    }
    res
}

fn is_empty(block: &Block) -> bool {
    match block {
        Block::File { id } => false,
        Block::Empty => true,
    }
}

fn compact_disk(disk: Vec<Block>) -> Vec<Block> {
    let mut left_idx = 0;
    let mut compacted = disk.to_vec();
    for (right_idx, block) in disk.iter().enumerate().rev() {
        match block {
            Block::File { id } => {
                while (!is_empty(&disk[left_idx])) {
                    left_idx += 1;
                }
                if (left_idx >= right_idx) {
                    break;
                }
                compacted.swap(left_idx, right_idx);
                left_idx += 1;
            }
            Block::Empty => continue,
        }
    }
    compacted
}

fn check_sum(disk: Vec<Block>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(position, block)| match block {
            Block::File { id } => position * id,
            Block::Empty => 0,
        })
        .sum()
}

fn solution_a(disk: Vec<Block>) -> usize {
    check_sum(compact_disk(disk))
}

fn compact_disk_b(disk: Vec<Block>) -> Vec<Block> {
    let mut compacted = disk.to_vec();

    let mut white_spaces = Vec::new();
    let mut white_space_chunk_position = 0;
    for chunk in get_chunks(&disk) {
        if (is_empty(&chunk[0])) {
            white_spaces.push((white_space_chunk_position, chunk.len()));
        }
        white_space_chunk_position += chunk.len();
    }

    let mut chunk_position = disk.len();
    for chunk in get_chunks(&disk).rev() {
        chunk_position -= chunk.len();
        if (is_empty(&chunk[0])) {
            continue;
        }

        for (white_space_chunk_position, white_space_len) in white_spaces.iter_mut() {
            if *white_space_chunk_position >= chunk_position {
                break;
            }
            if *white_space_len >= chunk.len() {
                for i in 0..chunk.len() {
                    compacted.swap(*white_space_chunk_position + i, chunk_position + i);
                }

                *white_space_chunk_position += chunk.len();
                *white_space_len -= chunk.len();
                break;
            }
        }
    }

    compacted
}

fn get_chunks(disk: &Vec<Block>) -> impl DoubleEndedIterator<Item = &[Block]> {
    disk.chunk_by(|a, b| match (a, b) {
        (Block::File { id: id1 }, Block::File { id: id2 }) => id1 == id2,
        (Block::Empty, Block::Empty) => true,
        _ => false,
    })
}

fn solution_b(disk: Vec<Block>) -> usize {
    check_sum(compact_disk_b(disk))
}

fn pretty_print_disk(disk: &Vec<Block>) {
    let pretty_repr = disk
        .iter()
        .map(|block| match block {
            Block::File { id } => id.to_string(),
            Block::Empty => String::from_str(".").unwrap(),
        })
        .join("");
    println!("{}", pretty_repr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let disk = parse(TEST_INPUT_A1);
        assert_eq!(solution_a(disk), 60);

        let disk = parse(TEST_INPUT_A2);
        assert_eq!(solution_a(disk), 1928);

        let disk = parse(INPUT);
        println!("{}", solution_a(disk));

        let disk = parse(TEST_INPUT_A2);
        assert_eq!(solution_b(disk), 2858);

        let disk = parse(INPUT);
        println!("{}", solution_b(disk));
    }

    const TEST_INPUT_A1: &str = "12345";

    const TEST_INPUT_A2: &str = "2333133121414131402";

    const INPUT: &str = include_str!("input.txt");
}
