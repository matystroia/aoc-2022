fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|ln| {
            ln.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut _ans = grid.len() * 2 + grid[0].len() * 2 - 4;
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            if is_visible(&grid, i, j) {
                _ans += 1;
            }
        }
    }

    let mut max = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            max = max.max(scenic_score(&grid, i, j));
        }
    }

    println!("{max}");
}

fn is_visible(grid: &[Vec<u32>], i: usize, j: usize) -> bool {
    let h = grid[i][j];
    let directions = [
        iter_top(grid, i, j),
        iter_bottom(grid, i, j),
        iter_left(grid, i, j),
        iter_right(grid, i, j),
    ];

    'a: for iter in directions {
        for (ii, jj) in iter {
            if grid[ii][jj] >= h {
                continue 'a;
            }
        }
        return true;
    }
    false
}

fn scenic_score(grid: &[Vec<u32>], i: usize, j: usize) -> usize {
    let h = grid[i][j];
    let direction_scores = [
        iter_top(grid, i, j),
        iter_bottom(grid, i, j),
        iter_left(grid, i, j),
        iter_right(grid, i, j),
    ]
    .map(|iter| {
        let mut s = 0;
        for (ii, jj) in iter {
            s += 1;
            if grid[ii][jj] >= h {
                break;
            }
        }
        s
    });

    direction_scores.iter().product::<usize>()
}

fn iter_top(_grid: &[Vec<u32>], i: usize, j: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new((0..i).rev().map(move |ii| (ii, j)))
}

fn iter_bottom(grid: &[Vec<u32>], i: usize, j: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new((i + 1..grid.len()).map(move |ii| (ii, j)))
}

fn iter_left(_grid: &[Vec<u32>], i: usize, j: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new((0..j).rev().map(move |jj| (i, jj)))
}

fn iter_right(grid: &[Vec<u32>], i: usize, j: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new((j + 1..grid[0].len()).map(move |jj| (i, jj)))
}
