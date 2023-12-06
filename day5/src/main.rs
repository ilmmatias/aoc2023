#![feature(test)]
extern crate test;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{collections::HashMap, fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    let mut lines = lines.peekable();
    while lines.peek().unwrap().is_empty() {
        _ = lines.next();
    }

    let seeds = lines.next().unwrap()[7..]
        .split(' ')
        .map(|seed| seed.trim())
        .filter(|seed| !seed.is_empty())
        .map(|seed| seed.parse::<i64>().unwrap())
        .tuples()
        .collect::<Vec<(_, _)>>();

    let mut maps = Vec::new();
    for _ in 0..7 {
        while lines.peek().unwrap().is_empty() {
            _ = lines.next();
        }

        _ = lines.next();
        let mut map = HashMap::new();

        while lines.peek().is_some() && !lines.peek().unwrap().is_empty() {
            let (dest_range_start, source_range_start, range_len) = lines
                .next()
                .unwrap()
                .split(' ')
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.parse().unwrap())
                .collect_tuple()
                .unwrap();

            map.insert(
                (source_range_start, source_range_start + range_len),
                dest_range_start,
            );
        }

        maps.push(map)
    }

    let num_threads = num_cpus::get() * 2;
    (0..num_threads)
        .cartesian_product(seeds.iter())
        .map(|(thread, (range_start, range_len))| {
            let per_thread_range_len = range_len / num_threads as i64;
            let this_thread_range_start = *range_start + per_thread_range_len * thread as i64;
            let this_thread_range_len =
                per_thread_range_len.min(range_len - per_thread_range_len * thread as i64);
            this_thread_range_start..this_thread_range_start + this_thread_range_len
        })
        .par_bridge()
        .map(|range| {
            println!("checking range {:?}", range);
            let mut smallest = i64::MAX;

            for mut entry in range {
                for map in &maps {
                    entry = map
                        .iter()
                        .find_map(
                            |((source_range_start, source_range_end), dest_range_start)| {
                                if entry >= *source_range_start && entry < *source_range_end {
                                    Some(entry - source_range_start + dest_range_start)
                                } else {
                                    None
                                }
                            },
                        )
                        .unwrap_or(entry)
                }

                smallest = smallest.min(entry);
            }

            smallest
        })
        .min()
        .unwrap()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day5.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day5.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
