fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();

    let mut stacks: [Vec<char>; 9] = Default::default();

    for line in lines.iter().take(8) {
        for i in 0..9 {
            let letter: &char = &line[4 * i + 1..4 * i + 2].chars().next().unwrap();
            if letter != &' ' {
                stacks[i].insert(0, *letter)
            }
        }
    }

    for line in lines.iter().skip(10) {
        let values: Vec<_> = line.split(' ').collect();
        let (n, from, to): (usize, usize, usize) = (
            values[1].parse::<usize>().unwrap(),
            values[3].parse::<usize>().unwrap() - 1,
            values[5].parse::<usize>().unwrap() - 1,
        );

        let from_stack = &mut stacks[from];
        let chars: Vec<_> = from_stack.drain(from_stack.len() - n..).collect();
        stacks[to].extend(chars);

        // for _ in 0..n {
        //     let c = stacks[from as usize].pop().unwrap();
        //     stacks[to as usize].push(c);
        // }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
}
