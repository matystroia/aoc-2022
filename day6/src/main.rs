use std::collections::VecDeque;

fn main() {
    let mut chars = include_str!("input.txt").lines().next().unwrap().chars();

    let mut prev = VecDeque::new();
    for _ in 0..14 {
        prev.push_back(chars.next().unwrap());
    }

    let mut i = 14;
    for c in chars {
        if check(&prev) {
            break;
        }
        add(&mut prev, c);
        i += 1;
    }

    println!("{i}")
}

fn add(arr: &mut VecDeque<char>, c: char) {
    arr.pop_front();
    arr.push_back(c);
}

fn check(arr: &VecDeque<char>) -> bool {
    for (i, x) in arr.iter().enumerate() {
        for y in arr.iter().take(i) {
            if x == y {
                return false;
            }
        }
    }
    true
}
