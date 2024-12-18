#![allow(unused)]
#![feature(let_chains)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn parse(input: &str) -> (Computer, Vec<u64>) {
    let input = input.replace("\r\n", "\n");
    let (computer_str, program_str) = input.split_once("\n\n").unwrap();
    let regex = Regex::new(r"\d+").unwrap();
    let mut registers = regex.find_iter(computer_str);
    let computer = Computer {
        a: registers.next().unwrap().as_str().parse().unwrap(),
        b: registers.next().unwrap().as_str().parse().unwrap(),
        c: registers.next().unwrap().as_str().parse().unwrap(),
    };
    let program = regex
        .find_iter(program_str)
        .map(|x| x.as_str().parse().unwrap())
        .collect();
    (computer, program)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
}

fn get_combo_operand(computer: &Computer, operand: u64) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => panic!(),
    }
}

fn solution(mut computer: Computer, program: Vec<u64>) -> String {
    let mut inst_ptr = 0;
    let mut outs = vec![];
    loop {
        if inst_ptr >= program.len() {
            break;
        }
        let opcode = program[inst_ptr];
        let operand = program[inst_ptr + 1];
        match opcode {
            0 => computer.a = adv(computer, operand),
            1 => computer.b = computer.b ^ operand,
            2 => computer.b = get_combo_operand(&computer, operand) % 8,
            3 => {
                if computer.a != 0 {
                    inst_ptr = operand as usize;
                    continue;
                }
            }
            4 => computer.b = computer.b ^ computer.c,
            5 => outs.push(get_combo_operand(&computer, operand) % 8),
            6 => computer.b = adv(computer, operand),
            7 => computer.c = adv(computer, operand),
            _ => panic!(),
        }
        inst_ptr = inst_ptr + 2;
    }

    outs.into_iter().join(",")
}

fn solution_brute_force(mut initial_computer: Computer, program: Vec<u64>) -> u64 {
    let mut outs = vec![];
    let mut guess = 1;
    loop {
        if guess % 10000000 == 0 {
            println!("{}", guess);
        }
        let mut inst_ptr = 0;
        let mut computer = Computer {
            a: guess,
            b: initial_computer.a,
            c: initial_computer.b,
        };
        let mut failed = false;
        loop {
            if inst_ptr >= program.len() {
                break;
            }
            let opcode = program[inst_ptr];
            let operand = program[inst_ptr + 1];
            match opcode {
                0 => computer.a = adv(computer, operand),
                1 => computer.b = computer.b ^ operand,
                2 => computer.b = get_combo_operand(&computer, operand) % 8,
                3 => {
                    if computer.a != 0 {
                        inst_ptr = operand as usize;
                        continue;
                    }
                }
                4 => computer.b = computer.b ^ computer.c,
                5 => {
                    let out = get_combo_operand(&computer, operand) % 8;
                    outs.push(out);
                    if outs.len() > program.len() || out != program[outs.len() - 1] {
                        outs.clear();
                        failed = true;
                        break;
                    }
                }
                6 => computer.b = adv(computer, operand),
                7 => computer.c = adv(computer, operand),
                _ => panic!(),
            }
            inst_ptr = inst_ptr + 2;
        }

        if !failed && outs.len() == program.len() {
            return guess;
        }
        outs.clear();
        guess += 1;
    }
    panic!()
}

fn adv(computer: Computer, operand: u64) -> u64 {
    let numerator = computer.a;
    let combo_operand = get_combo_operand(&computer, operand);
    let denominator = 2_u64.pow(combo_operand as u32);
    let result = numerator / denominator;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (computer, program) = parse(TEST_INPUT_A1);
        assert_eq!(
            solution(computer, program),
            String::from("4,6,3,5,6,3,5,2,1,0")
        );
        let (computer, program) = parse(INPUT);
        println!("{:?}", solution(computer, program));

        let (computer, program) = parse(TEST_INPUT_B1);
        assert_eq!(solution_brute_force(computer, program), 117440);
        let (computer, program) = parse(INPUT);
        println!("{:?}", solution_brute_force(computer, program));

        // for i in 0..7 {
        //     println!(
        //         "{}",
        //         (((((i * 8) * 8 + 3) * 8 + 4) * 8 + 5) * 8 + 3) * 8 + 0
        //     );
        // }
    }

    const TEST_INPUT_A1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_INPUT_B1: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    const INPUT: &str = include_str!("input.txt");
}
