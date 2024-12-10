#![allow(unused)]
#![feature(let_chains)]
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

static DIRECTIONS: &'static [(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn try_move(x: usize, y: usize, dx: i32, dy: i32, grid: &[Vec<u32>]) -> Option<(usize, usize)> {
    let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
    if 0 <= new_x && new_x < grid[0].len() as i32 && 0 <= new_y && new_y < grid.len() as i32 {
        return Some((new_x as usize, new_y as usize));
    }
    None
}

fn get_neighbours(x: usize, y: usize, grid: &[Vec<u32>]) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.iter().filter_map(move |(dx, dy)| {
        if let Some((new_x, new_y)) = try_move(x, y, *dx, *dy, grid)
            && (grid[new_y][new_x] == grid[y][x] + 1)
        {
            return Some((new_x, new_y));
        }
        return None;
    })
}

fn find_starts(grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(
                move |(x, height)| {
                    if *height == 0 { Some((x, y)) } else { None }
                },
            )
        })
        .collect()
}

fn count_peaks(start: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut count = 0;
    while let Some((x, y)) = stack.pop() {
        if (!visited.insert((x, y))) {
            continue;
        }
        if grid[y][x] == 9 {
            count += 1;
            continue;
        }
        for (new_x, new_y) in get_neighbours(x, y, grid) {
            stack.push((new_x, new_y));
        }
    }
    count
}

fn solution_a(grid: &[Vec<u32>]) -> usize {
    let starts = find_starts(grid);
    starts.iter().map(|start| count_peaks(*start, grid)).sum()
}

fn topological_sort(start: (usize, usize), grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut in_degree = HashMap::new();
    in_degree.insert(start, 0);
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    while let Some((x, y)) = stack.pop() {
        if (!visited.insert((x, y))) {
            continue;
        }
        if grid[y][x] == 9 {
            continue;
        }

        for (new_x, new_y) in get_neighbours(x, y, grid) {
            in_degree
                .entry((new_x, new_y))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            stack.push((new_x, new_y));
        }
    }

    let mut topological_order = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some((x, y)) = queue.pop_front() {
        topological_order.push((x, y));
        for (new_x, new_y) in get_neighbours(x, y, grid) {
            let degree = in_degree.get_mut(&(new_x, new_y)).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push_back((new_x, new_y));
            }
        }
    }

    topological_order
}

fn count_paths(start: (usize, usize), grid: &[Vec<u32>]) -> usize {
    let topological_order = topological_sort(start, grid);
    let mut dp = HashMap::<(usize, usize), usize>::new();
    for &(x, y) in &topological_order {
        if grid[y][x] == 9 {
            dp.insert((x, y), 1);
        }
    }
    for &(x, y) in topological_order.iter().rev() {
        for (new_x, new_y) in get_neighbours(x, y, grid) {
            let neighbour_count = *dp.entry((new_x, new_y)).or_default();
            dp.entry((x, y))
                .and_modify(|count| *count += neighbour_count)
                .or_insert(neighbour_count);
        }
    }
    dp[&start]
}

fn solution_b(grid: &[Vec<u32>]) -> usize {
    let starts = find_starts(grid);
    starts.iter().map(|start| count_paths(*start, grid)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = parse(TEST_INPUT_A1);
        assert_eq!(solution_a(&grid), 1);
        //
        let grid = parse(TEST_INPUT_A2);
        assert_eq!(solution_a(&grid), 36);

        let grid = parse(INPUT);
        println!("{}", solution_a(&grid));

        let grid = parse(TEST_INPUT_A3);
        assert_eq!(solution_b(&grid), 3);

        let grid = parse(TEST_INPUT_A2);
        assert_eq!(solution_b(&grid), 81);

        let grid = parse(INPUT);
        println!("{}", solution_b(&grid));
    }

    const TEST_INPUT_A1: &str = "0123
1234
8765
9876";

    const TEST_INPUT_A2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const TEST_INPUT_A3: &str = "1111808
1143211
1151121
1165431
1171141
1187651
1191111";

    const INPUT: &str = include_str!("input.txt");
}
