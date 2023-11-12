use itertools::Itertools;
use std::collections::HashSet;
use std::ops::RangeInclusive;

use itertools::any;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Rhomboid {
    b1: i64,
    b2: i64,
    b3: i64,
    b4: i64,
}

impl Rhomboid {
    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.b3 > other.b1 || self.b1 < other.b3 || self.b4 > other.b2 || self.b2 < other.b4 {
            return None;
        }

        Some(Self {
            b1: self.b1.min(other.b1),
            b2: self.b2.min(other.b2),
            b3: self.b3.max(other.b3),
            b4: self.b4.max(other.b4),
        })
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        let mut ret = Vec::new();

        let asc = [i64::MAX, other.b1 + 1, other.b3 - 1, i64::MIN];
        let desc = [i64::MIN, other.b4 - 1, other.b2 + 1, i64::MAX];

        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                let (b1, b3) = (asc[i], asc[i + 1]);
                let (b4, b2) = (desc[j], desc[j + 1]);

                // if (i * 3 + j) % 2 == 1 {
                //     (b1, b3) = (b1 - 1, b3 + 1);
                //     (b4, b2) = (b4 + 1, b2 - 1);
                // }

                let mask = Rhomboid { b1, b2, b3, b4 };
                if let Some(res) = self.intersect(&mask) {
                    ret.push(res);
                }
            }
        }

        ret
    }

    fn as_point(&self) -> (i64, i64) {
        if self.b1 != self.b3 || self.b2 != self.b4 {
            panic!();
        }
        let x = (self.b2 - self.b1) / 2;
        let y = x + self.b1;
        (x, y)
    }

    fn from(sensor: &(i64, i64), beacon: &(i64, i64)) -> Self {
        let dist = (beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.1)) as i64;
        let b1 = sensor.1 - (sensor.0 - dist);
        let b2 = (sensor.0 + dist) + sensor.1;
        Self {
            b1,
            b2,
            b3: b1 - 2 * dist,
            b4: b2 - 2 * dist,
        }
    }

    fn sized(sensor: &(i64, i64), size: i64) -> Self {
        let b1 = sensor.1 - (sensor.0 - size);
        let b2 = (sensor.0 + size) + sensor.1;
        Self {
            b1,
            b2,
            b3: b1 - 2 * size,
            b4: b2 - 2 * size,
        }
    }

    fn includes(&self, n: i64, point: &(i64, i64)) -> bool {
        point.0 + self.b1 >= point.1
            && point.0 + self.b3 <= point.1
            && -point.0 + self.b2 >= point.1
            && -point.0 + self.b4 <= point.1
    }
}

fn print_map(n: i64, rhomboids: &Vec<&Rhomboid>) {
    for i in 0..=n {
        for j in 0..=n {
            if any(rhomboids, |&r| r.includes(n, &(i, j))) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let re = Regex::new(
        r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)",
    )
    .unwrap();

    let mut ans: HashSet<i64> = HashSet::new();
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in include_str!("input.txt").lines() {
        let (sensor, beacon) = re
            .captures(line)
            .map(|caps| {
                let sensor_x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let sensor_y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let beacon_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let beacon_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

                ((sensor_x, sensor_y), (beacon_x, beacon_y))
            })
            .unwrap();

        sensors.push(sensor);
        beacons.push(beacon);
    }

    let rhomboids: Vec<_> = std::iter::zip(sensors, beacons)
        .map(|(s, b)| Rhomboid::from(&s, &b))
        .collect();

    let n: i64 = 4_000_000;
    let mut possible: HashSet<Rhomboid> =
        HashSet::from_iter(vec![Rhomboid::from(&(n / 2, n / 2), &(0, 0))]);

    for r in rhomboids {
        possible = possible.iter().flat_map(|pr| pr.subtract(&r)).collect();
    }

    println!(
        "{:?}",
        possible
            .into_iter()
            .filter(|&r| r.b1 == r.b3 && r.b2 == r.b4)
            .find(|&r| {
                let p = r.as_point();
                p.0 >= 0 && p.0 <= n && p.1 >= 0 && p.1 <= n
            })
            .map(|r| {
                let p = r.as_point();
                p.0 * 4000000 + p.1
            })
    );
}

#[cfg(test)]
mod tests {
    use crate::Rhomboid;

    #[test]
    fn test1() {
        let a = Rhomboid::from(&(10, 10), &(10, 10));
        let b = Rhomboid::from(&(11, 11), &(11, 11));
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn test2() {
        let a = Rhomboid::from(&(10, 10), &(10, 10));
        let b = Rhomboid::from(&(9, 9), &(9, 9));
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn test3() {
        let a = Rhomboid::from(&(10, 10), &(15, 10));
        let b = Rhomboid::from(&(10, 10), &(12, 10));

        assert_eq!(a.intersect(&b), Some(b));
    }

    #[test]
    fn test4() {
        let a = Rhomboid::from(&(10, 10), &(15, 10));
        let b = Rhomboid::from(&(20, 10), &(15, 10));

        assert_eq!(a.intersect(&b), Some(Rhomboid::from(&(15, 10), &(15, 10))));
    }

    #[test]
    fn test_sub1() {
        let a = Rhomboid::from(&(0, 0), &(0, 0));
        let b = Rhomboid::from(&(0, 10), &(0, 10));

        assert_eq!(a.subtract(&b), vec![a]);
    }

    #[test]
    fn test_sub2() {
        let a = Rhomboid::from(&(10, 10), &(10, 10));
        let b = Rhomboid::from(&(10, 10), &(10, 10));

        assert_eq!(a.subtract(&b), Vec::new());
    }

    #[test]
    fn test_sub3() {
        let a = Rhomboid::from(&(10, 10), &(15, 10));
        let b = Rhomboid::from(&(10, 10), &(15, 10));

        assert_eq!(a.subtract(&b), Vec::new());
    }

    #[test]
    fn test_sub4() {
        let a = Rhomboid::from(&(10, 10), &(20, 10));
        let b = Rhomboid::from(&(100, 100), &(100, 90));

        assert_eq!(a.subtract(&b), vec![a]);
    }

    #[test]
    fn test_sub5() {
        let a = Rhomboid::from(&(0, 0), &(0, 24));
        let b = Rhomboid::from(&(0, 0), &(0, 8));

        assert_eq!(
            a.subtract(&b),
            vec![
                Rhomboid::sized(&(-16, 0), 8),
                Rhomboid::sized(&(-8, 8), 8),
                Rhomboid::sized(&(0, 16), 8),
                //
                Rhomboid::sized(&(-8, -8), 8),
                Rhomboid::sized(&(8, 8), 8),
                //
                Rhomboid::sized(&(0, -16), 8),
                Rhomboid::sized(&(8, -8), 8),
                Rhomboid::sized(&(16, 0), 8),
            ]
        )
    }
}
