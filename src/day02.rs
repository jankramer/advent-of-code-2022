pub fn run(input: &str) -> (i32, i32) {
    let a = input
        .lines()
        .map(|l| l.split(" ").collect())
        .map(|l: Vec<&str>| round_a(l[0], l[1]))
        .sum::<i32>();

    let b = input
        .lines()
        .map(|l| l.split(" ").collect())
        .map(|l: Vec<&str>| round_b(l[0], l[1]))
        .sum::<i32>();

    (a, b)
}

fn round_a(a: &str, b: &str) -> i32 {
    let shape_score = match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let match_score = match a {
        "A" => match b {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => 0,
        },
        "B" => match b {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        },
        "C" => match b {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => 0,
        },
        _ => 0,
    };

    shape_score + match_score
}

fn round_b(a: &str, b: &str) -> i32 {
    match a {
        // Rock
        "A" => match b {
            // Lose -> Scissor
            "X" => 3 + 0,
            // Draw -> Rock
            "Y" => 1 + 3,
            // Win -> Paper
            "Z" => 2 + 6,
            _ => 0,
        },
        // Paper
        "B" => match b {
            // Lose -> Rock
            "X" => 1 + 0,
            // Draw -> Paper
            "Y" => 2 + 3,
            // Win -> Scissor
            "Z" => 3 + 6,
            _ => 0,
        },
        // Scissor
        "C" => match b {
            // Lose -> Paper
            "X" => 2 + 0,
            // Draw -> Scissor
            "Y" => 3 + 3,
            // Win -> Rock
            "Z" => 1 + 6,
            _ => 0,
        },
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
