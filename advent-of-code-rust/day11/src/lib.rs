#![allow(unused)]
#![feature(let_chains)]
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count_digits(number: u64) -> u32 {
    let mut digits = 0;
    let mut x = number;
    while (x != 0) {
        x /= 10;
        digits += 1;
    }
    digits
}

fn try_split_number(number: u64) -> Option<(u64, u64)> {
    let count = count_digits(number);
    if count % 2 == 0 {
        let half = 10_u64.pow(count / 2);
        let left = number / half;
        let right = number % half;
        return Some((left, right));
    }
    None
}

fn solution(stones: &[u64], blinks: usize) -> u64 {
    let mut count_by_stone = HashMap::<u64, u64>::new();
    for &stone in stones {
        count_by_stone
            .entry(stone)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    for blink_num in 0..blinks {
        let mut new_count_by_stone = HashMap::<u64, u64>::new();
        for (stone, count) in count_by_stone {
            if stone == 0 {
                let new_stone = 1;
                new_count_by_stone
                    .entry(new_stone)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            } else if let Some((left, right)) = try_split_number(stone) {
                new_count_by_stone
                    .entry(left)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
                new_count_by_stone
                    .entry(right)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            } else {
                let new_stone = stone * 2024;
                new_count_by_stone
                    .entry(new_stone)
                    .and_modify(|x| *x += count)
                    .or_insert(count);
            }
        }
        count_by_stone = new_count_by_stone;
    }
    count_by_stone.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stones = parse(TEST_INPUT_A1);
        assert_eq!(solution(&stones, 1), 7);

        let stones = parse(TEST_INPUT_A2);
        assert_eq!(solution(&stones, 6), 22);

        let stones = parse(TEST_INPUT_A2);
        assert_eq!(solution(&stones, 25), 55312);

        let stones = parse(INPUT);
        println!("{}", solution(&stones, 25));

        let stones = parse(INPUT);
        println!("{}", solution(&stones, 75));
    }

    const TEST_INPUT_A1: &str = "0 1 10 99 999";

    const TEST_INPUT_A2: &str = "125 17";

    const INPUT: &str = include_str!("input.txt");
}
