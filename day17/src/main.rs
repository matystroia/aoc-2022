use std::vec;

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    highest_point: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut ret = Board {
            width,
            height,
            cells: vec![vec![false; width]; height],
            highest_point: 0,
        };
        for j in 0..width {
            ret.cells[0][j] = true;
        }
        ret
    }

    fn drop_rock(&mut self, rock: &[Vec<bool>], jets: &Vec<char>, d_jets: usize) -> usize {
        let mut i = self.highest_point() + 4;
        let mut j = 2;
        let mut d_jets = d_jets;

        loop {
            // Left / Right
            let dj: isize = if jets[d_jets % jets.len()] == '<' {
                -1
            } else {
                1
            };
            if !self.touches(rock, i, j + dj) {
                j += dj;
            }
            d_jets += 1;

            // Down
            if self.touches(rock, i - 1, j) {
                break;
            }
            i -= 1;
        }

        for (di, rock_row) in rock.iter().rev().enumerate() {
            for (dj, rock_cell) in rock_row.iter().enumerate() {
                if *rock_cell {
                    self.cells[i + di][j as usize + dj] = true;
                }
            }
        }

        d_jets
    }

    fn touches(&self, rock: &[Vec<bool>], i: usize, j: isize) -> bool {
        for (di, rock_row) in rock.iter().rev().enumerate() {
            for (dj, rock_cell) in rock_row.iter().enumerate() {
                let (ii, jj) = (i + di, j + dj as isize);
                if jj < 0 || jj >= self.width as isize {
                    if !rock_cell {
                        continue;
                    } else {
                        return true;
                    }
                }
                if self.cells[ii][jj as usize] && *rock_cell {
                    return true;
                }
            }
        }
        false
    }

    fn highest_point(&mut self) -> usize {
        for i in self.highest_point..self.height {
            let mut is_empty_row = true;
            for j in 0..self.width {
                if self.cells[i][j] {
                    is_empty_row = false;
                }
            }
            if is_empty_row {
                self.highest_point = i - 1;
                return self.highest_point;
            }
        }
        0
    }

    fn pattern(&mut self) -> Vec<usize> {
        let h = self.highest_point();
        let pattern: Vec<usize> = (0..self.width)
            .map(|j| {
                for i in (0..=h).rev() {
                    if self.cells[i][j] {
                        return i;
                    }
                }
                0
            })
            .collect();

        let min = pattern.iter().min().unwrap();

        pattern.iter().map(|j| j - min).collect()
    }
}

fn main() {
    let jets = include_str!("input.txt").trim().chars().collect::<Vec<_>>();

    let shapes = [
        vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, true, true, true],
        ],
        vec![
            vec![false, false, false, false],
            vec![false, true, false, false],
            vec![true, true, true, false],
            vec![false, true, false, false],
        ],
        vec![
            vec![false, false, false, false],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![true, true, true, false],
        ],
        vec![
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
        ],
        vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, true, false, false],
            vec![true, true, false, false],
        ],
    ];

    let n_jets = jets.len();

    let mut board = Board::new(7, 4 * 1000000);
    let mut i_jet = 0;

    let mut keys = Vec::new();
    let mut heights = Vec::new();

    let mut rocks_left: usize = 1000000000000;

    for k in 0..1000000 {
        let i_rock = k % 5;
        i_jet = board.drop_rock(&shapes[i_rock], &jets, i_jet) % n_jets;
        rocks_left -= 1;

        let key = (i_rock, i_jet, board.pattern());
        if keys.contains(&key) {
            let prev = keys.iter().position(|k| k == &key).unwrap();
            let delta_i = k - prev;
            let delta_h = board.highest_point() - heights[prev];

            let div = rocks_left / delta_i;
            let modulo = rocks_left % delta_i;

            let mut ans = board.highest_point() + div * delta_h;
            ans += heights[prev + modulo] - heights[prev];
            println!("{ans}");
            println!("{modulo}");

            break;
        }

        keys.push(key);
        heights.push(board.highest_point());
    }
}
