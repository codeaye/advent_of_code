fn part_1(t: &str) -> u16 {
    t.lines().fold(0, |acc, x| {
        let t = x.split(" ").collect::<Vec<&str>>();
        let c2: u8 = match t[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("Not a valid move!"),
        };
        let c1: u8 = match t[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Not a valid move!"),
        };
        let win: u8 = {
            match (c1, c2) {
                (1, 3) => 6,
                (3, 2) => 6,
                (2, 1) => 6,
                (x, y) if x == y => 3,
                _ => 0,
            }
        };

        acc + ((win + c1) as u16)
    })
}

fn part_2(t: &str) -> u16 {
    t.lines().fold(0, |acc, x| {
        let t = x.split(" ").collect::<Vec<&str>>();
        let c2: u8 = match t[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("Not a valid move!"),
        };
        let win: u8 = match t[1] {
            "X" => {
                0 + match c2 - 1 {
                    t if t < 1 => 3,
                    t => t,
                }
            }
            "Y" => 3 + c2,
            "Z" => {
                6 + match c2 + 1 {
                    t if t > 3 => 1,
                    t => t,
                }
            }
            _ => panic!("Not a valid move!"),
        };

        acc + win as u16
    })
}

fn main() {
    let str = include_str!("input.txt");
    println!("P1 Total: {}", part_1(&str));
    println!("P2 Total: {}", part_2(&str));
}
