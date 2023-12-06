#![feature(test)]
extern crate test;

use std::{fs::read_to_string, str::Lines};

pub fn solve(mut lines: Lines) -> i64 {
    let time = lines.next().unwrap()[5..]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distance = lines.next().unwrap()[9..]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let a = -1f64;
    let b = time as f64;
    let c = -distance as f64;
    let delta = b * b - 4f64 * a * c;
    let x0 = (-b + delta.sqrt()) / (2f64 * a);
    let x1 = (-b - delta.sqrt()) / (2f64 * a);
    (x1.ceil() - x0.ceil()) as i64
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day6.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day6.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
