#![allow(unused)]
use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, char},
    combinator::map,
    sequence::{preceded, tuple},
};
use regex::Regex;
use std::fmt::{self};

#[derive(fmt::Debug)]
struct Pair {
    a: i32,
    b: i32,
}

#[derive(fmt::Debug)]
enum Operation {
    Multiply(Pair),
    Dont,
    Do,
}

fn parse(input: &str) -> Vec<Pair> {
    let mut vec: Vec<Pair> = Vec::new();
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    re.captures_iter(input)
        .map(|x| {
            let (_, [a, b]) = x.extract();
            Pair {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            }
        })
        .collect()
}

fn parse_b(input: &str) -> Vec<Operation> {
    let mut vec: Vec<Pair> = Vec::new();
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d*,\d*\)").unwrap();
    re.find_iter(input)
        .map(|x| {
            let text = x.as_str();
            if text.starts_with("don't") {
                Operation::Dont
            } else if text.starts_with("do") {
                Operation::Do
            } else {
                let pair = parse_pair(text);
                Operation::Multiply(pair.unwrap().1)
            }
        })
        .collect()
}

fn parse_pair(text: &str) -> IResult<&str, Pair> {
    let pair = map(
        preceded(
            tag("mul"),
            tuple((
                char('('),
                complete::i32,
                char(','),
                complete::i32,
                char(')'),
            )),
        ),
        |x| Pair { a: x.1, b: x.3 },
    )(text);
    pair
}

fn solution_a(vec: &[Pair]) -> i32 {
    vec.iter().map(|p| p.a * p.b).sum()
}

fn solution_b(vec: &[Operation]) -> i32 {
    let mut condition = true;
    let mut res = 0;
    for operation in vec {
        match operation {
            Operation::Multiply(pair) => res += if condition { pair.a * pair.b } else { 0 },
            Operation::Dont => condition = false,
            Operation::Do => condition = true,
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = parse(TEST_INPUT);
        println!("{} {:?}", vec.len(), vec);
        assert_eq!(solution_a(&vec), 161);
        let vec = parse(INPUT);
        println!("{}", solution_a(&vec));

        let vec = parse_b(TEST_INPUT_B);
        println!("{} {:?}", vec.len(), vec);
        assert_eq!(solution_b(&vec), 48);
        let vec = parse_b(INPUT);
        println!("{}", solution_b(&vec));
    }

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const INPUT: &str = include_str!("input.txt");
}
