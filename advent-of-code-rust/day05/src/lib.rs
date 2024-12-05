#![allow(unused)]
use core::panic;
use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        newline,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)
}

fn to_rule_map(rules: Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();
    for (parent, child) in rules {
        map.entry(child).or_insert(vec![]).push(parent);
    }
    map
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, separated_list1(tag(","), complete::u32))(input)
}

fn parse(rules_str: &str, updates_str: &str) -> (Vec<Vec<u32>>, HashMap<u32, Vec<u32>>) {
    let rules = to_rule_map(parse_rules(rules_str).unwrap().1);
    let updates = parse_updates(updates_str).unwrap().1;
    (updates, rules)
}
fn validate_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    let mut active_rules = HashSet::<u32>::new();
    for number in update.iter().rev() {
        if !active_rules.is_empty() {
            if active_rules.contains(number) {
                active_rules.remove(number);
            } else {
                return None;
            }
        }
        if let Some(new_rules) = rules.get(number) {
            let relevant_rules = new_rules.iter().filter(|x| update.contains(x));
            for rule in relevant_rules {
                active_rules.insert(*rule);
            }
        }
    }
    if active_rules.is_empty() {
        Some(update[update.len() / 2])
    } else {
        None
    }
}

fn solution_a(updates: Vec<Vec<u32>>, rules: HashMap<u32, Vec<u32>>) -> u32 {
    let mut res = 0;
    for update in updates {
        if let Some(mid) = validate_update(&update, &rules) {
            res += mid;
        }
    }
    res
}

fn create_valid_path(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut parent_to_child = HashMap::<u32, Vec<u32>>::new();
    for number in update.iter() {
        if let Some(parents) = rules.get(&number) {
            for parent in parents {
                if update.contains(parent) {
                    parent_to_child.entry(*parent).or_insert(vec![]).push(*number);
                }
            }
        }
    }
    let mut child_to_parent = HashMap::new();
    for (parent, children) in &parent_to_child {
        for child in children {
            child_to_parent.entry(child).or_insert(vec![]).push(parent);
        }
    }
    // root has no parent
    let root = update.iter().filter(|x| !child_to_parent.contains_key(x)).next().unwrap();

    let mut stack: Vec<(u32, Vec<u32>)> = vec![(*root, vec![*root])];
    while !stack.is_empty() {
        let (parent, path) = stack.pop().unwrap();
        if path.len() == update.len() {
            return path;
        }
        if let Some(children) = &parent_to_child.get(&parent) {
            for child in *children {
                let mut new_path = path.clone();
                new_path.push(*child);
                stack.push((*child, new_path));
            }
        }
    }
    panic!("never should happen!")
}

fn solution_b(updates: Vec<Vec<u32>>, rules: HashMap<u32, Vec<u32>>) -> u32 {
    let mut res = 0;
    for update in updates {
        if validate_update(&update, &rules).is_none() {
            let new_update = create_valid_path(&update, &rules);
            if let Some(mid) = validate_update(&new_update, &rules) {
                res += mid;
            } else {
                panic!("never should happen!");
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (updates, rules) = parse(TEST_RULES, TEST_UPDATES);
        assert_eq!(solution_a(updates, rules), 143);

        let (updates, rules) = parse(
            RULES.replace("\r\n", "\n").as_str(),
            UPDATES.replace("\r\n", "\n").as_str(),
        );
        println!("{}", solution_a(updates, rules));

        let (updates, rules) = parse(TEST_RULES, TEST_UPDATES);
        assert_eq!(solution_b(updates, rules), 123);
        let (updates, rules) = parse(
            RULES.replace("\r\n", "\n").as_str(),
            UPDATES.replace("\r\n", "\n").as_str(),
        );

        let now = Instant::now();
        println!("{}", solution_b(updates, rules));
        println!("{:.2?}", now.elapsed());
    }

    const TEST_UPDATES: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    const TEST_RULES: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    const RULES: &str = include_str!("rules.txt");
    const UPDATES: &str = include_str!("updates.txt");
}
