const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(run(INPUT_TEST), (2, 4));
    let (a, b) = run(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

type Pair = (Range, Range);
type Range = (usize, usize);

pub fn run(input: &str) -> (usize, usize) {
    (
        input.lines().map(parse).filter(contains_a).count(),
        input.lines().map(parse).filter(contains_b).count(),
    )
}

fn contains_a(pair: &Pair) -> bool {
    let ((left_min, left_max), (right_min, right_max)) = pair;

    left_min <= right_min && left_max >= right_max || left_min >= right_min && left_max <= right_max
}

fn contains_b(pair: &Pair) -> bool {
    let ((left_min, left_max), (right_min, right_max)) = pair;

    (left_min >= right_min && left_min <= right_max)
        || (right_min >= left_min && right_min <= left_max)
}

fn parse(line: &str) -> Pair {
    let (left_pair, right_pair) = line.split_once(',').unwrap();

    (parse_pair(left_pair), parse_pair(right_pair))
}

fn parse_pair(pair: &str) -> Range {
    let (left, right) = pair.split_once('-').unwrap();

    (left.parse().unwrap(), right.parse().unwrap())
}
