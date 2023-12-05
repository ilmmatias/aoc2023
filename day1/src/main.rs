#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    lines
        .map(|line| {
            let pat = |(i, (pat, value))| line[i..].starts_with(pat).then_some(value);

            let left = (0..line.len())
                .cartesian_product(digits)
                .find_map(pat)
                .unwrap();

            let right = (0..line.len())
                .rev()
                .cartesian_product(digits)
                .find_map(pat)
                .unwrap();

            left * 10 + right
        })
        .sum()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day1.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day1.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
