#![allow(unused)]
#![feature(let_chains)]
use std::collections::HashSet;

use itertools::Itertools;

static DIRECTIONS: &'static [(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn try_move(x: usize, y: usize, dx: i32, dy: i32, grid: &[Vec<char>]) -> Option<(usize, usize)> {
    let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
    if 0 <= new_x && new_x < grid[0].len() as i32 && 0 <= new_y && new_y < grid.len() as i32 {
        return Some((new_x as usize, new_y as usize));
    }
    None
}

fn get_neighbours(x: usize, y: usize, grid: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.iter().filter_map(move |(dx, dy)| {
        if let Some((new_x, new_y)) = try_move(x, y, *dx, *dy, grid)
            && (grid[new_y][new_x] == grid[y][x])
        {
            return Some((new_x, new_y));
        }
        return None;
    })
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solution_a(grid: &[Vec<char>]) -> u32 {
    let mut res = 0;
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut stack = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            stack.push((x, y));
            let mut area = 0;
            let mut total_perimiter = 0;
            while let Some((x, y)) = stack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }
                area += 1;
                let mut neighbour_count = 0;
                for coord in get_neighbours(x, y, grid) {
                    neighbour_count += 1;
                    stack.push(coord);
                }
                let perimeter = 4 - neighbour_count;
                total_perimiter += perimeter;
            }

            res += total_perimiter * area;
        }
    }
    res
}

fn solution_b(grid: &[Vec<char>]) -> u32 {
    let mut res = 0;
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut stack = vec![];
    let mut edges = HashSet::<((usize, usize), (usize, usize))>::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            stack.push((x, y));
            let mut area = 0;
            while let Some((x, y)) = stack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }
                area += 1;

                for &(dx, dy) in DIRECTIONS {
                    if let Some((new_x, new_y)) = try_move(x, y, dx, dy, grid)
                        && grid[new_y][new_x] == grid[y][x]
                    {
                        stack.push((new_x, new_y));
                        continue;
                    }
                    let edge = match (dx, dy) {
                        (1, 0) => ((x + 1, y), (x + 1, y + 1)), // right
                        (-1, 0) => ((x, y), (x, y + 1)),        // left
                        (0, -1) => ((x, y), (x + 1, y)),        // up
                        (0, 1) => ((x, y + 1), (x + 1, y + 1)), // down
                        _ => panic!("never should happen"),
                    };
                    edges.insert(edge);
                }
            }
            let edges_copy = edges.clone();
            let mut total_sides = 0;
            while edges.len() != 0 {
                let (from, to) = *edges.iter().next().unwrap();

                if from.1 == to.1 {
                    // try left
                    let mut left_from = (from.0 - 1, from.1);
                    let mut left_to = from;
                    while !is_divergent(left_to, &edges_copy) && edges.remove(&(left_from, left_to))
                    {
                        left_to = left_from;
                        left_from = (left_from.0 - 1, left_from.1);
                    }

                    // try right
                    let mut right_from = from;
                    let mut right_to = to;
                    while edges.remove(&(right_from, right_to)) {
                        right_from = right_to;
                        right_to = (right_to.0 + 1, right_to.1);
                        if is_divergent(right_from, &edges_copy) {
                            break;
                        }
                    }
                } else {
                    // try up
                    let mut up_from = (from.0, from.1 - 1);
                    let mut up_to = from;
                    while !is_divergent(up_to, &edges_copy) && edges.remove(&(up_from, up_to)) {
                        up_to = up_from;
                        up_from = (up_from.0, up_from.1 - 1);
                    }

                    // try down
                    let mut down_from = from;
                    let mut down_to = to;
                    while edges.remove(&(down_from, down_to)) {
                        down_from = down_to;
                        down_to = (down_to.0, down_to.1 + 1);
                        if is_divergent(down_from, &edges_copy) {
                            break;
                        }
                    }
                }
                total_sides += 1;
            }

            res += total_sides * area;
        }
    }
    res
}

fn is_divergent(coord: (usize, usize), edges: &HashSet<((usize, usize), (usize, usize))>) -> bool {
    let (x, y) = coord;
    let res = edges.contains(&((x, y), (x + 1, y)))
        && edges.contains(&((x - 1, y), (x, y)))
        && edges.contains(&((x, y), (x, y + 1)))
        && edges.contains(&((x, y - 1), (x, y)));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = parse(TEST_INPUT_A1);
        assert_eq!(solution_a(&grid), 140);
        let grid = parse(TEST_INPUT_A2);
        assert_eq!(solution_a(&grid), 772);
        let grid = parse(TEST_INPUT_A3);
        assert_eq!(solution_a(&grid), 1930);
        let grid = parse(INPUT);
        println!("{}", solution_a(&grid));

        let grid = parse(TEST_INPUT_A1);
        assert_eq!(solution_b(&grid), 80);
        let grid = parse(TEST_INPUT_A2);
        assert_eq!(solution_b(&grid), 436);
        let grid = parse(TEST_INPUT_B1);
        assert_eq!(solution_b(&grid), 236);
        let grid = parse(TEST_INPUT_B2);
        assert_eq!(solution_b(&grid), 368);
        let grid = parse(INPUT);
        println!("{}", solution_b(&grid));
    }

    const TEST_INPUT_A1: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST_INPUT_A2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_INPUT_A3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT_B1: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_INPUT_B2: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    const INPUT: &str = include_str!("input.txt");
}
