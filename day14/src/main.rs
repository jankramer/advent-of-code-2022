use itertools::Itertools;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST, false), 24);
    println!("Part A: {}", solve(INPUT, false));

    assert_eq!(solve(INPUT_TEST, true), 93);
    println!("Part B: {}", solve(INPUT, true));
}

fn solve(input: &str, part_b: bool) -> isize {
    let lines: Vec<Vec<(isize, isize)>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').unwrap();

                    (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
                })
                .collect_vec()
        })
        .collect_vec();

    let mut grid: BTreeMap<(isize, isize), bool> = BTreeMap::new();

    for points in lines {
        for point in points
            .into_iter()
            .tuple_windows()
            .flat_map(|(a, b)| points_between(a, b))
        {
            grid.insert(point, true);
        }
    }
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();
    let mut units = 0;
    while drop_sand(&mut grid, max_y, part_b) {
        units += 1;
    }

    units
}

fn drop_sand(grid: &mut BTreeMap<(isize, isize), bool>, max_y: isize, part_b: bool) -> bool {
    if grid.get(&(500, 0)).is_some() {
        return false;
    }

    let mut pos = (500, 0);
    let down = (0, 1);
    let down_left = (-1, 1);
    let down_right = (1, 1);

    loop {
        if part_b && pos.1 + 1 == max_y + 2 {
            grid.insert(pos, true);
            return true;
        }

        match (
            grid.get(&add(pos, down)),
            grid.get(&add(pos, down_left)),
            grid.get(&add(pos, down_right)),
        ) {
            (None, _, _) => {
                pos = add(pos, down);

                if pos.1 > max_y + 3 {
                    return false;
                }
            }
            (Some(_), None, _) => {
                pos = add(pos, down_left);
            }
            (Some(_), Some(_), None) => {
                pos = add(pos, down_right);
            }
            (Some(_), Some(_), Some(_)) => {
                grid.insert(pos, true);
                return true;
            }
        }
    }
}

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn points_between(a: (isize, isize), b: (isize, isize)) -> Vec<(isize, isize)> {
    let dir = direction(a, b);
    let mut points: Vec<(isize, isize)> = vec![a];
    let mut pos = a;
    while pos != b {
        pos = add(pos, dir);
        points.push(pos);
    }

    points
}

fn direction(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    (clamp(dx, -1, 1), clamp(dy, -1, 1))
}

fn clamp(x: isize, min: isize, max: isize) -> isize {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
