#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{fs::read_to_string, str::Lines};

fn translate(map: &Vec<((i64, i64), i64)>, range: &(i64, i64)) -> Vec<(i64, i64)> {
    let mut position = range.0;
    let mut ranges = Vec::new();

    while position < range.0 + range.1 {
        let remaining_length = range.0 + range.1 - position;
        let mapping = map
            .iter()
            .find(|((source_range_start, source_range_len), _)| {
                *source_range_start <= position && source_range_start + source_range_len > position
            });

        // Intersection between ranges, use everything we can from the match.
        if let Some(((source_range_start, source_range_len), dest_range_start)) = mapping {
            let dest_range_len =
                remaining_length.min(source_range_len - (position - source_range_start));

            ranges.push((
                position - source_range_start + dest_range_start,
                dest_range_len,
            ));

            position += dest_range_len;
            continue;
        }

        // There are more entries after us, append everything up to the closest entry unchanged
        // and keep on going afterwards. */
        if let Some(((source_range_start, _), _)) = map
            .iter()
            .find(|((source_range_start, _), _)| *source_range_start > position)
        {
            let dest_range_len = remaining_length.min(source_range_start - position);
            ranges.push((position, dest_range_len));
            position += dest_range_len;
            continue;
        }

        // Nothing left, we should be done (append as is, and return).
        ranges.push((position, remaining_length));
        break;
    }

    ranges
}

fn merge(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut i = 0;

    ranges.sort_unstable_by_key(|(range_start, _)| *range_start);
    while i < ranges.len() - 1 {
        if ranges[i].0 + ranges[i].1 >= ranges[i + 1].0 {
            ranges[i].1 = ranges[i + 1].0 + ranges[i + 1].1 - ranges[i].0;
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    ranges
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
        .map(|seed| seed.parse().unwrap())
        .tuples()
        .collect_vec();

    let mut maps = Vec::new();
    for _ in 0..7 {
        while lines.peek().unwrap().is_empty() {
            _ = lines.next();
        }

        _ = lines.next();
        let mut map = Vec::new();

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

            map.push(((source_range_start, range_len), dest_range_start));
        }

        map.sort_unstable_by_key(|((source_range_start, _), _)| *source_range_start);
        maps.push(map)
    }

    *maps
        .iter()
        .fold(seeds, |ranges, map| {
            merge(
                ranges
                    .iter()
                    .flat_map(|range| translate(map, range))
                    .collect_vec(),
            )
        })
        .iter()
        .map(|(range_start, _)| range_start)
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
