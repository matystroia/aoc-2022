const N: usize = 200;
const M: usize = 1500;

fn main() {
    let mut map = [[false; M]; N];
    let mut max_i = 0;

    for line in include_str!("input.txt").lines() {
        let points: Vec<_> = line
            .split("->")
            .map(|p| {
                let values: Vec<_> = p
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                (values[0], values[1])
            })
            .collect();

        for window in points.windows(2) {
            let (a, b) = (window[0], window[1]);
            for p in get_line(a, b) {
                max_i = max_i.max(p.1);
                map[p.1][p.0] = true;
            }
        }
    }

    for j in 0..M {
        map[max_i + 2][j] = true;
    }

    _print_map(&map);

    let mut i = 0;
    loop {
        i += 1;
        if add_grain(&mut map) {
            println!("{i}");
            break;
        }
    }
}

fn get_line(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    if a.0 == b.0 {
        (a.1.min(b.1)..=b.1.max(a.1)).map(|y| (a.0, y)).collect()
    } else {
        (a.0.min(b.0)..=b.0.max(a.0)).map(|x| (x, a.1)).collect()
    }
}

fn _print_map(map: &[[bool; M]]) {
    for row in map.iter().take(15) {
        println!(
            "{}",
            row[490..510]
                .iter()
                .map(|x| if *x { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn add_grain(map: &mut [[bool; M]]) -> bool {
    let mut p = (0, 500);
    'a: loop {
        let tries = [(p.0 + 1, p.1), (p.0 + 1, p.1 - 1), (p.0 + 1, p.1 + 1)];
        for new_p in tries {
            if !map[new_p.0][new_p.1] {
                p = new_p;
                continue 'a;
            }
        }
        if p == (0, 500) {
            return true;
        }
        break;
    }
    map[p.0][p.1] = true;
    false
}
