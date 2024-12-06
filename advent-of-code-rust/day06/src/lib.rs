#![allow(unused)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

impl State {
    fn rotate(&self) -> State {
        let new_direction = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        State {
            x: self.x,
            y: self.y,
            dir: new_direction,
        }
    }

    pub fn advance(
        &self,
        obstacles: &[Vec<bool>],
        new_obstacle: Option<(usize, usize)>,
    ) -> Option<State> {
        let next_pos = State::get_next_pos(self, obstacles);

        if let Some((next_x, next_y)) = next_pos {
            let new_state =
                if obstacles[next_y][next_x] || new_obstacle.is_some_and(|x| (next_x, next_y) == x) {
                    self.rotate()
                } else {
                    // travel
                    State {
                        x: next_x,
                        y: next_y,
                        dir: self.dir.clone(),
                    }
                };
            Some(new_state)
        } else {
            None
        }
    }

    fn get_next_pos(state: &State, obstacles: &[Vec<bool>]) -> Option<(usize, usize)> {
        let next_pos = match state.dir {
            Direction::Up => {
                if state.y == 0 {
                    None
                } else {
                    Some((state.x, state.y - 1))
                }
            }
            Direction::Down => {
                if state.y == obstacles.len() - 1 {
                    None
                } else {
                    Some((state.x, state.y + 1))
                }
            }
            Direction::Left => {
                if state.x == 0 {
                    None
                } else {
                    Some((state.x - 1, state.y))
                }
            }
            Direction::Right => {
                if state.x == obstacles[0].len() - 1 {
                    None
                } else {
                    Some((state.x + 1, state.y))
                }
            }
        };
        next_pos
    }

}

fn parse_grid(input: &str) -> (Vec<Vec<bool>>, State) {
    let obstacles = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let pos = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '^' => Some(State {
                    x,
                    y,
                    dir: Direction::Up,
                }),
                'v' => Some(State {
                    x,
                    y,
                    dir: Direction::Down,
                }),
                '<' => Some(State {
                    x,
                    y,
                    dir: Direction::Left,
                }),
                '>' => Some(State {
                    x,
                    y,
                    dir: Direction::Right,
                }),
                _ => None,
            })
        })
        .filter_map(|x| x)
        .next();
    (obstacles, pos.unwrap())
}

fn solution_a(obstacles: &[Vec<bool>], initial_state: State) -> usize {
    let visited_positions = get_visited_positions(initial_state, obstacles);
    visited_positions.len()
}

fn get_visited_positions(initial_state: State, obstacles: &[Vec<bool>]) -> HashSet<(usize, usize)> {
    let mut state = initial_state;
    let mut visited_positions = HashSet::<(usize, usize)>::new();
    loop {
        visited_positions.insert((state.x, state.y));
        if let Some(new_state) = state.advance(obstacles, None) {
            state = new_state;
        } else {
            break;
        }
    }
    visited_positions
}

fn solution_b(obstacles: &[Vec<bool>], initial_state: State) -> usize {
    let mut loops = 0;
    let mut visited_positions = get_visited_positions(initial_state.clone(), obstacles);
    visited_positions.remove(&(initial_state.x, initial_state.y));
    for new_obstacle in visited_positions {
        if (detect_cycle(&initial_state, new_obstacle, obstacles)) {
            loops += 1;
        }
    }
    loops
}

fn detect_cycle(
    initial_state: &State,
    new_obstacle: (usize, usize),
    obstacles: &[Vec<bool>],
) -> bool {
    let mut slow_state = initial_state.clone();
    let mut fast_state = initial_state.clone();
    loop {
        if let Some(new_state) = slow_state.advance(obstacles, Some(new_obstacle)) {
            slow_state = new_state;
        } else {
            return false; // reached map end
        }

        if let Some(new_state) = fast_state.advance(obstacles, Some(new_obstacle)) {
            fast_state = new_state;
        } else {
            return false; // reached map end
        }

        if let Some(new_state) = fast_state.advance(obstacles, Some(new_obstacle)) {
            fast_state = new_state;
        } else {
            return false; // reached map end
        }
        if slow_state == fast_state {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (obstacles, initial_state) = parse_grid(TEST_INPUT);
        assert_eq!(solution_a(&obstacles, initial_state), 41);

        let (obstacles, initial_state) = parse_grid(INPUT);
        println!("{}", solution_a(&obstacles, initial_state));

        let (obstacles, initial_state) = parse_grid(TEST_INPUT);
        assert_eq!(solution_b(&obstacles, initial_state), 6);

        let (obstacles, initial_state) = parse_grid(INPUT);
        println!("{}", solution_b(&obstacles, initial_state));
    }

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const INPUT: &str = include_str!("input.txt");
}
