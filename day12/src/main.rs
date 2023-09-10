#[derive(Clone, Debug)]
struct Pos(usize, usize);

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
fn dfs(p: &Pos, map: &Vec<Vec<u8>>, dist: &mut [Vec<usize>]) {
    for d in DIRS {
        let new_pos = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
        if (0..map.len() as isize).contains(&(new_pos.0 as isize))
            && (0..map[0].len() as isize).contains(&(new_pos.1 as isize))
            && map[new_pos.0][new_pos.1] <= map[p.0][p.1] + 1
            && dist[new_pos.0][new_pos.1] > dist[p.0][p.1] + 1
        {
            dist[new_pos.0][new_pos.1] = dist[p.0][p.1] + 1;
            dfs(&Pos(new_pos.0, new_pos.1), map, dist);
        }
    }
}

fn main() {
    let mut _start = Pos(0, 0);
    let mut end = Pos(0, 0);
    let mut a_pos: Vec<Pos> = Vec::new();

    let mut map: Vec<Vec<u8>> = Vec::new();
    for (i, line) in include_str!("input.txt").lines().enumerate() {
        map.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            let h = match c {
                'S' => {
                    _start = Pos(i, j);
                    0
                }
                'E' => {
                    end = Pos(i, j);
                    b'z' - b'a'
                }
                'a' => {
                    a_pos.push(Pos(i, j));
                    0
                }
                c => (c as u8) - b'a',
            };
            map[i].push(h);
        }
    }

    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];

    for a in a_pos.iter() {
        dist[a.0][a.1] = 0;
        dfs(a, &map, &mut dist);
    }

    println!("{}", dist[end.0][end.1]);
}
