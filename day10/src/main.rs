fn main() {
    let mut cycles = Vec::from([1]);

    for line in include_str!("input.txt").lines() {
        if line == "noop" {
            cycles.push(*cycles.last().unwrap());
        } else {
            let y: i32 = line.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
            cycles.push(*cycles.last().unwrap());
            cycles.push(cycles.last().unwrap() + y);
        }
    }

    let _ans: i32 = [20, 60, 100, 140, 180, 220]
        .map(|i| (i as i32) * cycles[i])
        .iter()
        .sum();

    for row in 0..6 {
        for i in 0..40 {
            if (cycles[row * 40 + i] - (i as i32)).abs() <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}
