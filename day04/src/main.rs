use parse_display::{Display, FromStr};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

#[derive(Display, FromStr)]
#[display("{0}-{1},{2}-{3}")]
struct Input(usize, usize, usize, usize);

fn main() {
    assert_eq!(run(INPUT_TEST), (2, 4));
    let (a, b) = run(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

pub fn run(input: &str) -> (usize, usize) {
    let input: Vec<Input> = input.lines().filter_map(|l| l.parse().ok()).collect();

    (
        input.iter().filter(|x| contains_a(x)).count(),
        input.iter().filter(|x| contains_b(x)).count(),
    )
}

fn contains_a(Input(left_min, left_max, right_min, right_max): &Input) -> bool {
    left_min <= right_min && left_max >= right_max || left_min >= right_min && left_max <= right_max
}

fn contains_b(Input(left_min, left_max, right_min, right_max): &Input) -> bool {
    (left_min >= right_min && left_min <= right_max)
        || (right_min >= left_min && right_min <= left_max)
}
