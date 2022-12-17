use itertools::Itertools;
use std::iter::repeat;
use std::ops::Add;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), (3068, 1514285714288));

    let (a, b) = solve(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

fn solve(input: &str) -> (usize, usize) {
    let max_heights = solve_a(input, 10000);

    (max_heights[2021], solve_b(&max_heights, 1000000000000 - 1))
}

fn solve_a(input: &str, n_iterations: usize) -> Vec<usize> {
    let width = 7;

    let jets = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            _ => None,
        })
        .collect_vec();

    let mut jets = jets.into_iter().cycle();

    let patterns = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut chamber = vec![repeat(true).take(width + 1).collect_vec()];
    let mut max_lengths = vec![];

    for rock in patterns.iter().cycle().take(n_iterations) {
        let mut x = 3;
        let mut y = chamber.len() + 3;

        loop {
            let direction = jets.next().unwrap();
            if !collides(&chamber, rock, direction + x, y) {
                x = direction + x;
            }

            if !collides(&chamber, rock, x, y - 1) {
                y -= 1;
            } else {
                add_to_chamber(&mut chamber, rock, x, y);

                break;
            }
        }

        max_lengths.push(chamber.len() - 1);
    }

    max_lengths
}

fn solve_b(max_heights: &[usize], x: usize) -> usize {
    let (period, delta) = (1..10000)
        .map(|period| {
            (
                period,
                max_heights
                    .iter()
                    .skip(1000)
                    .step_by(period)
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .take(10)
                    .collect_vec(),
            )
        })
        .find(|(_period, xs)| xs.iter().all_equal())
        .map(|(period, xs)| (period, xs[0]))
        .expect("no period found, try a higher ");

    let ref_index = 99;
    let ref_value = max_heights[ref_index];

    let period_val = delta * ((x - ref_index) / period);
    let offset = (x - ref_index) % period;
    let offset_val = max_heights[ref_index + offset] - max_heights[ref_index];

    ref_value + period_val + offset_val
}

fn collides(chamber: &[Vec<bool>], pattern: &[Vec<bool>], x: usize, y: usize) -> bool {
    if x < 1 || x + pattern[0].len() > 8 {
        return true;
    }

    for (p_y, row) in pattern.iter().enumerate() {
        for (p_x, lit) in row.iter().enumerate() {
            if *lit
                && *chamber
                    .get(y + p_y)
                    .and_then(|row| row.get(x + p_x))
                    .unwrap_or(&false)
            {
                return true;
            }
        }
    }

    false
}

fn add_to_chamber(chamber: &mut Vec<Vec<bool>>, pattern: &[Vec<bool>], x: usize, y: usize) {
    for (p_y, row) in pattern.iter().enumerate() {
        if chamber.len() < y + p_y + 1 {
            chamber.push(chamber[0].clone().iter().map(|_| false).collect_vec());
        }

        for (p_x, lit) in row.iter().enumerate() {
            chamber[y + p_y][x + p_x] = chamber[y + p_y][x + p_x] || *lit;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Add<usize> for Jet {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Jet::Left => rhs - 1,
            Jet::Right => rhs + 1,
        }
    }
}

#[allow(dead_code)]
fn draw(chamber: &[Vec<bool>], rock: &[Vec<bool>], rock_x: usize, rock_y: usize) {
    print!("{}[2J", 27 as char);
    let max_y = chamber.len() + rock.len() + 3;
    for y in (0..max_y).rev() {
        print!("{:04} ", y);
        for x in 0..8 {
            if y == 0 {
                if x == 0 {
                    print!("+");
                } else {
                    print!("-");
                }
                continue;
            }

            let contains_rock = if y >= rock_y && x >= rock_x {
                rock.get(y - rock_y).and_then(|row| row.get(x - rock_x))
            } else {
                None
            };

            print!(
                "{}",
                match (x, contains_rock, chamber.get(y).and_then(|row| row.get(x))) {
                    (0, _, _) => '|',
                    (_, Some(true), _) => '@',
                    (_, _, Some(true)) => '#',
                    _ => '.',
                }
            );
        }
        if y == max_y {
            println!("+");
        } else {
            println!("|");
        }
    }
    println!();
}
