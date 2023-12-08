#![feature(test)]
extern crate test;

use std::{collections::HashMap, fs::read_to_string, str::Lines};

use num::Integer;

pub fn solve(mut lines: Lines) -> i64 {
    let input = lines.next().unwrap().chars().cycle();
    let nodes = lines
        .skip(1)
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect::<HashMap<_, _>>();

    nodes
        .iter()
        .filter_map(|(node, _)| node.ends_with('A').then_some(node))
        .map(|mut node| {
            let mut steps = 0;
            let mut input = input.clone();

            while !node.ends_with('Z') {
                let input = input.next().unwrap();

                if input == 'L' {
                    node = &nodes[*node].0;
                } else {
                    node = &nodes[*node].1;
                }

                steps += 1;
            }

            steps
        })
        .fold(0, |current_lcm, value| {
            if current_lcm == 0 {
                value
            } else {
                current_lcm.lcm(&value)
            }
        })
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day8.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day8.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
