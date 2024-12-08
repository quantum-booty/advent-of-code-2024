#![allow(unused)]
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
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

fn get_a_to_b_vec(a: &Vector, b: &Vector) -> Vector {
    Vector {
        x: b.x - a.x,
        y: b.y - a.y,
    }
}

fn parse_grid(input: &str) -> (HashMap<char, Vec<Vector>>, Vector) {
    let y_len = input.lines().count();
    let x_len = input.lines().next().unwrap().len();
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '.' => None,
                _ => Some((c, Vector {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                })),
            })
        })
        .filter_map(|x| x);
    let mut antennas_by_frequency = HashMap::new();
    for (frequency, vec) in antennas {
        antennas_by_frequency
            .entry(frequency)
            .or_insert(vec![])
            .push(vec);
    }
    (antennas_by_frequency, Vector {
        x: x_len.try_into().unwrap(),
        y: y_len.try_into().unwrap(),
    })
}

fn check_valid_position_vector(p: Vector, grid_size: Vector) -> bool {
    0 <= p.x && p.x < grid_size.x && 0 <= p.y && p.y < grid_size.y
}

fn solution_a(antennas: &HashMap<char, Vec<Vector>>, grid_size: Vector) -> usize {
    let mut unique_nodes = HashSet::new();
    for (_, position_vectors) in antennas {
        for (a, b) in position_vectors.iter().tuple_combinations() {
            let a_to_b = get_a_to_b_vec(a, b);
            let c = *a - a_to_b;
            let d = *b + a_to_b;
            if check_valid_position_vector(c, grid_size) {
                unique_nodes.insert(c);
            }
            if check_valid_position_vector(d, grid_size) {
                unique_nodes.insert(d);
            }
        }
    }
    unique_nodes.len()
}

fn solution_b(antennas: &HashMap<char, Vec<Vector>>, grid_size: Vector) -> usize {
    let mut unique_nodes = HashSet::new();
    for (_, position_vectors) in antennas {
        for (a, b) in position_vectors.iter().tuple_combinations() {
            let a_to_b = get_a_to_b_vec(a, b);
            unique_nodes.insert(*a);
            unique_nodes.insert(*b);
            let mut c = *a - a_to_b;
            let mut d = *b + a_to_b;
            while check_valid_position_vector(c, grid_size) {
                unique_nodes.insert(c);
                c = c - a_to_b;
            }
            while check_valid_position_vector(d, grid_size) {
                unique_nodes.insert(d);
                d = d + a_to_b;
            }
        }
    }
    unique_nodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (antennas, grid_size) = parse_grid(TEST_INPUT);
        assert_eq!(solution_a(&antennas, grid_size), 14);

        let (antennas, grid_size) = parse_grid(INPUT);
        println!("{}", solution_a(&antennas, grid_size));

        let (antennas, grid_size) = parse_grid(TEST_INPUT_B);
        assert_eq!(solution_b(&antennas, grid_size), 9);

        let (antennas, grid_size) = parse_grid(TEST_INPUT);
        assert_eq!(solution_b(&antennas, grid_size), 34);

        let (antennas, grid_size) = parse_grid(INPUT);
        println!("{}", solution_b(&antennas, grid_size));
    }

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const TEST_INPUT_B: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    const INPUT: &str = include_str!("input.txt");
}
