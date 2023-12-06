#![feature(test)]
extern crate test;

use std::{collections::HashMap, fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    lines
        .map(|line| {
            line[5..]
                .split_once(|ch: char| !ch.is_digit(10))
                .unwrap()
                .1
                .split(';')
                .map(|game| game.split(','))
                .map(|game| game.map(|part| part.trim()))
                .map(|game| game.map(|part| part.split_once(' ').unwrap()))
                .map(|game| game.map(|(count, color)| (count.parse().unwrap(), color)))
                .fold(HashMap::new(), |mut map, game| {
                    game.for_each(|(count, color)| {
                        map.entry(color)
                            .and_modify(|parts: &mut Vec<i64>| parts.push(count))
                            .or_insert(vec![count]);
                    });

                    map
                })
                .iter()
                .map(|(_, value)| value.iter().max().unwrap())
                .product::<i64>()
        })
        .sum()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day2.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day2.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
