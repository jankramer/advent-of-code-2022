use itertools::Itertools;

#[derive(PartialEq, Debug, Clone)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct State {
    stacks: Vec<Vec<char>>,
}

pub fn run(input: &str) -> (String, String) {
    let (state, moves) = parse(input);

    (part_a(state.clone(), moves.clone()), part_b(state, moves))
}

fn part_a(mut state: State, moves: Vec<Move>) -> String {
    for m in moves {
        for _i in 1..=m.quantity {
            let from = state.stacks[m.from - 1].pop();
            state.stacks[m.to - 1].push(from.unwrap());
        }
    }

    state
        .stacks
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .join("")
}

fn part_b(mut state: State, moves: Vec<Move>) -> String {
    for m in moves {
        let mut temp_stack = vec![];
        for _i in 1..=m.quantity {
            let from = state.stacks[m.from - 1].pop();
            temp_stack.push(from.unwrap());
        }

        for _i in 1..=m.quantity {
            state.stacks[m.to - 1].push(temp_stack.pop().unwrap());
        }
    }

    state
        .stacks
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .join("")
}

fn parse(input: &str) -> (State, Vec<Move>) {
    let (state, moves) = input.split_once("\n\n").unwrap();

    (parse_state(state), moves.lines().map(parse_move).collect())
}

fn parse_state(state: &str) -> State {
    State {
        stacks: transpose(state.lines().map(|l| l.chars().collect()).collect())
            .into_iter()
            .filter(|stack| stack[0] != ' ')
            .map(|stack| stack.into_iter().filter(|&c| c != ' ').skip(1).collect())
            .collect(),
    }
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split_whitespace().collect();

    Move {
        quantity: parts[1].parse().unwrap(),
        from: parts[3].parse().unwrap(),
        to: parts[5].parse().unwrap(),
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|inner| inner[i].clone())
                .rev()
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run(INPUT), ("CMZ".to_string(), "MCD".to_string()));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT),
            (
                State {
                    stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
                },
                vec![
                    Move {
                        quantity: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        quantity: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        quantity: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        quantity: 1,
                        from: 1,
                        to: 2
                    },
                ]
            )
        );
    }

    const INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}
