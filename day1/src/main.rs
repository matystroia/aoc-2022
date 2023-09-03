use itertools::Itertools;

fn main() {
    let lines: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|ln| ln.parse::<i64>().ok())
        .collect();

    let groups = lines
        .split(|ln| ln.is_none())
        .map(|group| group.iter().map(|x| x.unwrap()).sum::<i64>());

    let ans = groups.sorted_by_key(|x| -x).take(3).sum::<i64>();

    println!("{ans}");
}
