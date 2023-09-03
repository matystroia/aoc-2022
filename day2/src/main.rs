fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let scores: Vec<_> = lines
        .iter()
        .map(|ln| {
            let values: Vec<_> = ln.split(' ').collect();
            (
                values[0].chars().next().unwrap() as u8 - b'A',
                values[1].chars().next().unwrap() as u8 - b'X',
            )
        })
        .map(|(a, b)| rps2(a, b))
        .collect();

    println!("{:#?}", scores.iter().sum::<i64>());
}

fn rps(a: u8, b: u8) -> i64 {
    match (a, b) {
        (x, y) if x == y => (b + 1 + 3).into(),       // Draw
        (x, y) if x == (y + 1) % 3 => (b + 1).into(), // Loss
        _ => (b + 1 + 6).into(),                      // Win
    }
}

fn rps2(a: u8, b: u8) -> i64 {
    match (a, b) {
        (_, 0) => ((a + 2) % 3 + 1).into(),
        (_, 1) => (a + 1 + 3).into(),
        _ => (((a + 1) % 3) + 1 + 6).into(),
    }
}
