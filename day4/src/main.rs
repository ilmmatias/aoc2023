#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    lines
        .enumerate()
        .fold(HashMap::new(), |mut cards, (i, line)| {
            let (winning_numbers, card_numbers) = &line[8..].split_once('|').unwrap();

            let card_numbers = card_numbers
                .split(' ')
                .map(|card| card.trim())
                .filter(|card| !card.is_empty())
                .collect_vec();

            let entries = *cards.entry(i).and_modify(|entry| *entry += 1).or_insert(1);

            for j in 0..winning_numbers
                .split(' ')
                .map(|card| card.trim())
                .filter(|card| !card.is_empty() && card_numbers.contains(card))
                .count()
                .try_into()
                .unwrap_or_default()
            {
                cards
                    .entry(i + j + 1)
                    .and_modify(|entry| *entry += entries)
                    .or_insert(entries);
            }

            cards
        })
        .iter()
        .map(|(_, card)| card)
        .sum()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day4.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day4.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
