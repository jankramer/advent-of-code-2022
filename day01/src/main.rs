use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(run(INPUT_TEST), (24000, 45000));
    let (a, b) = run(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

pub fn run(input: &str) -> (i32, i32) {
    let elves: Vec<i32> = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .collect();

    (elves[0], elves[0..3].iter().sum())
}
