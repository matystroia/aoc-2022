use std::cmp::Ordering;

#[derive(Debug)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

impl Value {
    fn find_matching(s: &str, i: usize) -> usize {
        let mut k = 1;
        let mut j = i + 1;
        let chars: Vec<_> = s.chars().collect();
        while j < s.len() {
            match chars[j] {
                '[' => k += 1,
                ']' => k -= 1,
                _ => (),
            }
            if k == 0 {
                return j;
            }
            j += 1;
        }
        panic!();
    }

    fn parse_list(s: &str) -> Value {
        let chars: Vec<_> = s.chars().collect();
        let mut values: Vec<Value> = Vec::new();

        assert_eq!(*chars.first().unwrap(), '[');
        assert_eq!(*chars.last().unwrap(), ']');

        let mut i = 1;
        while i < s.len() - 1 {
            if chars[i] == '[' {
                let j = Self::find_matching(s, i);
                values.push(Self::parse_list(&s[i..=j]));
                i = j + 1;
            } else if chars[i].is_ascii_digit() {
                let mut j = i + 1;
                while chars[j].is_ascii_digit() {
                    j += 1;
                }
                values.push(Self::parse_integer(&s[i..j]));
                i = j;
            } else {
                i += 1;
            }
        }

        Value::List(values)
    }

    fn parse_integer(s: &str) -> Value {
        Value::Integer(s.chars().collect::<String>().parse::<i32>().unwrap())
    }
}

fn compare(left: &Value, right: &Value) -> Ordering {
    if let (Value::Integer(left), Value::Integer(right)) = (left, right) {
        left.cmp(right)
    } else if let (Value::List(left), Value::List(right)) = (left, right) {
        for i in 0..(left.len().min(right.len())) {
            if compare(&left[i], &right[i]) != Ordering::Equal {
                return compare(&left[i], &right[i]);
            }
        }
        left.len().cmp(&right.len())
    } else if let (Value::Integer(left), _) = (left, right) {
        compare(&Value::List(vec![Value::Integer(*left)]), right)
    } else if let (_, Value::Integer(right)) = (left, right) {
        compare(left, &Value::List(vec![Value::Integer(*right)]))
    } else {
        panic!("Oops");
    }
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt")
        .lines()
        .filter(|&ln| !ln.is_empty())
        .collect();

    let mut values: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, &ln)| (i, Value::parse_list(ln)))
        .collect();

    values.sort_by(|(_, a), (_, b)| compare(a, b));

    let p1 = values.iter().position(|(i, _)| *i == 0).unwrap() + 1;
    let p2 = values.iter().position(|(i, _)| *i == 1).unwrap() + 1;

    println!("{}", p1 * p2)
}
