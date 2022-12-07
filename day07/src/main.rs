use itertools::Itertools;
use std::collections::BTreeMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), (95437, 24933642));
    let (a, b) = solve(INPUT);

    println!("Part A: {}", a);
    println!("Part B: {}", b);
}

fn solve(input: &str) -> (usize, usize) {
    let fs = parse(input);
    let mut fs_b: BTreeMap<String, usize> = BTreeMap::new();

    let mut total_a = 0;
    let mut total_b = 0;
    for (dir, size) in &fs {
        total_b += size;
        let mut dir_size = *size;
        for (dir2, size2) in &fs {
            if dir2.starts_with(&format!("{}/", dir)) {
                dir_size += size2;
            }
        }

        if dir_size <= 100000 {
            total_a += dir_size;
        }

        fs_b.insert(dir.to_string(), dir_size);
    }

    let mut min = usize::MAX;
    let unused = 70000000 - total_b;
    let desired = 30000000;

    for (_dir, size) in fs_b {
        if unused + size > desired && size < min {
            min = size;
        }
    }

    (total_a, min)
}

fn parse(input: &str) -> BTreeMap<String, usize> {
    let mut stack: Vec<String> = vec![];
    let mut fs: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    fs.insert("".to_string(), BTreeMap::new());

    for line in input.lines().filter_map(|l| l.parse::<Line>().ok()) {
        match line {
            Line::Cd(dir) => {
                if dir == ".." {
                    stack.pop();
                } else {
                    stack.push(dir.trim_start_matches('/').to_string())
                }
            }
            Line::Dir(name) => {
                let mut dir = stack.clone();
                dir.push(name);
                fs.insert(dir.join("/"), BTreeMap::new());
            }
            Line::File(size, name) => {
                fs.entry(stack.join("/")).and_modify(|x| {
                    x.insert(name, size);
                });
            }
        }
    }

    fs.into_iter()
        .map(|(key, files)| (key, files.values().sum()))
        .collect::<BTreeMap<String, usize>>()
}

enum Line {
    Cd(String),
    Dir(String),
    File(usize, String),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();

        if s.starts_with("$ cd") {
            return Ok(Line::Cd(parts[2].to_string()));
        }

        if s.starts_with("dir ") {
            return Ok(Line::Dir(parts[1].to_string()));
        }

        if let Ok(size) = parts[0].parse::<usize>() {
            return Ok(Line::File(size, parts[1].to_string()));
        }

        Err(())
    }
}
