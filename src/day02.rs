pub fn run(_input: &str) -> (i32, i32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run(INPUT), (0, 0));
    }

    const INPUT: &'static str = "";
}
