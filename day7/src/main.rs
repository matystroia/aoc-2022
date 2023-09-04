use std::collections::HashMap;

#[derive(Debug)]
enum Object {
    File(i64, String),
    Directory(String),
}

fn main() {
    let mut tree: HashMap<String, Vec<Object>> = HashMap::new();
    let mut cd = String::from("/");

    let mut lines = include_str!("input.txt").lines();

    let mut line = lines.next().unwrap();
    'a: loop {
        let values: Vec<_> = line.split(' ').collect();

        let command = values[1];
        let arg = if command == "cd" { values[2] } else { "" };

        let mut output: Vec<&str> = Vec::new();

        for ln in lines.by_ref() {
            line = ln;
            if ln.starts_with('$') {
                match command {
                    "cd" => cd = parse_cd(&cd, arg),
                    "ls" => parse_ls(&mut tree, &cd, &output),
                    _ => (),
                }
                continue 'a;
            } else {
                output.push(line);
            }
        }
        if command == "ls" {
            parse_ls(&mut tree, &cd, &output)
        }
        break;
    }

    // Part 1
    let _ans: u128 = tree
        .keys()
        .map(|k| dir_size(&tree, k) as u128)
        .filter(|&size| size <= 100_000)
        .sum();

    // Part 2
    let free_space = 70_000_000 - dir_size(&tree, &String::from("/"));
    let delta = 30_000_000 - free_space;

    let target_dir = tree
        .keys()
        .map(|k| dir_size(&tree, k))
        .filter(|sz| sz.ge(&delta))
        .min()
        .unwrap();

    println!("{target_dir}");
}

fn parse_cd(cd: &str, dir: &str) -> String {
    let mut values = vec![""];
    values.extend(cd.split('/').filter(|x| !x.is_empty()));

    match dir {
        "/" => String::from("/"),
        ".." => {
            values.pop();
            values.join("/")
        }
        _ => {
            values.push(dir);
            values.join("/")
        }
    }
}

fn parse_ls(tree: &mut HashMap<String, Vec<Object>>, dir: &String, lines: &[&str]) {
    let objects = lines
        .iter()
        .map(|ln| {
            let values: Vec<_> = ln.split(' ').collect();
            if values[0] == "dir" {
                Object::Directory(String::from(values[1]))
            } else {
                Object::File(values[0].parse().unwrap(), String::from(values[1]))
            }
        })
        .collect();

    tree.insert(dir.to_owned(), objects);
}

fn dir_size(tree: &HashMap<String, Vec<Object>>, dir: &String) -> i64 {
    tree.get(dir)
        .unwrap()
        .iter()
        .map(|o| match o {
            Object::File(size, _) => *size,
            Object::Directory(d) => {
                dir_size(tree, &format!("{}/{d}", if dir == "/" { "" } else { dir }))
            }
        })
        .sum()
}
