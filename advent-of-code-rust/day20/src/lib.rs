#![allow(unused)]
#![feature(let_chains)]
use Object::*;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn find_char(input: &str, char_to_find: char) -> Vector {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == char_to_find {
                    Some((Vector { x, y }))
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap()
}

fn parse(input: &str) -> (Vec<Vec<Object>>, Vector, Vector) {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' => Empty,
                    _ => Empty,
                })
                .collect()
        })
        .collect_vec();

    let start = find_char(input, 'S');
    let end = find_char(input, 'E');

    (grid, start, end)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: usize,
    y: usize,
}

static DIRECTIONS: &'static [(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Vector {
    fn advance(&self, dx: i32, dy: i32) -> Vector {
        Vector {
            x: if dx < 0 {
                self.x.wrapping_sub(dx.abs() as usize)
            } else {
                self.x + dx as usize
            },
            y: if dy < 0 {
                self.y.wrapping_sub(dy.abs() as usize)
            } else {
                self.y + dy as usize
            },
        }
    }

    fn try_advance(&self, grid: &[Vec<Object>], dx: i32, dy: i32) -> Option<Vector> {
        let new_state = self.advance(dx, dy);
        if new_state.x < grid[0].len()
            && new_state.y < grid.len()
            && match grid[new_state.y][new_state.x] {
                Empty => true,
                Wall => false,
            }
        {
            return Some(new_state);
        }
        None
    }

    fn get_neighbours(&self, grid: &[Vec<Object>]) -> impl Iterator<Item = Vector> {
        DIRECTIONS
            .iter()
            .filter_map(move |(dx, dy)| self.try_advance(grid, *dx, *dy))
    }

    fn distance(&self, other: &Vector) -> u32 {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);
        (x_dist + y_dist) as u32
    }
}

fn solution(
    grid: Vec<Vec<Object>>,
    start: Vector,
    end: Vector,
    savings_filter: u32,
    cheat_length: u32,
) -> u32 {
    let dist_from_start = calc_dist_from_start(start, end, &grid);

    let mut count = 0;
    for ((start, &start_cost), (end, &end_cost)) in dist_from_start.iter().tuple_combinations() {
        let dist = start.distance(end);
        if dist <= cheat_length && start_cost.abs_diff(end_cost) - dist >= savings_filter {
            count += 1;
        }
    }
    count
}

fn calc_dist_from_start(start: Vector, end: Vector, grid: &[Vec<Object>]) -> HashMap<Vector, u32> {
    let mut dists_from_start = HashMap::from([(start, 0)]);
    let mut stack = vec![start];
    while let Some(pos) = stack.pop() {
        if pos == end {
            break;
        }

        let cost = dists_from_start[&pos];
        for neighbour in pos.get_neighbours(&grid) {
            if dists_from_start.contains_key(&neighbour) {
                continue;
            }
            dists_from_start.insert(neighbour, cost + 1);
            stack.push(neighbour);
        }
    }

    dists_from_start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (mut grid, start, end) = parse(TEST_INPUT_A1);
        println!("{:?}", solution(grid, start, end, 20, 2));
        let (mut grid, start, end) = parse(INPUT);
        println!("{:?}", solution(grid, start, end, 100, 2));
        let (mut grid, start, end) = parse(INPUT);
        println!("{:?}", solution(grid, start, end, 100, 20));
    }

    const TEST_INPUT_A1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    const INPUT: &str = include_str!("input.txt");
}
