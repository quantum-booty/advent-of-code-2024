#![allow(unused)]
#![feature(let_chains)]
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

fn parse(input: &str) -> Vec<Robot> {
    let regex = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let numbers = regex
                .find_iter(line)
                .filter_map(|digits| digits.as_str().parse::<i32>().ok())
                .collect_vec();
            Robot {
                position: Vector {
                    x: numbers[0],
                    y: numbers[1],
                },
                velocity: Vector {
                    x: numbers[2],
                    y: numbers[3],
                },
            }
        })
        .collect()
}

fn solution(
    robots: Vec<Robot>,
    width: i32,
    height: i32,
    iterations: usize,
    do_pretty_print: bool,
) -> usize {
    let mut robots = robots;
    for iteration in 0..iterations {
        for robot in &mut robots {
            let mut new_position = robot.position + robot.velocity;
            if new_position.x < 0 {
                new_position.x = width + new_position.x;
            } else if new_position.x >= width {
                new_position.x = new_position.x % width;
            }
            if new_position.y < 0 {
                new_position.y = height + new_position.y;
            } else if new_position.y >= height {
                new_position.y = new_position.y % height;
            }
            robot.position = new_position;
        }
        if do_pretty_print {
            pretty_print(&robots, width, height, iteration);
        }
    }
    count_block(&robots, 0, (width - 1) / 2 - 1, 0, (height - 1) / 2 - 1) // upper left
        * count_block(&robots, 0, (width - 1) / 2 - 1, (height - 1) / 2 + 1, height) // lower left
        * count_block(&robots, (width - 1) / 2 + 1, width, 0, (height - 1) / 2 - 1) // upper right
        * count_block( // lower right
            &robots,
            (width - 1) / 2 + 1,
            width,
            (height - 1) / 2 + 1,
            height,
        )
}

fn count_block(robots: &[Robot], min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> usize {
    robots
        .iter()
        .filter(|robot| {
            min_x <= robot.position.x
                && robot.position.x <= max_x
                && min_y <= robot.position.y
                && robot.position.y <= max_y
        })
        .count()
}

fn pretty_print(robots: &[Robot], width: i32, height: i32, iteration: usize) {
    let mut chars_array: Vec<Vec<char>> = (0..height)
        .map(|_| (0..width).map(|_| ' ').collect())
        .collect();
    for robot in robots {
        chars_array[robot.position.y as usize][robot.position.x as usize] = 'x'
    }
    let mut total_distance = 0;
    let mut pairs_count = 0;
    for line in &chars_array {
        for (a, b) in line
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == 'x' { Some(i) } else { None })
            .tuple_windows()
        {
            total_distance += b - a;
            pairs_count += 1;
        }
    }
    let average_distance = total_distance / pairs_count;
    if average_distance < 8 {
        let string = chars_array
            .iter()
            .map(|chars| chars.iter().join(""))
            .join("\n");
        println!("{} {}", iteration + 1, average_distance);
        println!("{}", string);
        println!(
            "---------------------------------------------------------------------------------------------------------"
        );
        println!(
            "---------------------------------------------------------------------------------------------------------"
        );
        println!(
            "---------------------------------------------------------------------------------------------------------"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let robots = parse(TEST_INPUT_A1);
        assert_eq!(solution(robots, 11, 7, 100, false), 12);
        let robots = parse(INPUT);
        println!("{}", solution(robots, 101, 103, 100, false));
        let robots = parse(INPUT);
        println!("{}", solution(robots, 101, 103, 10000, true));
    }

    const TEST_INPUT_A1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    const INPUT: &str = include_str!("input.txt");
}
