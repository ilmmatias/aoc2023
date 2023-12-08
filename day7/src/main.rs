#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    let strength: HashMap<char, i64> = [
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]
    .into();

    let ranked = lines
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            let mut cards = hand.chars().fold(HashMap::new(), |mut map, card| {
                map.entry(card).and_modify(|entry| *entry += 1).or_insert(1);
                map
            });

            if cards.len() != 1 {
                if let Some(joker_count) = cards.remove(&'J') {
                    let i = *cards
                        .iter()
                        .sorted_unstable_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count))
                        .next()
                        .unwrap()
                        .0;
                    *cards.get_mut(&i).unwrap() += joker_count;
                }
            }

            (
                cards
                    .iter()
                    .map(|(_, count)| match count {
                        5 => 6,
                        4 => 5,
                        3 => {
                            if cards.len() == 2 {
                                4
                            } else {
                                3
                            }
                        }
                        2 => {
                            if cards.len() == 3 {
                                2
                            } else {
                                1
                            }
                        }
                        _ => 0,
                    })
                    .sorted_unstable_by(|a_count, b_count| b_count.cmp(a_count))
                    .next()
                    .unwrap(),
                hand,
                bid.parse().unwrap(),
            )
        })
        .sorted_unstable_by(|(a_rank, a_hand, _), (b_rank, b_hand, _)| {
            let order = b_rank.cmp(a_rank);

            if order == Ordering::Equal {
                for (a_card, b_card) in a_hand.chars().zip(b_hand.chars()) {
                    if strength[&b_card] < strength[&a_card] {
                        return Ordering::Less;
                    } else if strength[&b_card] > strength[&a_card] {
                        return Ordering::Greater;
                    }
                }
            }

            order
        })
        .collect_vec();

    ranked
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (ranked.len() - i) as i64)
        .sum()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day7.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day7.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
