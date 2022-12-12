use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), Some(31));
    println!("Part A: {:#?}", solve(INPUT));

    assert_eq!(solve_b(INPUT_TEST), Some(29));
    println!("Part B: {:#?}", solve_b(INPUT));
}

fn solve(input: &str) -> Option<usize> {
    let map: Map = input.parse().unwrap();

    shortest_path(map)
}

fn solve_b(input: &str) -> Option<usize> {
    let map: Map = input.parse().unwrap();
    let maps: Vec<_> = map
        .elevations
        .iter()
        .filter(|(point, &elevation)| elevation == 1)
        .map(|(point, elevation)| Map {
            start: point.clone(),
            end: map.end.clone(),
            elevations: map.elevations.clone(),
        })
        .collect_vec();

    maps.into_par_iter().filter_map(shortest_path).min()
}

fn shortest_path(map: Map) -> Option<usize> {
    println!(
        "Running shortest path with start: {}, {}",
        map.start.x, map.start.y
    );
    let mut distances: HashMap<Point, usize> = HashMap::new();
    let mut queue: BinaryHeap<(isize, usize, Point)> = BinaryHeap::new();

    distances.insert(Point::new(0, 0), 0);
    queue.push((-1, 0, map.start.clone()));

    while let Some((elevation, steps, point)) = queue.pop() {
        if point == map.end {
            return Some(steps);
        }

        if steps > *distances.get(&point).unwrap_or(&usize::MAX) {
            continue;
        }

        for (height, point) in map.valid_neighbours(point) {
            if steps + 1 < *distances.get(&point).unwrap_or(&usize::MAX) {
                queue.push((-(height as isize), steps + 1, point.clone()));
                distances.insert(point, steps + 1);
            }
        }
    }

    None
}

#[derive(Debug)]
struct Map {
    start: Point,
    end: Point,
    elevations: HashMap<Point, usize>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elevations: HashMap<Point, usize> = HashMap::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let elevation = match char {
                    'S' => 'a' as usize,
                    'E' => 'z' as usize,
                    _ => char as usize,
                } - 96;

                let point = Point::new(x as i32, y as i32);
                elevations.insert(point.clone(), elevation);

                match char {
                    'S' => {
                        start = point;
                    }
                    'E' => {
                        end = point;
                    }
                    _ => {}
                }
            }
        }

        Ok(Map {
            start,
            end,
            elevations,
        })
    }
}

impl Map {
    pub fn valid_neighbours(&self, point: Point) -> Vec<(usize, Point)> {
        let current_elevation = *self.elevations.get(&point).unwrap();

        return [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ]
        .iter()
        .map(|x| x.add(&point))
        .filter_map(|x| {
            self.elevations
                .get(&x)
                .filter(|&&x| x <= current_elevation + 1)
                .map(|&h| (h, x))
        })
        .collect_vec();
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
