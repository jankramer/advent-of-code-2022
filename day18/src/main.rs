use parse_display::{Display, FromStr};
use std::collections::BTreeSet;
use std::ops::{Add, Mul};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve_a("1,1,1\n2,1,1"), 10);
    assert_eq!(solve_a(INPUT_TEST), 64);
    println!("Part A: {}", solve_a(INPUT));

    assert_eq!(solve_b(INPUT_TEST), 58);
    println!("Test input passes");
    println!("Part B: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let points: BTreeSet<Point> = input.lines().filter_map(|l| l.parse().ok()).collect();

    points.iter().map(|p| (&p.sides() - &points).len()).sum()
}

fn solve_b(input: &str) -> usize {
    let points: BTreeSet<Point> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let min = Point::new(
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
        points.iter().map(|p| p.z).min().unwrap(),
    );

    let max = Point::new(
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
        points.iter().map(|p| p.z).max().unwrap(),
    );

    let mut open_sides: BTreeSet<Point> = BTreeSet::new();
    let mut open_sides_count = 0;
    for point in &points {
        for side in &point.sides() - &points {
            if open_sides.contains(&side) {
                open_sides_count += 1;
                continue;
            }

            let mut queue: Vec<Point> = vec![side.clone()];
            let mut seen: BTreeSet<Point> = BTreeSet::new();
            seen.insert(point.clone());

            while let Some(cube) = queue.pop() {
                if cube.x < min.x
                    || cube.y < min.y
                    || cube.z < min.z
                    || cube.x > max.x
                    || cube.y > max.y
                    || cube.z > max.z
                {
                    open_sides.insert(side);
                    open_sides_count += 1;
                    break;
                }

                for neighbour in cube.sides() {
                    if seen.contains(&neighbour) {
                        continue;
                    }

                    if points.contains(&neighbour) {
                        seen.insert(neighbour.clone());
                        continue;
                    }

                    seen.insert(neighbour.clone());
                    queue.push(neighbour.clone());
                }
            }
        }
    }

    open_sides_count
}

#[derive(Display, FromStr, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[display("{x},{y},{z}")]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

const DIRECTIONS: &[Point] = &[
    Point { x: 0, y: 0, z: -1 },
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
];

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    fn sides(&self) -> BTreeSet<Point> {
        DIRECTIONS.iter().map(|p| self + p).collect()
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<&Point> for isize {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
