use itertools::Itertools;
use parse_display::{Display, FromStr};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve_a(INPUT_TEST, 10), 26);
    println!("Part A: {}", solve_a(INPUT, 2000000));

    assert_eq!(solve_b(INPUT_TEST, 20), 56000011);
    println!("Part B: {}", solve_b(INPUT, 4000000));
}

fn solve_a(input: &str, y: isize) -> usize {
    let sensors: Vec<Sensor> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let lines_at_y = sensors.iter().filter_map(|s| s.line_at_y(y)).collect_vec();
    let merged = Line::merge(lines_at_y);

    merged.iter().map(|l| l.length()).sum()
}

fn solve_b(input: &str, search_space: isize) -> isize {
    let sensors: Vec<Sensor> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let mut x = 0;
    let mut y = 0;
    for y in 0..search_space {
        let lines = Line::merge(sensors.iter().filter_map(|s| s.line_at_y(y)).collect_vec());
        if lines.len() > 1 {
            x = lines[0].b + 1;
        }
    }

    for x in 0..search_space {
        let lines = Line::merge(sensors.iter().filter_map(|s| s.line_at_x(x)).collect_vec());
        if lines.len() > 1 {
            y = lines[0].b + 1;
        }
    }

    x * 4000000 + y
}

#[derive(FromStr, Display, Clone, Debug)]
#[display("Sensor at x={pos.x}, y={pos.y}: closest beacon is at x={beacon.x}, y={beacon.y}")]
struct Sensor {
    #[from_str(default)]
    pos: Point,

    #[from_str(default)]
    beacon: Point,
}

impl Sensor {
    fn beacon_distance(&self) -> isize {
        (self.pos.x - self.beacon.x).abs() + (self.pos.y - self.beacon.y).abs()
    }

    fn line_at_y(&self, y: isize) -> Option<Line> {
        let distance = self.beacon_distance();

        if y > self.pos.y + distance || y < self.pos.y - distance {
            return None;
        }

        let width = distance - (y - self.pos.y).abs();

        Some(Line {
            a: self.pos.x - width,
            b: self.pos.x + width,
        })
    }

    fn line_at_x(&self, x: isize) -> Option<Line> {
        let distance = self.beacon_distance();

        if x > self.pos.x + distance || x < self.pos.x - distance {
            return None;
        }

        let width = distance - (x - self.pos.x).abs();

        Some(Line {
            a: self.pos.y - width,
            b: self.pos.y + width,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Default, Ord, PartialOrd, Display, Copy)]
#[display("{x},{y}")]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
struct Line {
    a: isize,
    b: isize,
}

impl Line {
    fn length(&self) -> usize {
        self.a.abs_diff(self.b)
    }

    fn merge(lines: Vec<Line>) -> Vec<Line> {
        if lines.is_empty() {
            return lines;
        }

        let mut new_lines = vec![];
        let mut queue = lines;

        queue.sort();
        queue.reverse();

        while let Some(mut current) = queue.pop() {
            while let Some(next) = queue.pop() {
                if next.a <= (current.b + 1) {
                    if next.b > current.b {
                        current.b = next.b;
                    }
                    continue;
                } else {
                    queue.push(next);
                    break;
                }
            }

            new_lines.push(current);
        }

        new_lines
    }
}
