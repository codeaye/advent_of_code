static ARR: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn part_1(t: &str) -> usize {
    t.lines().fold(0, |acc, f| {
        let (a, b) = f.split_at(f.len() / 2);
        for i in a.chars() {
            if b.contains(i) {
                return acc + ARR.iter().position(|&x| x == i).unwrap() + 1;
            }
        }
        acc
    })
}

fn part_2(t: &str) -> usize {
    let mut k = t.lines();
    let mut total = 0usize;
    while let [Some(a), Some(b), Some(c)] = [k.next(), k.next(), k.next()] {
        for i in a.chars() {
            if b.contains(i) && c.contains(i) {
                total += ARR.iter().position(|&x| x == i).unwrap() + 1;
                break;
            }
        }
    }

    total
}

pub static IN_1: &str = include_str!("input.txt");
pub static IN_2: &str = include_str!("input2.txt");

fn main() {
    println!("P1 Priorities: {}", part_1(IN_1));
    println!("P2 Priorities: {}", part_2(IN_2))
}

mod day_3_tests {
    use crate::{part_1, part_2, IN_1, IN_2};
    use std::time::Instant;
    #[test]
    pub fn bench() {
        let start = Instant::now();
        part_1(&IN_1);
        part_2(&IN_2);
        let elapsed = start.elapsed();
        println!("Elapsed: {:?}", elapsed);
    }
}
