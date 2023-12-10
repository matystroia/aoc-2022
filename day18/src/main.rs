use itertools::iproduct;
use std::collections::HashSet;

fn main() {
    let mut map = [[[false; 25]; 25]; 25];
    let mut cubes: Vec<(usize, usize, usize)> = Vec::new();

    for line in include_str!("input.txt").lines() {
        let coords = line
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (x, y, z) = (coords[0] + 2, coords[1] + 2, coords[2] + 2);
        cubes.push((x, y, z));
        map[x][y][z] = true;
    }

    let mut ans = 0;
    for cube in cubes {
        for (x, y, z) in get_neighbors(&cube) {
            if (x < 0 || y < 0 || z < 0) || !map[x as usize][y as usize][z as usize] {
                ans += 1;
            }
        }
    }

    let mut ret = Ret { count: 0 };
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    fill(vec![&(0, 0, 0)], map, &mut visited, &mut ret);

    println!("{ans}");
    println!("{}", ret.count);
}

struct Ret {
    count: usize,
}

fn get_neighbors(pos: &(usize, usize, usize)) -> Vec<(i32, i32, i32)> {
    iproduct!(-1..=1_i32, -1..=1_i32, -1..=1_i32)
        .filter_map(|(dx, dy, dz)| {
            if dx.abs() + dy.abs() + dz.abs() == 1 {
                return Some((pos.0 as i32 + dx, pos.1 as i32 + dy, pos.2 as i32 + dz));
            }
            None
        })
        .collect()
}

fn fill(
    positions: Vec<&(usize, usize, usize)>,
    map: [[[bool; 25]; 25]; 25],
    visited: &mut HashSet<(usize, usize, usize)>,
    ret: &mut Ret,
) {
    if positions.is_empty() {
        return;
    }

    visited.extend(positions.clone());
    let mut next_pos = HashSet::new();

    for pos in positions.iter() {
        for (x, y, z) in get_neighbors(pos) {
            if !(0..25).contains(&x) || !(0..25).contains(&y) || !(0..25).contains(&z) {
                continue;
            }

            if map[x as usize][y as usize][z as usize] {
                ret.count += 1;
                continue;
            }

            if !visited.contains(&(x as usize, y as usize, z as usize)) {
                next_pos.insert((x as usize, y as usize, z as usize));
            }
        }
    }

    fill(next_pos.iter().collect::<Vec<_>>(), map, visited, ret);
}
