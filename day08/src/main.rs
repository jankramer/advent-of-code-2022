use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), (21, 8));

    let (a, b) = solve(INPUT);
    println!("Part A: {}", a);
    println!("Part A: {}", b);
}

fn solve(input: &str) -> (i32, i32) {
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let width = input[0].len() as i32;
    let height = input.len() as i32;

    let on_border = |p: (i32, i32)| p.0 == 0 || p.1 == 0 || p.0 == width - 1 || p.1 == height - 1;

    let dirs: Vec<(i32, i32)> = vec![
        (0, -1), // north
        (1, 0),  // east
        (0, 1),  // south
        (-1, 0), // west
    ];

    let mut num_visible = 0;
    for (y, xs) in input.iter().enumerate() {
        for (x, h) in xs.iter().enumerate() {
            let mut visible = false;

            for dir in &dirs {
                let mut pos = (x as i32, y as i32);
                let mut max: i32 = *h;

                loop {
                    if on_border(pos) {
                        visible = true;
                        break;
                    }

                    pos.0 += dir.0;
                    pos.1 += dir.1;

                    if input[pos.1 as usize][pos.0 as usize] >= max {
                        break;
                    }

                    max = *h;
                }

                if visible {
                    break;
                }
            }

            if visible {
                num_visible += 1;
            }
        }
    }

    let mut max_score = 0;
    for (y, xs) in input.iter().enumerate() {
        for (x, h) in xs.iter().enumerate() {
            let mut scores: Vec<_> = vec![];
            for dir in &dirs {
                let mut pos = (x as i32, y as i32);
                let mut max: i32 = *h;
                let mut num_trees_visible = 0;

                loop {
                    if on_border(pos) {
                        break;
                    }

                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    num_trees_visible += 1;

                    if input[pos.1 as usize][pos.0 as usize] >= max {
                        break;
                    }

                    max = *h;
                }
                scores.push(num_trees_visible);
            }

            let score = scores.into_iter().reduce(|a, b| a * b).unwrap();
            if score > max_score {
                max_score = score;
            }
        }
    }

    (num_visible, max_score)
}
