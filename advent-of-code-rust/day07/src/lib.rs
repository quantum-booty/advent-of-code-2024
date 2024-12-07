#![allow(unused)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (answer_str, inputs_str) = line.split_once(": ").unwrap();
            let answer = answer_str.parse().unwrap();
            let inputs = inputs_str.split(' ').map(|x| x.parse().unwrap()).collect();
            (answer, inputs)
        })
        .collect()
}

fn try_minus(answer: i64, number: i64) -> Option<i64> {
    let minus_answer = answer - number;
    let valid_minus = minus_answer > 0;
    if valid_minus {
        Some(minus_answer)
    } else {
        None
    }
}

fn try_divide(answer: i64, number: i64) -> Option<i64> {
    let divide_answer = answer / number;
    let valid_divide = answer % number == 0;
    if valid_divide {
        Some(divide_answer)
    } else {
        None
    }
}

fn try_split_tail(full: i64, tail: i64) -> Option<i64> {
    // Calculate the length (number of digits) of the tail
    let mut divisor = 1;
    let mut temp = tail;
    while temp > 0 {
        divisor *= 10;
        temp /= 10;
    }

    // Check if the tail matches the last digits of the full number
    if full % divisor == tail {
        let head = full / divisor;
        Some(head)
    } else {
        None
    }
}

fn is_valid(answer: i64, inputs: &[i64], i: usize) -> bool {
    let minus_answer = try_minus(answer, inputs[i]);
    let divide_answer = try_divide(answer, inputs[i]);

    if i == 0 {
        return minus_answer.is_some_and(|x| x == 0) || divide_answer.is_some_and(|x| x == 1);
    }
    minus_answer.is_some_and(|x| is_valid(x, inputs, i - 1))
        || divide_answer.is_some_and(|x| is_valid(x, inputs, i - 1))
}

fn solution_a(equations: Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .iter()
        .filter(|(answer, inputs)| is_valid(*answer, inputs, inputs.len() - 1))
        .map(|(answer, _)| answer)
        .sum()
}

fn is_valid_part_b(answer: i64, inputs: &[i64], i: usize) -> bool {
    let minus_answer = try_minus(answer, inputs[i]);
    let divide_answer = try_divide(answer, inputs[i]);
    let split_answer = try_split_tail(answer, inputs[i]);

    if i == 0 {
        return minus_answer.is_some_and(|x| x == 0)
            || divide_answer.is_some_and(|x| x == 1)
            || split_answer.is_some_and(|head| head == 0);
    }

    minus_answer.is_some_and(|x| is_valid_part_b(x, inputs, i - 1))
        || divide_answer.is_some_and(|x| is_valid_part_b(x, inputs, i - 1))
        || split_answer.is_some_and(|head| is_valid_part_b(head, inputs, i - 1))
}

fn solution_b(equations: Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .iter()
        .filter(|(answer, inputs)| is_valid_part_b(*answer, inputs, inputs.len() - 1))
        .map(|(answer, _)| answer)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let equations = parse_equations(TEST_INPUT);
        assert_eq!(solution_a(equations), 3749);

        let equations = parse_equations(INPUT);
        println!("{}", solution_a(equations));

        let equations = parse_equations(TEST_INPUT);
        assert_eq!(solution_b(equations), 11387);

        let equations = parse_equations(INPUT);
        println!("{}", solution_b(equations));
    }

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    const INPUT: &str = include_str!("input.txt");
}
