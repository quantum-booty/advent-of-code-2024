#![allow(unused)]
#![feature(let_chains)]
use Object::*;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn parse(input: &str, grid_size: usize) -> (Vec<(usize, usize)>, Vec<Vec<Object>>, Vector, Vector) {
    let coords = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    let mut grid = (0..grid_size)
        .map(|_| (0..grid_size).map(|_| Empty).collect_vec())
        .collect_vec();

    let start = Vector { x: 0, y: 0 };
    let end = Vector {
        x: grid_size - 1,
        y: grid_size - 1,
    };
    (coords, grid, start, end)
}

#[derive(Debug, Clone, Copy)]
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
}

fn solution_a(
    coords: &[(usize, usize)],
    grid: &mut Vec<Vec<Object>>,
    start: Vector,
    end: Vector,
    byte_count: usize,
) -> Option<u32> {
    // pretty_print(&grid, start);

    for &(x, y) in coords.iter().take(byte_count) {
        grid[y][x] = Wall;
    }
    // bfs
    let mut seen = HashSet::<Vector>::new();
    seen.insert(start);
    let mut queue = VecDeque::<(Vector, u32)>::new();
    queue.push_back((start, 0));
    while let Some((pos, cost)) = queue.pop_front() {
        if pos == end {
            return Some(cost);
        }

        for neighbour in pos.get_neighbours(&grid) {
            if !seen.insert(neighbour) {
                continue;
            };
            queue.push_back((neighbour, cost + 1));
        }
    }

    None
}

fn solution_b(
    coords: Vec<(usize, usize)>,
    mut grid: Vec<Vec<Object>>,
    start: Vector,
    end: Vector,
    initial_byte_count_guess: usize,
) -> (usize, usize) {
    let mut byte_count = initial_byte_count_guess;
    while let Some(_) = solution_a(&coords, &mut grid, start, end, byte_count) {
        byte_count += 1;
    }
    coords[byte_count - 1]
}

fn pretty_print(grid: &[Vec<Object>], pos: Vector) {
    let pretty_str = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, object)| {
                    if pos.x == x && pos.y == y {
                        'O'
                    } else {
                        match object {
                            Empty => '.',
                            Wall => '#',
                        }
                    }
                })
                .join("")
        })
        .join("\n");
    println!("{}", pretty_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (coords, mut grid, start, end) = parse(TEST_INPUT_A1, 7);
        assert_eq!(solution_a(&coords, &mut grid, start, end, 12).unwrap(), 22);
        let (coords, mut grid, start, end) = parse(INPUT, 71);
        println!(
            "{:?}",
            solution_a(&coords, &mut grid, start, end, 1024).unwrap()
        );

        let (coords, grid, start, end) = parse(TEST_INPUT_A1, 7);
        assert_eq!(solution_b(coords, grid, start, end, 12), (6, 1));
        let (coords, grid, start, end) = parse(INPUT, 71);
        println!("{:?}", solution_b(coords, grid, start, end, 1024));
    }

    const TEST_INPUT_A1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    const INPUT: &str = include_str!("input.txt");
}
