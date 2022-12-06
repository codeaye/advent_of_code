fn unique(strn: &str) -> bool {
    let mut set = std::collections::HashSet::new();
    for ch in strn.chars() {
        if set.contains(&ch) {
            return false;
        }
        set.insert(ch);
    }
    true
}

fn solution(t: &str, stepper: u8) -> usize {
    for (i, _) in t.chars().skip(stepper as usize - 1).enumerate() {
        let it = i as i32 - stepper as i32;
        if it >= 0 {
            let k = &t[it as usize..i];
            if unique(&k) {
                return i;
            }
        }
    }
    0
}

static IN: &str = include_str!("input.txt");

fn main() {
    println!("P1: {}", solution(IN, 4));
    println!("P2: {}", solution(IN, 14));
}

#[test]
fn test_part1() {
    assert_eq!(7, solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
}
