pub fn part_one(text: &str) {
    let mut c = text.split("\n\n").map(|f| {
        f.split("\n")
            .map(|e| e.parse::<u32>().unwrap())
            .sum::<u32>()
    });
    let Some(val) = c.clone().max() else {
        panic!("No max amount found :(")
    };
    let pos = c.position(|r| r == val);
    let Some(pos) = pos else {
        panic!("That should physically not be possible..")
    };
    println!("Value: {}\nElf Number: {}", val, pos + 1)
}

pub fn part_two(text: &str) {
    let mut c = text
        .split("\n\n")
        .map(|f| {
            f.split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    c.sort();
    let top_three: u32 = c.iter().rev().take(3).sum();
    println!("Value: {}", top_three)
}
