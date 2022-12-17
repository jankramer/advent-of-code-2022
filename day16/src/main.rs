use itertools::Itertools;
use parse_display::FromStr;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::iter::repeat;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    let test_valves = parse(INPUT_TEST);
    let real_valves = parse(INPUT);

    assert_eq!(solve(&test_valves, 1, 30, 450), 1651);
    println!("Part A: {}", solve(&real_valves, 1, 30, 450));

    assert_eq!(solve(&test_valves, 2, 26, 1000), 1707);
    println!("Part B: {}", solve(&real_valves, 2, 26, 1000));
}

fn parse(input: &str) -> Vec<Valve> {
    let valves_input: Vec<ValveInput> =
        input.lines().sorted().map(|l| l.parse().unwrap()).collect();

    valves_input
        .iter()
        .enumerate()
        .map(|(id, v)| Valve {
            id,
            label: v.id.clone(),
            flow_rate: v.flow_rate,
            leads_to: v
                .leads_to
                .split(", ")
                .map(|x| valves_input.iter().position(|y| y.id == x).unwrap())
                .collect(),
        })
        .collect()
}

fn solve(valves: &[Valve], n_actors: usize, n_minutes: usize, search_space_size: usize) -> usize {
    let open_valves_with_flow: HashSet<usize> = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect();

    let mut volcanos: Vec<Volcano> = Vec::with_capacity(search_space_size);
    volcanos.push(Volcano::new(open_valves_with_flow, n_actors, n_minutes));
    let mut queue: BinaryHeap<Volcano> = BinaryHeap::with_capacity(10 * search_space_size);

    let mut max_value = 0;

    for i in 1..=n_minutes {
        println!("Minute {}, max value {}", i, max_value);

        while let Some(mut volcano) = volcanos.pop() {
            volcano.tick(valves, &mut queue);
        }

        let mut j = 0;
        while let Some(volcano) = queue.pop() {
            let current_value = volcano.value();
            if current_value > max_value {
                max_value = current_value;
            }

            if volcano.remaining_valves.is_empty() {
                continue;
            }

            volcanos.push(volcano);

            j += 1;
            if j >= search_space_size {
                break;
            }
        }
    }

    max_value
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Volcano {
    remaining_valves: HashSet<usize>,
    current_valves: Vec<usize>,
    prev_valve: Option<usize>,
    total_pressure: usize,
    total_flow_rate: usize,
    minutes_left: usize,
}

impl Volcano {
    fn new(open_valves: HashSet<usize>, n_actors: usize, minutes_left: usize) -> Self {
        Volcano {
            remaining_valves: open_valves,
            current_valves: repeat(0).take(n_actors).collect(),
            prev_valve: None,
            total_pressure: 0,
            total_flow_rate: 0,
            minutes_left,
        }
    }

    fn tick(&mut self, valves: &[Valve], queue: &mut BinaryHeap<Volcano>) {
        self.minutes_left -= 1;
        self.total_pressure += self.total_flow_rate;

        if self.remaining_valves.is_empty() {
            return;
        }

        let possible_actions = (0..self.current_valves.len())
            .flat_map(|v| self.possible_actions(valves, v))
            .combinations(self.current_valves.len())
            .collect_vec();

        for actions in possible_actions {
            if actions.len() == 2 && (actions[0] == actions[1] || actions[0].0 == actions[1].0) {
                continue;
            }

            let mut new_volcano = self.clone();
            for (actor, action) in actions {
                new_volcano.apply_action(valves, actor, action);
            }

            queue.push(new_volcano);
        }
    }

    fn apply_action(&mut self, valves: &[Valve], actor: usize, action: Action) {
        match action {
            Action::Move(new_pos) => {
                self.current_valves[actor] = new_pos;
            }
            Action::Open(valve) => {
                if self.remaining_valves.remove(&valve) {
                    self.total_flow_rate += valves[valve].flow_rate;
                }
            }
        }
    }

    fn possible_actions(&self, valves: &[Valve], actor: usize) -> Vec<(usize, Action)> {
        let mut actions = vec![];

        let current_actor_pos = self.current_valves[actor];
        let current_valve = &valves[current_actor_pos];

        if self.remaining_valves.contains(&current_actor_pos) {
            actions.push((actor, Action::Open(current_actor_pos)));
        }

        for next_valve in current_valve.leads_to.iter() {
            actions.push((actor, Action::Move(*next_valve)));
        }

        actions
    }

    fn value(&self) -> usize {
        self.total_pressure + self.total_flow_rate * self.minutes_left
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Move(usize),
    Open(usize),
}

impl PartialOrd<Self> for Volcano {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Volcano {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    id: usize,
    label: String,
    flow_rate: usize,
    leads_to: Vec<usize>,
}

#[derive(FromStr, Debug, Clone)]
#[from_str(
    regex = "Valve (?P<id>[A-Z]{2}) has flow rate=(?P<flow_rate>[0-9]+); tunnels? leads? to valves? (?P<leads_to>.+)"
)]
struct ValveInput {
    id: String,
    flow_rate: usize,
    leads_to: String,
}
