#![allow(unused)]
#![feature(let_chains)]
use Object::*;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

fn parse(input: &str) -> (Vec<Vec<Object>>, Vector, Vec<Vector>) {
    let input = input.replace("\r\n", "\n");
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();
    let grid = grid_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' => Empty,
                    '@' => Robot,
                    'O' => BoxThing,
                    _ => panic!("never should happen!"),
                })
                .collect()
        })
        .collect();

    let robot = grid_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '@' => Some(Vector {
                    x: x as i32,
                    y: y as i32,
                }),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let moves = moves_str
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '<' => Vector { x: -1, y: 0 },
            '>' => Vector { x: 1, y: 0 },
            '^' => Vector { x: 0, y: -1 },
            'v' => Vector { x: 0, y: 1 },
            _ => panic!("never should happen!"),
        })
        .collect();
    (grid, robot, moves)
}

fn parse_b(input: &str) -> (Vec<Vec<Object>>, Vector, Vec<Vector>) {
    let input = input.replace("\r\n", "\n");
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();
    let grid = grid_str
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => vec![Wall, Wall],
                    '.' => vec![Empty, Empty],
                    '@' => vec![Robot, Empty],
                    'O' => vec![BoxLeft, BoxRight],
                    _ => panic!("never should happen!"),
                })
                .collect()
        })
        .collect();

    let robot = grid_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '@' => Some(Vector {
                    x: (x * 2) as i32,
                    y: y as i32,
                }),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let moves = moves_str
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '<' => Vector { x: -1, y: 0 },
            '>' => Vector { x: 1, y: 0 },
            '^' => Vector { x: 0, y: -1 },
            'v' => Vector { x: 0, y: 1 },
            _ => panic!("never should happen!"),
        })
        .collect();
    (grid, robot, moves)
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn left(&self) -> Vector {
        Vector {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Vector {
        Vector {
            x: self.x + 1,
            y: self.y,
        }
    }
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

#[derive(Debug, Clone, Copy)]
enum Object {
    Wall,
    BoxThing,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

fn try_get_next_position(old: Vector, dv: Vector, grid: &[Vec<Object>]) -> Option<Vector> {
    let (new_x, new_y) = (old.x + dv.x, old.y + dv.y);
    if 0 <= new_x && new_x < grid[0].len() as i32 && 0 <= new_y && new_y < grid.len() as i32 {
        return Some(Vector { x: new_x, y: new_y });
    }
    None
}

fn check_can_move(grid: &[Vec<Object>], pos: Vector, dv: Vector) -> bool {
    let maybe_next_position = try_get_next_position(pos, dv, grid);
    if maybe_next_position.is_none() {
        return false;
    }
    let new_pos = maybe_next_position.unwrap();
    let next_object = grid[new_pos.y as usize][new_pos.x as usize];
    match next_object {
        Wall => false,
        BoxThing => check_can_move(grid, new_pos, dv),
        Empty => true,
        Robot => true,
        BoxLeft => {
            dv.y == 0 && check_can_move(grid, new_pos, dv)
                || check_can_move(grid, new_pos, dv) && check_can_move(grid, new_pos.right(), dv)
        }
        BoxRight => {
            dv.y == 0 && check_can_move(grid, new_pos, dv)
                || check_can_move(grid, new_pos, dv) && check_can_move(grid, new_pos.left(), dv)
        }
    }
}

fn try_move(grid: &mut Vec<Vec<Object>>, pos: Vector, dv: Vector) {
    let new_pos = try_get_next_position(pos, dv, grid).unwrap();
    let curr_object = grid[pos.y as usize][pos.x as usize];
    match curr_object {
        BoxLeft => {
            if dv.y != 0 {
                try_move(grid, new_pos.right(), dv);
                grid[new_pos.y as usize][(new_pos.x + 1) as usize] =
                    grid[pos.y as usize][(pos.x + 1) as usize];
                grid[pos.y as usize][(pos.x + 1) as usize] = Empty;
            }
        }
        BoxRight => {
            if dv.y != 0 {
                try_move(grid, new_pos.left(), dv);
                grid[new_pos.y as usize][(new_pos.x - 1) as usize] =
                    grid[pos.y as usize][(pos.x - 1) as usize];
                grid[pos.y as usize][(pos.x - 1) as usize] = Empty;
            }
        }
        Wall => return,
        Empty => return,
        BoxThing => (),
        Robot => (),
    };
    try_move(grid, new_pos, dv);
    grid[new_pos.y as usize][new_pos.x as usize] = grid[pos.y as usize][pos.x as usize];
    grid[pos.y as usize][pos.x as usize] = Empty;
}

fn solution(
    mut grid: Vec<Vec<Object>>,
    mut robot: Vector,
    moves: Vec<Vector>,
    part_2: bool,
    do_print: bool,
) -> usize {
    for dv in moves {
        if do_print {
            println!("{:?} {}", dv, check_can_move(&grid, robot, dv));
        }
        if check_can_move(&grid, robot, dv) {
            try_move(&mut grid, robot, dv);
            robot = robot + dv;
        }
        if do_print {
            pretty_print(&grid);
        }
    }
    if part_2 {
        let mut res = 0;
        for (y, line) in grid.iter().enumerate() {
            let mut x = 0;
            while x < grid[0].len() - 1 {
                if let (BoxLeft, BoxRight) = (line[x], line[x + 1]) {
                    res += 100 * y + x;
                    x += 2;
                } else {
                    x += 1;
                }
            }
        }
        res
    } else {
        grid.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, object)| match object {
                        BoxThing => Some(100 * y + x),
                        _ => None,
                    })
            })
            .sum()
    }
}

fn pretty_print(grid: &[Vec<Object>]) {
    let pretty_str = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|object| match object {
                    Wall => '#',
                    BoxThing => 'O',
                    Robot => '@',
                    Empty => '.',
                    BoxLeft => '[',
                    BoxRight => ']',
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
        let (grid, robot, moves) = parse(TEST_INPUT_A1);
        assert_eq!(solution(grid, robot, moves, false, false), 2028);
        let (grid, robot, moves) = parse(TEST_INPUT_A2);
        assert_eq!(solution(grid, robot, moves, false, false), 10092);
        let (grid, robot, moves) = parse(INPUT);
        println!("{}", solution(grid, robot, moves, false, false));

        let (grid, robot, moves) = parse_b(TEST_INPUT_B2);
        solution(grid, robot, moves, true, false);

        let (grid, robot, moves) = parse_b(TEST_INPUT_B4);
        solution(grid, robot, moves, true, false);

        let (grid, robot, moves) = parse_b(TEST_INPUT_A2);
        assert_eq!(solution(grid, robot, moves, true, false), 9021);

        let (grid, robot, moves) = parse_b(INPUT);
        println!("{}", solution(grid, robot, moves, true, false));
    }

    const TEST_INPUT_A1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_INPUT_A2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const TEST_INPUT_B1: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const TEST_INPUT_B2: &str = "#######
#...#.#
#..O..#
#..O@.#
#.....#
#.....#
#######

<v<^";

    const TEST_INPUT_B3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^";

    const TEST_INPUT_B4: &str = "#######
#.#...#
#.....#
#.OOO.#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^";

    const INPUT: &str = include_str!("input.txt");
}
