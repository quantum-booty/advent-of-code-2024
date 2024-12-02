#![allow(unused)]
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let report = line.split(" ").map(|c| c.parse().unwrap()).collect_vec();
        vec.push(report);
    }
    vec
}

fn test_report_a<'a>(
    report: impl Iterator<Item = &'a i32>,
    condition_func: fn(i32, i32) -> bool,
) -> bool {
    report
        .tuple_windows()
        .all(|(curr, next)| condition_func(*curr, *next))
}

fn is_desc(curr: i32, next: i32) -> bool {
    curr > next && (curr - next) <= 3
}

fn is_asc(curr: i32, next: i32) -> bool {
    curr < next && (next - curr) <= 3
}

fn test_report_b(report: &[i32], condition_func: fn(i32, i32) -> bool) -> bool {
    test_report_a(report.iter(), condition_func)
        || (0..report.len()).any(|i| {
            let (left, right) = report.split_at(i);
            test_report_a(left.iter().chain(right.iter().skip(1)), condition_func)
        })
}

fn solution_a(vec: &[Vec<i32>]) -> usize {
    vec.iter()
        .filter(|&report| {
            test_report_a(report.iter(), is_desc) || test_report_a(report.iter(), is_asc)
        })
        .count()
}

fn solution_b(vec: &[Vec<i32>]) -> usize {
    vec.iter()
        .filter(|&report| test_report_b(report, is_asc) || test_report_b(report, is_desc))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = parse(TEST_INPUT);

        assert_eq!(solution_a(&vec), 2);
        assert_eq!(solution_b(&vec), 4);

        let vec = parse(INPUT);
        println!("{}", solution_a(&vec));
        println!("{}", solution_b(&vec));
    }

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const INPUT: &str = include_str!("input.txt");
}
