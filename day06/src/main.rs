use itertools::Itertools;
use std::iter::Iterator;

fn main() {
    let input: Vec<char> = include_str!("input.txt").chars().collect();

    println!("Part A: {}", solve(&input, 4));
    println!("Part B: {}", solve(&input, 14));
}

fn solve(chars: &[char], n: usize) -> usize {
    chars
        .windows(n)
        .position(|xs| xs.iter().all_unique())
        .unwrap()
        + n
}
