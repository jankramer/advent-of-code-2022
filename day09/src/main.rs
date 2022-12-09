use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");
const INPUT_TEST_B: &str = include_str!("input.test.b.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST, 2), 13);
    println!("Part A: {}", solve(INPUT, 2));

    assert_eq!(solve(INPUT_TEST_B, 10), 36);
    println!("Part B: {}", solve(INPUT, 10));
}

fn solve(input: &str, n: usize) -> usize {
    let mut knots: Vec<Point> = vec![];
    for _ in 0..n {
        knots.push(Point { x: 0, y: 0 });
    }

    let mut seen: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let (dir, length) = line.split_once(' ').unwrap();
        let length = length.parse::<i32>().unwrap();

        for _ in 0..length {
            let mut iterator = knots.iter_mut();
            let mut head = iterator.next().unwrap();
            match dir {
                "R" => {
                    head.x += 1;
                }
                "U" => {
                    head.y -= 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "D" => {
                    head.y += 1;
                }
                _ => {}
            }

            for tail in iterator {
                let dist = manhattan_distance(head, tail);
                let delta_x = head.x - tail.x;
                let delta_y = head.y - tail.y;

                if delta_x > 1 || (dist == 3 && delta_x == 1) {
                    tail.x += 1;
                }

                if delta_y > 1 || (dist == 3 && delta_y == 1) {
                    tail.y += 1;
                }

                if delta_x < -1 || (dist == 3 && delta_x == -1) {
                    tail.x -= 1;
                }

                if delta_y < -1 || (dist == 3 && delta_y == -1) {
                    tail.y -= 1;
                }

                head = tail;
            }

            seen.insert(knots.last().unwrap().clone());
        }
    }

    seen.len()
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}
