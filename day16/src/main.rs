use itertools::Itertools;
use memoize::memoize;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

use priority_queue::PriorityQueue;
use regex::Regex;

fn main() {
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut adj = HashMap::new();
    let mut flow_map = HashMap::new();
    for line in include_str!("input.txt").lines() {
        let (valve, flow, to_valves) = re
            .captures(line)
            .map(|caps| {
                let valve = caps.get(1).unwrap().as_str();
                let flow = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let to_valves = caps.get(3).unwrap().as_str();

                let to_valves: Vec<_> = to_valves.split(", ").collect();
                (valve, flow, to_valves)
            })
            .unwrap();

        adj.insert(valve, to_valves.iter().map(|&v| (v, 1)).collect::<Vec<_>>());
        flow_map.insert(valve, flow);
    }

    let flow_nodes: HashSet<&str> = flow_map
        .iter()
        .filter(|(_, &v)| v != 0)
        .map(|(&k, _)| k)
        .collect();

    let str_to_int: HashMap<&str, i32> = flow_nodes
        .iter()
        .enumerate()
        .map(|(i, &s)| (s, i as i32))
        .collect();

    let mut flow_map: HashMap<i32, i32> = flow_map
        .iter()
        .filter(|&(_, f)| *f > 0)
        .map(|(&n, f)| (str_to_int[n], *f))
        .collect();
    flow_map.insert(-1, 0);

    let mut adj2: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for &node in flow_nodes.iter() {
        adj2.insert(
            str_to_int[node],
            dijkstra(&adj, node)
                .into_iter()
                .filter(|&(k, _)| k != node && flow_nodes.contains(k))
                .map(|(n, d)| (str_to_int[n], d))
                .collect::<Vec<_>>(),
        );
    }

    // for &start in flow_nodes.iter() {
    //     let dist = dijkstra(&adj, "AA")[start];
    //     println!(
    //         "{start} {dist} {}",
    //         dp(start, 30 - dist, &HashSet::new(), 0, &adj2, &flow_map)
    //     );
    // }

    let dist_from_aa = dijkstra(&adj, "AA");
    println!("{:?}", dist_from_aa);

    adj2.insert(
        -1,
        dist_from_aa
            .into_iter()
            .filter(|&(k, _)| flow_nodes.contains(k))
            .map(|(n, d)| (str_to_int[n], d))
            .collect_vec(),
    );

    println!("{adj2:?}");

    let one_guy = (0..=26)
        .map(|remaining| dp(-1, remaining, &HashSet::new(), 0, &adj2, &flow_map))
        .collect();

    println!("{flow_map:?}");
    println!("{one_guy:?}");
    // for remaining in 0..=30 {
    //     println!(
    //         "{}",
    //         dp(-1, remaining, &HashSet::new(), 0, &adj2, &flow_map)
    //     )
    // }

    println!(
        "{}",
        dp2(
            -1,
            -1,
            0,
            0,
            0,
            26,
            0,
            &adj2,
            &flow_map,
            &mut HashMap::new(),
            &one_guy
        )
    );

    // for (i, &start1) in flow_nodes.iter().enumerate() {
    //     for &start2 in flow_nodes.iter().take(i) {
    //         let (dist1, dist2) = (dist_from_aa[start1], dist_from_aa[start2]);
    //         println!(
    //             "{start1} {start2} {}",
    //             dp2(
    //                 start1,
    //                 start2,
    //                 dist1,
    //                 dist2,
    //                 &HashSet::new(),
    //                 26,
    //                 0,
    //                 &adj2,
    //                 &flow_map,
    //             )
    //         );
    //     }
    // }
    // println!(
    //     "{:?}",
    //     dp2(
    //         "DD",
    //         "JJ",
    //         dist_from_aa["DD"],
    //         dist_from_aa["JJ"],
    //         &HashSet::new(),
    //         26,
    //         0,
    //         &adj2,
    //         &flow_map,
    //     ),
    // );

    // let mut node_map: HashMap<i32, i32> = HashMap::new();
    // for (i, &str) in flow_map.keys().enumerate() {
    //     node_map.insert(str, i as i32);
    // }

    // println!("{}", dp("DD", 29, &HashSet::new(), 0, &adj2, &flow_map));

    // println!(
    //     "{}",
    //     dp2(
    //         "AA",
    //         "AA",
    //         &HashSet::new(),
    //         &HashSet::new(),
    //         30,
    //         &HashSet::new(),
    //         0,
    //         &adj2,
    //         &flow_map
    //     )
    // );
}

fn dp(
    start: i32,
    remaining: i32,
    visited: &HashSet<i32>,
    score: i32,
    adj: &HashMap<i32, Vec<(i32, i32)>>,
    flow_map: &HashMap<i32, i32>,
) -> i32 {
    if remaining <= 0 {
        return score;
    }

    let mut new_visited = visited.clone();

    let (mut new_score, mut new_remaining) = (score, remaining);
    if flow_map[&start] > 0 && !visited.contains(&start) {
        new_visited.insert(start);
        new_score += (remaining - 1) * flow_map[&start];
        new_remaining -= 1;
    }

    let mut max = new_score;
    for (neighbor, cost) in adj[&start].iter().filter(|&(n, _)| !visited.contains(n)) {
        max = max.max(dp(
            *neighbor,
            new_remaining - cost,
            &new_visited,
            new_score,
            adj,
            flow_map,
        ))
    }

    max
}

// fn dp_part2(
//     start1: &str,
//     start2: &str,
//     skip1: i32,
//     skip2: i32,
//     remaining: i32,
//     visited: &HashSet<&str>,
//     score: i32,
//     adj: &HashMap<&str, Vec<(&str, i32)>>,
//     flow_map: &HashMap<&str, i32>,
// ) -> i32 {
//     if remaining <= 0 {
//         return score;
//     }

//     let mut new_visited = visited.clone();
//     let mut new_score = score;
//     let (mut valve1, mut valve2) = (false, false);

//     if skip1 == 0 && flow_map[start1] > 0 && !new_visited.contains(start1) {
//         new_visited.insert(start1);
//         new_score += (remaining - 1) * flow_map[start1];
//         valve1 = true;
//     }

//     if skip2 == 0 && flow_map[start2] > 0 && !new_visited.contains(start2) {
//         new_visited.insert(start2);
//         new_score += (remaining - 1) * flow_map[start2];
//         valve2 = true;
//     }

//     let neighbors1 = if valve1 {
//         vec![(start1, 0)]
//     } else {
//         adj[start1]
//             .iter()
//             .filter(|&(n, _)| !visited.contains(n))
//             .map(|&(n, cost)| (n, cost))
//             .collect()
//     };

//     let mut max = new_score;
//     for &(neighbor1, cost1) in neighbors1 {
//         for &(neighbor2, cost2) in adj[start2].iter().filter(|&(n, _)| !visited.contains(n)) {
//             max = max.max(dp_part2(
//                 neighbor1, neighbor2, skip1, skip2, remaining, visited, score, adj, flow_map,
//             ))
//         }
//     }

//     max
// }

fn dp2(
    start1: i32,
    start2: i32,
    skip1: i32,
    skip2: i32,
    visited: i32,
    remaining: i32,
    score: i32,
    adj: &HashMap<i32, Vec<(i32, i32)>>,
    flow_map: &HashMap<i32, i32>,
    memo: &mut HashMap<(i32, i32, i32, i32, i32, i32, i32), i32>,
    one_guy: &Vec<i32>,
) -> i32 {
    if remaining < 0 || (skip1 > remaining && skip2 > remaining) {
        return 0;
    }
    if remaining == 0 {
        return score;
    }

    let elapsed = 26 - remaining;
    if score < one_guy[elapsed as usize] {
        return -9999;
    }

    let key = (start1, start2, skip1, skip2, visited, remaining, score);
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }

    let mut new_visited = visited;
    let mut new_score = score;
    let (mut valve1, mut valve2) = (false, false);

    if flow_map[&start1] > 0 && (new_visited & (1 << start1)) == 0 && skip1 == 0 {
        new_visited |= 1 << start1;
        new_score += (remaining - 1) * flow_map[&start1];
        valve1 = true;
    }

    if flow_map[&start2] > 0 && (new_visited & (1 << start2)) == 0 && skip2 == 0 {
        new_visited |= 1 << start2;
        new_score += (remaining - 1) * flow_map[&start2];
        valve2 = true;
    }

    // if new_visited == ((1 << 15) - 1) {
    //     return score;
    // }

    let next1 = if valve1 {
        vec![(start1, 1)]
    } else if skip1 > 0 {
        vec![(start1, skip1)]
    } else {
        adj[&start1]
            .iter()
            .filter(|&(n, _)| (visited & (1 << n) == 0))
            .map(|&(n, cost)| (n, cost))
            .collect()
    };

    let next2 = if valve2 {
        vec![(start2, 1)]
    } else if skip2 > 0 {
        vec![(start2, skip2)]
    } else {
        adj[&start2]
            .iter()
            .filter(|&(n, _)| (visited & (1 << n) == 0))
            .map(|&(n, cost)| (n, cost))
            .collect()
    };

    let mut max = new_score;
    for (n1, c1) in next1.iter() {
        for (n2, c2) in next2.iter() {
            if start1 == -1 && start2 == -1 {
                println!("{n1} {n2}");
            }
            let min = c1.min(c2);

            max = max.max(dp2(
                *n1,
                *n2,
                c1 - min,
                c2 - min,
                new_visited,
                remaining - min,
                new_score,
                adj,
                flow_map,
                memo,
                one_guy,
            ));
        }
    }

    memo.insert(key, max);
    let other_key = (start2, start1, skip2, skip1, visited, remaining, score);
    memo.insert(other_key, max);
    max
}

fn dijkstra<'a>(adj: &HashMap<&str, Vec<(&'a str, i32)>>, src: &'a str) -> HashMap<&'a str, i32> {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();

    pq.push(src, 0);
    visited.insert(src);

    while !pq.is_empty() {
        let (node, priority) = pq.pop().unwrap();
        let priority = -priority;

        dist.insert(node, priority);

        for &(neighbor, cost) in adj.get(node).unwrap() {
            if visited.contains(neighbor) {
                continue;
            }

            let new_priority = priority + cost;
            if new_priority < *dist.get(neighbor).unwrap_or(&i32::MAX) {
                pq.push(neighbor, -new_priority);
            }
        }
    }

    dist
}
