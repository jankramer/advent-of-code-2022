use itertools::Itertools;

pub fn run(input: &str) -> (i32, i32) {
    let elves: Vec<i32> = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .collect();

    (elves[0], elves[0..3].iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run(INPUT), (24000, 45000));
    }

    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
}
