use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
enum Cost {
    Ore(i32),
    OreClay(i32, i32),
    OreObsidian(i32, i32),
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    costs: HashMap<Robot, Cost>,
}

impl Blueprint {
    fn max_ore_cost(&self) -> i32 {
        self.costs
            .values()
            .map(|cost| match cost {
                Cost::Ore(ore) => *ore,
                Cost::OreClay(ore, _) => *ore,
                Cost::OreObsidian(ore, _) => *ore,
            })
            .max()
            .unwrap()
    }
    fn max_clay_cost(&self) -> i32 {
        self.costs
            .values()
            .map(|cost| match cost {
                Cost::OreClay(_, clay) => *clay,
                _ => 0,
            })
            .max()
            .unwrap()
    }
    fn max_obsidian_cost(&self) -> i32 {
        self.costs
            .values()
            .map(|cost| match cost {
                Cost::OreObsidian(_, obsidian) => *obsidian,
                _ => 0,
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct State<'a> {
    blueprint: &'a Blueprint,
    time_left: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    resources: Resources,
}

impl State<'_> {
    fn new(blueprint: &Blueprint) -> State {
        State {
            blueprint,
            time_left: 32,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }

    fn wait(mut self, time: i32) -> Self {
        self.resources.ore += self.ore_robots * time;
        self.resources.clay += self.clay_robots * time;
        self.resources.obsidian += self.obsidian_robots * time;
        self.resources.geode += self.geode_robots * time;

        self.time_left -= time;
        self
    }

    fn build_robot(mut self, robot: &Robot) -> Self {
        self.resources = self.resources - self.blueprint.costs[robot];
        let mut ret = self.wait(1);
        ret.wait(1);

        match robot {
            Robot::Ore => ret.ore_robots += 1,
            Robot::Clay => ret.clay_robots += 1,
            Robot::Obsidian => ret.obsidian_robots += 1,
            Robot::Geode => ret.geode_robots += 1,
        };

        ret
    }
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl std::ops::Sub<Cost> for Resources {
    type Output = Self;
    fn sub(self, rhs: Cost) -> Self::Output {
        match rhs {
            Cost::Ore(ore) => Resources {
                ore: self.ore - ore,
                clay: self.clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Cost::OreClay(ore, clay) => Resources {
                ore: self.ore - ore,
                clay: self.clay - clay,
                obsidian: self.obsidian,
                geode: self.geode,
            },
            Cost::OreObsidian(ore, obsidian) => Resources {
                ore: self.ore - ore,
                clay: self.clay,
                obsidian: self.obsidian - obsidian,
                geode: self.geode,
            },
        }
    }
}

impl State<'_> {
    fn missing_resources(&self, cost: &Cost) -> Option<Resources> {
        match cost {
            Cost::Ore(ore) => {
                if self.resources.ore >= *ore {
                    None
                } else {
                    Some(Resources {
                        ore: ore - self.resources.ore,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    })
                }
            }
            Cost::OreClay(ore, clay) => {
                if self.resources.ore >= *ore && self.resources.clay >= *clay {
                    None
                } else {
                    Some(Resources {
                        ore: ore - self.resources.ore,
                        clay: clay - self.resources.clay,
                        obsidian: 0,
                        geode: 0,
                    })
                }
            }
            Cost::OreObsidian(ore, obsidian) => {
                if self.resources.ore >= *ore && self.resources.obsidian >= *obsidian {
                    None
                } else {
                    Some(Resources {
                        ore: ore - self.resources.ore,
                        clay: 0,
                        obsidian: obsidian - self.resources.obsidian,
                        geode: 0,
                    })
                }
            }
        }
    }
}

fn backtracking(blueprint: &Blueprint, state: State) -> i32 {
    if state.time_left < 0 {
        return 0;
    }
    if state.time_left == 0 {
        return state.resources.geode;
    }

    let mut new_states: Vec<State> = Vec::new();

    // Build ore robot
    if state.ore_robots < blueprint.max_ore_cost() {
        match state.missing_resources(&blueprint.costs[&Robot::Ore]) {
            Some(resources) => {
                let ore_time = (resources.ore as f32 / state.ore_robots as f32).ceil() as i32;
                if ore_time < state.time_left {
                    new_states.push(state.wait(ore_time).build_robot(&Robot::Ore));
                }
            }
            None => {
                new_states.push(state.build_robot(&Robot::Ore));
            }
        }
    }

    // Build clay robot
    if state.clay_robots < blueprint.max_clay_cost() {
        match state.missing_resources(&blueprint.costs[&Robot::Clay]) {
            Some(resources) => {
                let ore_time = (resources.ore as f32 / state.ore_robots as f32).ceil() as i32;
                if ore_time < state.time_left {
                    new_states.push(state.wait(ore_time).build_robot(&Robot::Clay));
                }
            }
            None => {
                new_states.push(state.build_robot(&Robot::Clay));
            }
        }
    }

    // Build obsidian robot
    if state.clay_robots > 0 && state.obsidian_robots < blueprint.max_obsidian_cost() {
        match state.missing_resources(&blueprint.costs[&Robot::Obsidian]) {
            Some(resources) => {
                let ore_time = (resources.ore as f32 / state.ore_robots as f32).ceil() as i32;
                let clay_time = (resources.clay as f32 / state.clay_robots as f32).ceil() as i32;
                let time = ore_time.max(clay_time);
                if time < state.time_left {
                    new_states.push(state.wait(time).build_robot(&Robot::Obsidian));
                }
            }
            None => {
                new_states.push(state.build_robot(&Robot::Obsidian));
            }
        }
    }

    // Build geode robot
    if state.obsidian_robots > 0 {
        match state.missing_resources(&blueprint.costs[&Robot::Geode]) {
            Some(resources) => {
                let ore_time = (resources.ore as f32 / state.ore_robots as f32).ceil() as i32;
                let obsidian_time =
                    (resources.obsidian as f32 / state.obsidian_robots as f32).ceil() as i32;
                let time = ore_time.max(obsidian_time);
                new_states.push(state.wait(time).build_robot(&Robot::Geode));
            }
            None => {
                new_states.push(state.build_robot(&Robot::Geode));
            }
        }
    }

    if new_states.is_empty() {
        new_states.push(state.wait(state.time_left));
    }

    // Return max of new_states
    new_states
        .into_iter()
        .map(|state| backtracking(blueprint, state))
        .max()
        .unwrap()
}

fn main() {
    let re = Regex::new(r"\b\d+\b").unwrap();
    let blueprints: Vec<Blueprint> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let matches: Vec<i32> = re
                .find_iter(line)
                .map(|matches| matches.as_str().parse::<i32>().unwrap())
                .collect();

            let mut costs: HashMap<Robot, Cost> = HashMap::new();
            costs.insert(Robot::Ore, Cost::Ore(matches[1]));
            costs.insert(Robot::Clay, Cost::Ore(matches[2]));
            costs.insert(Robot::Obsidian, Cost::OreClay(matches[3], matches[4]));
            costs.insert(Robot::Geode, Cost::OreObsidian(matches[5], matches[6]));

            Blueprint {
                id: matches[0],
                costs,
            }
        })
        .collect();

    let mut ans = 0;
    for blueprint in blueprints.iter() {
        let max = backtracking(blueprint, State::new(blueprint));
        println!("{max}");
        ans += blueprint.id * max;
    }

    println!("{ans}");
}
