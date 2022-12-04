type Pair = (Range, Range);
type Range = (usize, usize);

pub fn run(input: &str) -> (usize, usize) {
    (
        input.lines().map(parse).filter(contains_a).count(),
        input.lines().map(parse).filter(contains_b).count(),
    )
}

fn contains_a(pair: &Pair) -> bool {
    let ((left_min, left_max), (right_min, right_max)) = pair;

    left_min <= right_min && left_max >= right_max || left_min >= right_min && left_max <= right_max
}

fn contains_b(pair: &Pair) -> bool {
    let ((left_min, left_max), (right_min, right_max)) = pair;

    (left_min >= right_min && left_min <= right_max)
        || (right_min >= left_min && right_min <= left_max)
}

fn parse(line: &str) -> Pair {
    let (left_pair, right_pair) = line.split_once(',').unwrap();

    (parse_pair(left_pair), parse_pair(right_pair))
}

fn parse_pair(pair: &str) -> Range {
    let (left, right) = pair.split_once('-').unwrap();

    (left.parse().unwrap(), right.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("2-3,4-5"), ((2, 3), (4, 5)));
    }

    #[test]
    fn example() {
        assert_eq!(run(INPUT), (2, 4));
    }

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}
