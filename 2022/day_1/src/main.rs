fn solution(text: &str) {
    let mut c: Vec<u32> = text
        .split("\n\n")
        .map(|f| {
            f.split("\n")
                .map(|e| e.parse::<u32>().expect("Input is wrong."))
                .sum::<u32>()
        })
        .collect();
    c.sort();
    c.reverse();
    println!("P1: {}\nP2: {}", c[0], c[0..3].iter().sum::<u32>());
}

pub static IN: &str = include_str!("./input.txt");

fn main() {
    solution(&IN);
}

mod day_2_tests {
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
