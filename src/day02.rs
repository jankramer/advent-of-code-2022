pub fn run(input: &str) -> (i32, i32) {
    (
        input.lines().map(part_a).sum::<i32>(),
        input.lines().map(part_b).sum::<i32>(),
    )
}

fn part_a(line: &str) -> i32 {
    match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn part_b(line: &str) -> i32 {
    match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run(INPUT), (15, 12));
    }

    const INPUT: &'static str = "A Y
B X
C Z";
}
