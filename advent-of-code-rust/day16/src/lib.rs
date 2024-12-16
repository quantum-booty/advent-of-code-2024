#![allow(unused)]
#![feature(let_chains)]
use Direction::*;
use Object::*;
use core::panic;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn parse(input: &str) -> (Vec<Vec<Object>>, State) {
    let grid: Vec<Vec<Object>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' | 'S' => Empty,
                    'E' => Objective,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let initial_state = State {
        x: 1,
        y: grid.len() - 2,
        direction: Right,
    };
    (grid, initial_state)
}

#[derive(Debug, Clone, Copy)]
enum Object {
    Empty,
    Wall,
    Objective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl State {
    fn rotate_clockwise(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            direction: match self.direction {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            },
        }
    }

    fn rotate_counter_clockwise(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            direction: match self.direction {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            },
        }
    }

    fn advance(&self) -> State {
        State {
            x: match self.direction {
                Up => self.x,
                Down => self.x,
                Left => self.x.wrapping_sub(1),
                Right => self.x + 1,
            },
            y: match self.direction {
                Up => self.y.wrapping_sub(1),
                Down => self.y + 1,
                Left => self.y,
                Right => self.y,
            },
            direction: self.direction,
        }
    }

    fn try_advance(&self, grid: &[Vec<Object>]) -> Option<State> {
        let new_state = self.advance();
        if new_state.x < grid[0].len()
            && new_state.y < grid.len()
            && match grid[new_state.y][new_state.x] {
                Empty | Objective => true,
                Wall => false,
            }
        {
            return Some(new_state);
        }
        None
    }
}

fn get_neighbours_with_cost(
    state: State,
    grid: &[Vec<Object>],
) -> impl Iterator<Item = (State, u32)> {
    let possible_states = [
        state.try_advance(grid).map(|new_state| (new_state, 1)),
        Some((state.rotate_clockwise(), 1000)),
        Some((state.rotate_counter_clockwise(), 1000)),
    ];

    possible_states.into_iter().filter_map(|x| x)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeCost {
    state: State,
    cost: u32,
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost)) // min heap ordering
    }
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // min heap ordering
    }
}

fn dijkstra(
    grid: &[Vec<Object>],
    initial_state: State,
) -> (HashMap<State, u32>, HashMap<State, HashSet<State>>) {
    let mut costs = HashMap::<State, u32>::new();
    let mut min_cost_queue = BinaryHeap::<NodeCost>::from([NodeCost {
        state: initial_state,
        cost: 0,
    }]);
    let mut precedents = HashMap::<State, HashSet<State>>::new();

    while let Some(node_cost) = min_cost_queue.pop() {
        let NodeCost {
            state,
            cost: curr_cost,
        } = node_cost;

        // println!();
        // println!("{}", curr_cost);
        // pretty_print(grid, state);

        if let Objective = grid[state.y][state.x] {
            return (costs, precedents);
        }

        for (neighbour, cost) in get_neighbours_with_cost(state, grid) {
            let cost_to_neighbour = curr_cost + cost;
            let neighbour_old_cost = costs.entry(neighbour).or_insert(u32::MAX);
            if cost_to_neighbour < *neighbour_old_cost {
                *neighbour_old_cost = cost_to_neighbour;
                min_cost_queue.push(NodeCost {
                    state: neighbour,
                    cost: cost_to_neighbour,
                });
                precedents
                    .entry(neighbour)
                    .and_modify(|x| *x = HashSet::from([state])) // found cheaper path, clear existing path
                    .or_insert(HashSet::from([state]));
            } else if cost_to_neighbour == *neighbour_old_cost {
                precedents
                    .entry(neighbour)
                    .and_modify(|x| _ = x.insert(state)) // same cost, add to path
                    .or_insert(HashSet::from([state]));
            }
        }
    }
    panic!()
}

fn solution(grid: &[Vec<Object>], initial_state: State) -> (u32, usize) {
    let (costs, precedents) = dijkstra(grid, initial_state);
    let end_node_costs = costs
        .iter()
        .filter_map(|(state, cost)| {
            if let Objective = grid[state.y][state.x] {
                Some((state, cost))
            } else {
                None
            }
        })
        .collect_vec();
    let min_cost = end_node_costs.iter().map(|&(_, &c)| c).min().unwrap();
    let min_cost_end_states = end_node_costs
        .into_iter()
        .filter_map(|(&state, &cost)| if cost == min_cost { Some(state) } else { None })
        .collect_vec();

    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    for node in min_cost_end_states {
        stack.push(node);
        seen.insert(node);
    }
    while let Some(node) = stack.pop() {
        if let Some(precedent_nodes) = precedents.get(&node) {
            for precedent in precedent_nodes {
                if !seen.contains(precedent) {
                    seen.insert(*precedent);
                    stack.push(*precedent);
                }
            }
        }
    }
    (
        min_cost,
        seen.iter().map(|state| (state.x, state.y)).unique().count(),
    )
}

fn pretty_print(grid: &[Vec<Object>], state: State) {
    let pretty_str = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, object)| {
                    if state.x == x && state.y == y {
                        match state.direction {
                            Up => '^',
                            Down => 'v',
                            Left => '<',
                            Right => '>',
                        }
                    } else {
                        match object {
                            Empty => '.',
                            Wall => '#',
                            Objective => 'E',
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
        let (grid, initial_state) = parse(TEST_INPUT_A1);
        assert_eq!(solution(&grid, initial_state).0, 1002);
        let (grid, initial_state) = parse(TEST_INPUT_A2);
        assert_eq!(solution(&grid, initial_state), (7036, 45));
        let (grid, initial_state) = parse(TEST_INPUT_A3);
        assert_eq!(solution(&grid, initial_state), (11048, 64));
        let (grid, initial_state) = parse(TEST_INPUT_A4);
        assert_eq!(solution(&grid, initial_state), (9029, 62));
        let (grid, initial_state) = parse(INPUT);
        println!("{:?}", solution(&grid, initial_state));
    }

    const TEST_INPUT_A1: &str = "####
#.E#
#S.#
####";

    const TEST_INPUT_A2: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_INPUT_A3: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    const TEST_INPUT_A4: &str = "################
####.........#E#
###..#######.#.#
###.##...###.#.#
###.##.#.###.#.#
#......#.#.....#
#.#.####.#.#.###
#.#.####...#.###
#.#..#######.###
#S##.........###
################";

    const INPUT: &str = include_str!("input.txt");
}
