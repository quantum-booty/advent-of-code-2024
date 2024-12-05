#![allow(unused)]
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solution_a(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() - 3 {
        for x in 0..grid[0].len() - 3 {
            if (is_xmas(
                grid[y][x],
                grid[y + 1][x + 1],
                grid[y + 2][x + 2],
                grid[y + 3][x + 3],
            )) {
                // right down diagonal
                count += 1;
            }
            if (is_xmas(
                grid[y + 3][x],
                grid[y + 2][x + 1],
                grid[y + 1][x + 2],
                grid[y][x + 3],
            )) {
                // up right diagonal
                count += 1;
            }
        }
    }
    for y in 0..grid.len() - 3 {
        for x in 0..grid[0].len() {
            if (is_xmas(grid[y][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x])) {
                // vertical
                count += 1;
            }
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[0].len() - 3 {
            if (is_xmas(grid[y][x], grid[y][x + 1], grid[y][x + 2], grid[y][x + 3])) {
                // horizontal
                count += 1;
            }
        }
    }
    count
}

fn is_xmas(a: char, b: char, c: char, d: char) -> bool {
    a == 'X' && b == 'M' && c == 'A' && d == 'S' || d == 'X' && c == 'M' && b == 'A' && a == 'S'
}

fn is_mas(a: char, b: char, c: char) -> bool {
    a == 'M' && b == 'A' && c == 'S' || c == 'M' && b == 'A' && a == 'S'
}

fn solution_b(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() - 2 {
        for x in 0..grid[0].len() - 2 {
            let is_right_down_diagonal = is_mas(grid[y][x], grid[y + 1][x + 1], grid[y + 2][x + 2]);
            let is_up_right_diagonal = is_mas(grid[y + 2][x], grid[y + 1][x + 1], grid[y][x + 2]);
            if (is_right_down_diagonal && is_up_right_diagonal) {
                count += 1;
            }
            let is_middle_vertical = is_mas(grid[y][x + 1], grid[y + 1][x + 1], grid[y + 2][x + 1]);
            let is_middle_horizontal =
                is_mas(grid[y + 1][x], grid[y + 1][x + 1], grid[y + 1][x + 2]);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = parse(TEST_INPUT);
        assert_eq!(solution_a(&grid), 18);
        let grid = parse(INPUT);
        println!("{}", solution_a(&grid));

        let grid = parse(TEST_INPUT);
        assert_eq!(solution_b(&grid), 9);
        let grid = parse(INPUT);
        println!("{}", solution_b(&grid));
    }

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT: &str = include_str!("input.txt");
}
