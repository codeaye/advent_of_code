fn solution(text: &str) {
    let mut c: Vec<u32> = text
        .split("\n\n")
        .map(|f| {
            f.split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    c.sort();
    c.reverse();
    println!("P1: {}\nP2: {}", c[0], c[0..3].iter().sum::<u32>());
}

fn main() {
    solution(&include_str!("./input.txt"));
}
