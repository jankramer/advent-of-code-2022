use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::iter::repeat;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve_a(INPUT_TEST), Some(31));
    println!("Part A: {:#?}", solve_a(INPUT));

    assert_eq!(solve_b(INPUT_TEST), Some(29));
    println!("Part B: {:#?}", solve_b(INPUT));
}

fn solve_a(input: &str) -> Option<usize> {
    let chars = input.lines().flat_map(|l| l.chars()).collect_vec();
    let from = chars.iter().position(|&c| c == 'S').unwrap();
    let to = chars.iter().position(|&c| c == 'E').unwrap();

    let grid = parse(input);

    bfs(
        from,
        grid.clone(),
        |index, _| index == to,
        |node| {
            grid.neighbours(node)
                .iter()
                .filter(|(_, h)| *h <= grid.values[node] + 1)
                .map(|x| x.0)
                .collect_vec()
        },
    )
    .map(|path| path.len())
}

fn solve_b(input: &str) -> Option<usize> {
    let chars = input.lines().flat_map(|l| l.chars()).collect_vec();
    let from = chars.iter().position(|&c| c == 'E').unwrap();

    let grid = parse(input);

    bfs(
        from,
        grid.clone(),
        |_, height| height == 1,
        |node| {
            grid.neighbours(node)
                .iter()
                .filter(|(_, h)| *h >= grid.values[node] - 1)
                .map(|x| x.0)
                .collect_vec()
        },
    )
    .map(|path| path.len())
}

fn bfs<T, GoalFn, NeighboursFn>(
    start: usize,
    grid: Grid<T>,
    goal: GoalFn,
    neighbours: NeighboursFn,
) -> Option<Vec<usize>>
where
    T: Copy,
    GoalFn: Fn(usize, T) -> bool,
    NeighboursFn: Fn(usize) -> Vec<usize>,
{
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut trace: Vec<usize> = repeat(usize::MAX).take(grid.values.len()).collect();
    let mut seen: HashSet<usize> = HashSet::new();

    queue.push_back(start);
    seen.insert(start);

    let mut current_node: usize;
    loop {
        if let Some(node) = queue.pop_front() {
            current_node = node;
            if goal(node, grid.values[node]) {
                break;
            }

            for index in neighbours(node) {
                if seen.contains(&index) {
                    continue;
                }

                trace[index] = node;
                queue.push_back(index);
                seen.insert(index);
            }
        } else {
            return None;
        }
    }

    let mut index = current_node;
    let mut path = vec![index];
    while trace[index] != start {
        index = trace[index];
        path.push(index);
    }

    path.reverse();

    Some(path)
}

#[derive(Clone)]
struct Grid<T> {
    values: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn neighbours(&self, i: usize) -> Vec<(usize, T)>
    where
        T: Clone,
    {
        [
            if i == 0 { None } else { Some(i - 1) },
            if i > self.values.len() - 2 {
                None
            } else {
                Some(i + 1)
            },
            if i < self.width {
                None
            } else {
                Some(i - self.width)
            },
            if i > self.values.len() - self.width - 1 {
                None
            } else {
                Some(i + self.width)
            },
        ]
        .into_iter()
        .filter_map(|j| j.map(|k| (k, self.values[k].clone())))
        .collect()
    }
}

fn parse(input: &str) -> Grid<isize> {
    Grid {
        values: input
            .lines()
            .flat_map(|l| l.chars().map(elevation))
            .collect_vec(),
        width: input.lines().next().unwrap().len(),
    }
}

fn elevation(c: char) -> isize {
    match c {
        'S' => 1,
        'E' => 26,
        _ => c as isize - 96,
    }
}
