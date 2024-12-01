#![allow(unused)]

use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec_1: Vec<i32> = Vec::new();
    let mut vec_2: Vec<i32> = Vec::new();
    for line in input.lines()
    {
        let (a, b) = line.split_once("   ").unwrap();
        vec_1.push(a.parse().unwrap());
        vec_2.push(b.parse().unwrap())
    }
    vec_1.sort();
    vec_2.sort();
    (vec_1, vec_2)
}

fn solution_a(vec_1: &[i32], vec_2: &[i32]) -> i32 {
    vec_1.iter().zip(vec_2).map(|(a, b)| (*a - b).abs()).sum()
}

fn solution_b(vec_1: &[i32], vec_2: &[i32]) -> i32 {
    let mut vec_2_counter: HashMap<i32, i32> = HashMap::new();
    for x in vec_2 {
        vec_2_counter.entry(*x).and_modify(|count| *count+=1).or_insert(1);
    }
    let mut result = 0;
    for x in vec_1 {
        if let Some(count) = vec_2_counter.get(x) {
            result += x * count;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (vec_1, vec_2) = parse(TEST_INPUT);
        
        assert_eq!(solution_a(&vec_1, &vec_2), 11);
        assert_eq!(solution_b(&vec_1, &vec_2), 31);

        let (vec_3, vec_4) = parse(INPUT);
        println!("{}", solution_a(&vec_3, &vec_4));
        println!("{}", solution_b(&vec_3, &vec_4));
    }

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    const INPUT: &str = include_str!("input.txt");
}

