#![feature(test)]
extern crate test;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{collections::HashMap, fs::read_to_string, iter::Peekable, str::Lines};

fn build_map(lines: &mut Peekable<Lines>) -> HashMap<(i64, i64), i64> {
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

    map
}

fn translate(map: &HashMap<(i64, i64), i64>, entry: i64) -> i64 {
    map.iter()
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

    let soil_map = build_map(&mut lines);
    let fertilizer_map = build_map(&mut lines);
    let water_map = build_map(&mut lines);
    let light_map = build_map(&mut lines);
    let temperature_map = build_map(&mut lines);
    let humidity_map = build_map(&mut lines);
    let location_map = build_map(&mut lines);

    let num_threads = num_cpus::get();
    let threads = (0..num_threads)
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
                entry = translate(&soil_map, entry);
                entry = translate(&fertilizer_map, entry);
                entry = translate(&water_map, entry);
                entry = translate(&light_map, entry);
                entry = translate(&temperature_map, entry);
                entry = translate(&humidity_map, entry);
                smallest = smallest.min(translate(&location_map, entry));
            }

            smallest
        });

    threads.min().unwrap()
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
