use itertools::{self, Itertools};

struct Pair {
    from: i64,
    to: i64,
}

impl Pair {
    fn from(s: &str) -> Pair {
        let values: Vec<_> = s.split('-').collect();
        Pair {
            from: values[0].parse::<i64>().unwrap(),
            to: values[1].parse::<i64>().unwrap(),
        }
    }

    fn contains(&self, other: &Pair) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Pair) -> bool {
        !(self.to < other.from || self.from > other.to)
    }
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();

    let mut ans = 0;
    for line in lines {
        let (first, second): (&str, &str) = line.split(',').collect_tuple().unwrap();
        let (first_pair, second_pair) = (Pair::from(first), Pair::from(second));

        if first_pair.overlaps(&second_pair) {
            ans += 1
        }
    }
    println!("{ans}");
}
