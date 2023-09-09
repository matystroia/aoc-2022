use eval::Expr;
use num::integer::lcm;
use std::collections::VecDeque;

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let groups: Vec<_> = lines.split(|&x| x.is_empty()).collect();

    let mut monkeys: Vec<_> = groups.iter().map(|g| Monkey::from(g.to_vec())).collect();
    let lcm = monkeys.iter().fold(1, |acc, e| lcm(acc, e.test));

    for round in 1..=10_000 {
        monkey_round(&mut monkeys, lcm);
    }

    monkeys.sort_by_key(|monkey| u64::MAX - monkey.items_inspected);
    let monkey_business = monkeys[0].items_inspected * monkeys[1].items_inspected;
    println!("{monkey_business}");
}

fn monkey_round(monkeys: &mut Vec<Monkey>, modulo: u64) {
    for i in 0..monkeys.len() {
        while let Some(mut item) = monkeys[i].items.pop_front() {
            let target = monkeys[i].throw_to(&mut item, modulo);
            monkeys[target].items.push_back(item);
        }
    }
}

#[derive(Clone)]
struct Item {
    worry_level: u64,
}
struct Monkey {
    items: VecDeque<Item>,
    operation: String,
    test: u64,
    throw: (usize, usize),

    items_inspected: u64,
}

impl Monkey {
    fn from(lines: Vec<&str>) -> Monkey {
        let _i = lines[0].chars().collect::<Vec<_>>()[7]
            .to_digit(10)
            .unwrap() as i32;

        let starting_items: VecDeque<_> = lines[1]
            .split(' ')
            .skip_while(|s| s.is_empty())
            .collect::<Vec<_>>()[2..]
            .iter()
            .map(|&x| x[..2].parse::<u64>().unwrap())
            .map(|x| Item { worry_level: x })
            .collect();

        let operation = lines[2].split('=').last().unwrap().trim();
        let test = lines[3].split(' ').last().unwrap().parse::<u64>().unwrap();
        let throw1 = lines[4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let throw2 = lines[5]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items: starting_items,
            operation: String::from(operation),
            test,
            throw: (throw1, throw2),
            items_inspected: 0,
        }
    }

    fn throw_to(&mut self, item: &mut Item, modulo: u64) -> usize {
        self.inspect(item, modulo);
        if self.test(item) {
            self.throw.0
        } else {
            self.throw.1
        }
    }

    fn inspect(&mut self, item: &mut Item, modulo: u64) {
        // println!(
        //     "Monkey inspects an item with a worry level of {}",
        //     item.worry_level
        // );
        self.items_inspected += 1;
        let new_worry_level = Expr::new(&self.operation)
            .value("old", item.worry_level)
            .exec()
            .unwrap()
            .as_u64()
            .unwrap();

        item.worry_level = new_worry_level;
        item.worry_level %= modulo;
    }

    fn test(&self, item: &Item) -> bool {
        item.worry_level % self.test == 0
    }
}
