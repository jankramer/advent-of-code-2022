use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST, 1, 1), 3);
    println!("Part A: {}", solve(INPUT, 1, 1));

    assert_eq!(solve(INPUT_TEST, 811589153, 10), 1623178306);
    println!("Part B: {}", solve(INPUT, 811589153, 10));
}

fn solve(input: &str, key: isize, rounds: usize) -> isize {
    let mut numbers: Vec<(usize, isize)> = input
        .lines()
        .filter_map(|l| l.parse::<isize>().ok())
        .map(|x| x * key)
        .enumerate()
        .collect();

    let len = numbers.len();
    let len1 = (len - 1) as isize;

    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let current_pos = numbers.iter().position(|&(index, _)| index == i).unwrap();
            let item = numbers.remove(current_pos);
            let new_pos = (((current_pos as isize + item.1) % len1) + len1) % len1;

            numbers.insert(new_pos as usize, item);
        }
    }

    let numbers = numbers.iter().map(|&x| x.1).collect_vec();
    let pos_0 = numbers.iter().position(|&x| x == 0).unwrap();

    numbers[(pos_0 + 1000) % len as usize]
        + numbers[(pos_0 + 2000) % len as usize]
        + numbers[(pos_0 + 3000) % len as usize]
}
