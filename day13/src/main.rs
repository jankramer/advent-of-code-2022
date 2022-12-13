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
    println!("Part B: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    input
        .lines()
        .filter_map(|p| p.parse::<Value>().ok())
        .tuples()
        .enumerate()
        .filter_map(|(pair_index, (left, right))| match compare(&left, &right) {
            Ordering::Less => Some(pair_index + 1),
            _ => None,
        })
        .sum()
}

fn solve_b(input: &str) -> usize {
    let magic = [json![[[2]]], json![[[6]]]];

    input
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .chain(magic.clone())
        .sorted_by(compare)
        .positions(|p| magic.contains(&p))
        .map(|x| x + 1)
        .product()
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
