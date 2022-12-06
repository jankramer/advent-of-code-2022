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
        .find_position(|xs| xs.iter().all_unique())
        .map(|(start_pos, _)| start_pos + n)
        .unwrap()
}
