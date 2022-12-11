use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST, true, 20), 10605);
    println!("Part A: {}", solve(INPUT, true, 20));

    assert_eq!(solve(INPUT_TEST, false, 10000), 2713310158);
    println!("Part B: {}", solve(INPUT, false, 10000));
}

fn solve(input: &str, divide: bool, n_rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|m| m.parse().unwrap())
        .collect_vec();

    let num_monkeys = monkeys.len();
    let mut inspections: HashMap<usize, usize> = HashMap::new();

    for _ in 1..=n_rounds {
        for i in 0..num_monkeys {
            take_turn(&mut monkeys, i, &mut inspections, divide);
        }
    }

    inspections.values().sorted().rev().take(2).product()
}

fn take_turn(
    monkeys: &mut [Monkey],
    monkey_index: usize,
    inspections: &mut HashMap<usize, usize>,
    divide: bool,
) {
    let mut current_monkey: Option<&mut Monkey> = None;
    let mut next_monkey_true: Option<&mut Monkey> = None;
    let mut next_monkey_false: Option<&mut Monkey> = None;

    let next_true_index = monkeys[monkey_index].next_true;
    let next_false_index = monkeys[monkey_index].next_false;

    let wrap_value: usize = monkeys.iter().map(|m| m.test).product();

    for (i, monkey) in monkeys.iter_mut().enumerate() {
        if i == monkey_index {
            current_monkey = Some(monkey);
        } else if i == next_true_index {
            next_monkey_true = Some(monkey);
        } else if i == next_false_index {
            next_monkey_false = Some(monkey);
        }
    }

    let current_monkey = current_monkey.unwrap();

    let mut true_values = vec![];
    let mut false_values = vec![];

    let num_items = current_monkey.items.len();
    inspections
        .entry(monkey_index)
        .and_modify(|e| *e += num_items)
        .or_insert(num_items);

    for item in current_monkey.items.iter() {
        let operation_value = match current_monkey.op_value {
            OperationValue::Old => *item,
            OperationValue::Fixed(value) => value,
        };

        let worry_level = (match current_monkey.op {
            Operation::Add => *item + operation_value,
            Operation::Multiply => *item * operation_value,
        } / if divide { 3 } else { 1 })
            % wrap_value;

        if worry_level % current_monkey.test == 0 {
            true_values.push(worry_level);
        } else {
            false_values.push(worry_level);
        }
    }

    current_monkey.items = vec![];
    next_monkey_true.unwrap().items.extend(true_values);
    next_monkey_false.unwrap().items.extend(false_values);
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    op_value: OperationValue,
    test: usize,
    next_true: usize,
    next_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        let (_, items) = lines[1].split_once(": ").unwrap();
        let items = items
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();

        let (_, op) = lines[2].split_once(": new = old ").unwrap();
        let (op, op_value) = op.split_once(' ').unwrap();

        let test = lines[3].split_once("divisible by ").unwrap().1;
        let next_true = lines[4].split_once("monkey ").unwrap().1;
        let next_false = lines[5].split_once("monkey ").unwrap().1;

        Ok(Monkey {
            items,
            op: match op {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => {
                    panic!("invalid op");
                }
            },
            op_value: if op_value == "old" {
                OperationValue::Old
            } else {
                OperationValue::Fixed(op_value.parse::<usize>().unwrap())
            },
            test: test.parse::<usize>().unwrap(),
            next_true: next_true.parse().unwrap(),
            next_false: next_false.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
enum OperationValue {
    Old,
    Fixed(usize),
}
