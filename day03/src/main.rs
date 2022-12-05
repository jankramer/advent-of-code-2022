use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(run(INPUT_TEST), (157, 70));
    let (a, b) = run(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

pub fn run(input: &str) -> (usize, usize) {
    (
        input.lines().map(|l| priority(common_item_a(l))).sum(),
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|lines| priority(common_item_b(lines.collect())))
            .sum(),
    )
}

fn common_item_a(x: &str) -> char {
    let (left, right) = x.split_at(x.len() / 2);

    left.chars().find(|c| right.chars().contains(c)).unwrap()
}

fn common_item_b(xs: Vec<&str>) -> char {
    xs.first()
        .unwrap()
        .chars()
        .find(|c| xs.iter().dropping(1).all(|y| y.chars().contains(c)))
        .unwrap()
}

fn priority(x: char) -> usize {
    let char_code = x as usize;
    match char_code {
        65..=91 => char_code - 38,
        97..=122 => char_code - 96,
        _ => 0,
    }
}
