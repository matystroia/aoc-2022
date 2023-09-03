use std::collections::HashSet;

fn main() {
    let lines = include_str!("input.txt").lines();

    // let common_items: Vec<char> = lines
    //     .map(|ln| ln.split_at(ln.len() / 2))
    //     .map(|(left, right)| {
    //         let left_set: HashSet<char> = left.chars().collect();
    //         let right_set: HashSet<char> = right.chars().collect();
    //         *left_set.intersection(&right_set).next().unwrap()
    //     })
    //     .collect();

    let badges: Vec<char> = lines
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let sets: Vec<_> = chunk
                .iter()
                .map(|ln| ln.chars().collect::<HashSet<char>>())
                .collect();

            let intersection: HashSet<_> = sets[0].intersection(&sets[1]).copied().collect();
            *intersection.intersection(&sets[2]).next().unwrap()
        })
        .collect();

    let priorities: Vec<i64> = badges
        .iter()
        .map(|c| {
            if c.is_uppercase() {
                ((*c as u8) - b'A' + 27).into()
            } else {
                ((*c as u8) - b'a' + 1).into()
            }
        })
        .collect();

    println!("{}", priorities.iter().sum::<i64>());
}
