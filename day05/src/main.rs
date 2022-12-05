use itertools::Itertools;
use parse_display::{Display, FromStr};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(run(INPUT_TEST), ("CMZ".to_string(), "MCD".to_string()));
    let (a, b) = run(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

pub fn run(input: &str) -> (String, String) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let moves: Vec<Move> = moves.lines().filter_map(|l| l.parse().ok()).collect();
    let stacks: Vec<Vec<char>> = parse_stacks(stacks);

    (
        answer(stacks.clone(), &moves, true),
        answer(stacks, &moves, false),
    )
}

fn answer(stacks: Vec<Vec<char>>, moves: &[Move], reverse: bool) -> String {
    moves
        .iter()
        .fold(stacks, |stacks, m| m.apply(stacks, reverse))
        .iter()
        .filter_map(|v| v.last())
        .join("")
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .rev()
        .skip(1)
        .map(|l| l.chars().skip(1).step_by(4).collect())
        .collect::<Vec<Vec<char>>>()
        .transpose()
        .iter()
        .map(|row| {
            row.iter()
                .filter_map(|&f| match f {
                    ' ' => None,
                    _ => Some(f),
                })
                .collect()
        })
        .collect()
}

#[derive(Display, FromStr)]
#[display("move {qty} from {from} to {to}")]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn apply(&self, mut stacks: Vec<Vec<char>>, reverse: bool) -> Vec<Vec<char>> {
        let lift_from = stacks[self.from - 1].len() - self.qty;
        let mut lift_items: Vec<char> = stacks[self.from - 1].drain(lift_from..).collect();

        if reverse {
            lift_items.reverse();
        }

        stacks[self.to - 1].extend(lift_items);

        stacks
    }
}

trait Matrix<T> {
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T> Matrix<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn transpose(&self) -> Vec<Vec<T>> {
        (0..self[0].len())
            .map(|col| self.iter().map(|row| row[col]).collect())
            .collect()
    }
}
