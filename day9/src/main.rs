use std::collections::{HashMap, HashSet};

fn main() {
    let dirs = HashMap::from([("U", (0, -1)), ("D", (0, 1)), ("L", (-1, 0)), ("R", (1, 0))]);

    let mut knots = [(0, 0); 10];

    let mut visited = HashSet::from([(0, 0)]);

    for line in include_str!("input.txt").lines() {
        let values = line.split(' ').collect::<Vec<_>>();
        let (dir, count) = (values[0], values[1].parse::<i32>().unwrap());

        let d = dirs[dir];
        for _ in 0..count {
            knots[0] = (knots[0].0 + d.0, knots[0].1 + d.1);

            for i in 1..10 {
                if !is_adjacent(&knots[i - 1], &knots[i]) {
                    knots[i] = move_tail(&knots[i - 1], &knots[i]);
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    println!("{}", visited.len());
}

fn is_adjacent((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> bool {
    (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if tail.0 == head.0 {
        (tail.0, tail.1 + (head.1 - tail.1) / 2)
    } else if tail.1 == head.1 {
        (tail.0 + (head.0 - tail.0) / 2, tail.1)
    } else if (tail.0 - head.0).abs() == 1 {
        (head.0, tail.1 + (head.1 - tail.1) / 2)
    } else if (tail.1 - head.1).abs() == 1 {
        (tail.0 + (head.0 - tail.0) / 2, head.1)
    } else {
        (
            tail.0 + (head.0 - tail.0) / 2,
            tail.1 + (head.1 - tail.1) / 2,
        )
    }
}
