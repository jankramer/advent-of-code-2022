use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde_json::Value::{Array, Number};
use serde_json::{json, Value};
use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve_a(INPUT_TEST), 13);
    println!("Part A: {}", solve_a(INPUT));

    assert_eq!(solve_b(INPUT_TEST), 140);
    println!("Part A: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let pairs: Vec<Vec<Value>> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| serde_json::from_str(l).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut sum_indices = 0;
    for (i, _) in pairs.iter().enumerate() {
        if compare(&pairs[i][0].clone(), &pairs[i][1].clone()) != Ordering::Greater {
            sum_indices += i + 1;
        }
    }

    sum_indices
}

fn solve_b(input: &str) -> usize {
    let packets: Vec<Value> = format!("{}\n[[2]]\n[[6]]\n", input)
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .sorted_by(compare)
        .collect_vec();

    let magic_a = packets.iter().position(|p| p == &json![[[2]]]).unwrap();
    let magic_b = packets.iter().position(|p| p == &json![[[6]]]).unwrap();

    (magic_a + 1) * (magic_b + 1)
}

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Number(a), Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (Array(_), Number(b)) => compare(left, &json![[b]]),
        (Number(a), Array(_)) => compare(&json![[a]], right),
        (Array(a), Array(b)) => {
            for pair in a.iter().zip_longest(b.iter()) {
                let result = match pair {
                    Both(l, r) => compare(l, r),
                    Left(_) => Ordering::Greater,
                    Right(_) => Ordering::Less,
                };

                if result != Ordering::Equal {
                    return result;
                }
            }

            Ordering::Equal
        }
        _ => Ordering::Equal,
    }
}
