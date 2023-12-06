#![feature(test)]
extern crate test;

use itertools::Itertools;
use std::{fs::read_to_string, str::Lines};

pub fn solve(lines: Lines) -> i64 {
    let lines = lines.collect_vec();

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let mut line_accum = 0;

            for j in line
                .chars()
                .enumerate()
                .filter_map(|(j, ch)| if ch == '*' { Some(j) } else { None })
            {
                let bbox_y_start = if i > 0 { i - 1 } else { i };
                let bbox_x_start = if j > 0 { j - 1 } else { j };

                let bbox_lines = (bbox_y_start..bbox_y_start + 3).map(|y| {
                    lines
                        .iter()
                        .nth(y)
                        .unwrap_or(&"")
                        .chars()
                        .skip(bbox_x_start)
                        .take(3)
                });

                let (mut first_num, mut first_num_y, mut first_num_end) = (None::<i64>, 0, 0);
                let mut second_num = None::<i64>;
                for (bbox_y, bbox_line) in bbox_lines.enumerate() {
                    let full_bbox_line = lines.iter().nth(bbox_y_start + bbox_y).unwrap_or(&"");
                    let full_bbox_line_size = full_bbox_line.chars().count();

                    for num_pos in bbox_line.clone().enumerate().filter_map(|(num_pos, ch)| {
                        if ch.is_digit(10) {
                            Some(num_pos)
                        } else {
                            None
                        }
                    }) {
                        if first_num.is_some()
                            && first_num_y == bbox_y_start + bbox_y
                            && first_num_end > bbox_x_start + num_pos
                        {
                            continue;
                        }

                        let num_start = full_bbox_line
                            .chars()
                            .rev()
                            .skip(full_bbox_line_size - bbox_x_start - num_pos)
                            .position(|ch| !ch.is_digit(10))
                            .and_then(|pos| Some(bbox_x_start + num_pos - pos))
                            .unwrap_or(0);

                        let num_end = full_bbox_line
                            .chars()
                            .skip(bbox_x_start + num_pos)
                            .position(|ch| !ch.is_digit(10))
                            .and_then(|pos| Some(bbox_x_start + num_pos + pos))
                            .unwrap_or(full_bbox_line_size);

                        if first_num.is_none() {
                            first_num = Some(full_bbox_line[num_start..num_end].parse().unwrap());
                            first_num_y = bbox_y_start + bbox_y;
                            first_num_end = num_end;
                        } else {
                            second_num = Some(full_bbox_line[num_start..num_end].parse().unwrap());
                            break;
                        }
                    }

                    if first_num.is_some() && second_num.is_some() {
                        line_accum += first_num.unwrap() * second_num.unwrap();
                        break;
                    }
                }
            }

            line_accum
        })
        .sum()
}

fn main() {
    println!(
        "{}",
        solve(read_to_string("input/day3.txt").unwrap().lines())
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let text = read_to_string("../input/day3.txt").unwrap();
        b.iter(|| crate::solve(text.lines()));
    }
}
