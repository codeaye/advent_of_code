use lazy_static::lazy_static;
use regex::Regex;

fn solution(t: &str) -> (usize, usize) {
    let (mut pt1, mut pt2) = (0, 0);
    for x in t.lines() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\D").unwrap();
        }
        let m = RE
            .split(x)
            .map(|f| f.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        pt1 += match (m[0], m[1], m[2], m[3]) {
            (a, b, x, y) if x >= a && y <= b => 1,
            (a, b, x, y) if a >= x && b <= y => 1,
            _ => 0,
        };

        pt2 += match (m[0], m[1], m[2], m[3]) {
            (x1, y1, x2, y2) if x1 <= y2 && y1 >= x2 => 1,
            _ => 0,
        };
    }
    (pt1, pt2)
}

static IN: &str = include_str!("input.txt");

fn main() {
    let sol = solution(&IN);
    println!("Part 1: {}\nPart 2: {}", sol.0, sol.1);
}

mod day_4_tests {
    use crate::{solution, IN};
    use std::time::Instant;
    #[test]
    pub fn bench() {
        let start = Instant::now();
        solution(&IN);
        let elapsed = start.elapsed();
        println!("Elapsed: {:?}", elapsed);
    }
}
