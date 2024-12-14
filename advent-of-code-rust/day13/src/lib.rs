#![allow(unused)]
#![feature(let_chains)]
use faer::{Parallelism, linalg::matmul::matmul, mat::*, prelude::*};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let regex = Regex::new(r"\d+").unwrap();
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| {
            let numbers = regex
                .find_iter(lines)
                .filter_map(|digits| digits.as_str().parse::<f64>().ok())
                .collect_vec();
            ClawMachine {
                button_a: Vector {
                    x: numbers[0],
                    y: numbers[1],
                },
                button_b: Vector {
                    x: numbers[2],
                    y: numbers[3],
                },
                prize: Vector {
                    x: numbers[4],
                    y: numbers[5],
                },
            }
        })
        .collect()
}

fn solution(inputs: &[ClawMachine], prize_vector_offset: f64) -> f64 {
    let mut res = 0.0;
    let costs = mat![[3.0, 1.0]];
    for claw_machine in inputs {
        let buttons = mat![[claw_machine.button_a.x, claw_machine.button_b.x], [
            claw_machine.button_a.y,
            claw_machine.button_b.y
        ]];
        let prize = mat![[claw_machine.prize.x + prize_vector_offset], [claw_machine
            .prize
            .y
            + prize_vector_offset]];
        let lu = buttons.full_piv_lu();
        let moves = lu.solve(&prize);

        if !(is_whole_number(moves[(0, 0)]) && is_whole_number(moves[(1, 0)])) {
            continue;
        }

        let mut result = Mat::zeros(1, 1); // Allocate space for the single float result
        matmul(
            result.as_mut(), // Storage for result
            costs.as_ref(),  // Left matrix (1x2)
            moves.as_ref(),  // Right matrix (2x1)
            None,            // No addition to the result matrix
            1.0,             // Scaling factor for the product
            Parallelism::None,
        );
        let cost = result[(0, 0)];
        res += cost.round();
    }
    res
}

fn is_whole_number(number: f64) -> bool {
    (number - number.round()).abs() < 0.001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let claw_machines = parse(TEST_INPUT_A1);
        assert_eq!(solution(&claw_machines, 0.0), 480.0);
        let claw_machines = parse(INPUT);
        println!("{}", solution(&claw_machines, 0.0));

        let claw_machines = parse(INPUT);
        println!("{}", solution(&claw_machines, 10000000000000.0));
    }

    const TEST_INPUT_A1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    const INPUT: &str = include_str!("input.txt");
}
