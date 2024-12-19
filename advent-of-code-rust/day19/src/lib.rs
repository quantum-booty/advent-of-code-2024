#![allow(unused)]
#![feature(let_chains)]
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let input = input.replace("\r\n", "\n");
    let (patterns_str, goals_str) = input.split_once("\n\n").unwrap();
    let patterns = patterns_str
        .split(",")
        .map(|s| s.replace(" ", ""))
        .collect();
    let goals = goals_str
        .lines()
        .map(|s| String::from_str(s).unwrap())
        .collect();
    (patterns, goals)
}

fn try_get_next_pos(pattern: &String, goal: &String, pos: usize) -> Option<usize> {
    if pos + pattern.len() > goal.len() {
        return None;
    }
    let substr = &goal[pos..pos + pattern.len()];
    if substr == pattern {
        return Some(pos + pattern.len());
    }
    None
}

fn check_can_reach_goal(patterns: &[String], goal: &String) -> bool {
    // dfs
    let mut stack = Vec::new();
    stack.push(0);
    let mut seen = HashSet::new();
    while let Some(pos) = stack.pop() {
        if pos == goal.len() {
            return true;
        }
        for pattern in patterns {
            if let Some(new_pos) = try_get_next_pos(pattern, goal, pos) {
                if !seen.insert(new_pos) {
                    continue;
                }
                stack.push(new_pos);
            }
        }
    }

    false
}

fn solution_a(patterns: &[String], goals: &[String]) -> usize {
    goals
        .iter()
        .filter(|goal| check_can_reach_goal(patterns, goal))
        .count()
}

fn topological_sort(patterns: &[String], goal: &String) -> Vec<usize> {
    let mut in_degrees = calc_in_degrees(patterns, goal);
    let mut topo_path = vec![0];
    let mut queue = VecDeque::from([0]);
    while let Some(pos) = queue.pop_front() {
        if pos == goal.len() {
            break;
        }
        for pattern in patterns {
            if let Some(new_pos) = try_get_next_pos(pattern, goal, pos) {
                let in_degree = in_degrees.get_mut(&new_pos).unwrap();
                *in_degree -= 1;
                if *in_degree == 0 {
                    queue.push_back(new_pos);
                    topo_path.push(new_pos);
                }
            }
        }
    }

    topo_path
}

fn calc_in_degrees(patterns: &[String], goal: &String) -> HashMap<usize, i32> {
    let mut in_degrees = HashMap::new();
    let mut stack = vec![0];
    let mut seen = HashSet::new();
    while let Some(pos) = stack.pop() {
        if pos == goal.len() {
            continue;
        }
        for pattern in patterns {
            if let Some(new_pos) = try_get_next_pos(pattern, goal, pos) {
                in_degrees
                    .entry(new_pos)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                if !seen.insert(new_pos) {
                    continue;
                }
                stack.push(new_pos);
            }
        }
    }
    in_degrees
}

fn count_paths(patterns: &[String], goal: &String, topo_path: Vec<usize>) -> usize {
    let mut counts = HashMap::new();
    counts.insert(goal.len(), 1);
    for &pos in topo_path.iter().rev() {
        for pattern in patterns {
            if let Some(new_pos) = try_get_next_pos(pattern, goal, pos) {
                let neighbour_count = *counts.entry(new_pos).or_insert(0);
                let new_count = counts
                    .entry(pos)
                    .and_modify(|x| *x += neighbour_count)
                    .or_insert(neighbour_count);
            }
        }
    }
    counts[&0]
}

fn solution_b(patterns: &[String], goals: &[String]) -> usize {
    goals
        .iter()
        .filter_map(|goal| {
            if !check_can_reach_goal(patterns, goal) {
                return None;
            }
            let topo = topological_sort(patterns, goal);
            Some(count_paths(patterns, goal, topo))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (patterns, goals) = parse(TEST_INPUT_A1);
        assert_eq!(solution_a(&patterns, &goals), 6);
        let (patterns, goals) = parse(INPUT);
        println!("{:?}", solution_a(&patterns, &goals));

        let (patterns, goals) = parse(TEST_INPUT_A1);
        assert_eq!(solution_b(&patterns, &goals), 16);
        let (patterns, goals) = parse(INPUT);
        println!("{:?}", solution_b(&patterns, &goals));
    }

    const TEST_INPUT_A1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    const INPUT: &str = include_str!("input.txt");
}
