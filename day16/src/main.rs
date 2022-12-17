use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::repeat;

use itertools::Itertools;
use parse_display::{Display, FromStr};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST, 30), 1651);
    println!("Part A: {}", solve(INPUT, 30));
}

const MAGIC_NUMBER: usize = 300000;

fn solve(input: &str, minutes_left: usize) -> usize {
    let valves_input: Vec<ValveInput> =
        input.lines().sorted().map(|l| l.parse().unwrap()).collect();

    let valves: Vec<Valve> = valves_input
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
        .collect();

    let mut volcanos: Vec<Volcano> = Vec::with_capacity(MAGIC_NUMBER);
    volcanos.push(Volcano::new(minutes_left));
    let mut queue: BinaryHeap<Volcano> = BinaryHeap::with_capacity(10 * MAGIC_NUMBER);

    for i in 1..=30 {
        println!(
            "Minute {}, total flow rate {}, valves open {}",
            i,
            volcanos[0].total_flow_rate,
            volcanos[0].open.iter().join(", ")
        );

        while let Some(mut volcano) = volcanos.pop() {
            volcano.tick(&valves, &mut queue);
        }

        let mut j = 0;
        while let Some(volcano) = queue.pop() {
            volcanos.push(volcano);

            j += 1;
            if j >= MAGIC_NUMBER {
                break;
            }
        }
    }

    volcanos[0].total_pressure
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Volcano {
    open: HashSet<usize>,
    current_valve: usize,
    total_pressure: usize,
    total_flow_rate: usize,
    minutes_left: usize,
}

impl Volcano {
    fn new(minutes_left: usize) -> Self {
        Volcano {
            open: HashSet::new(),
            current_valve: 0,
            total_pressure: 0,
            total_flow_rate: 0,
            minutes_left,
        }
    }

    fn tick(&mut self, valves: &Vec<Valve>, queue: &mut BinaryHeap<Volcano>) {
        let current_valve = &valves[self.current_valve];

        self.total_pressure += self.total_flow_rate;

        if self.open.len() == valves.len() {
            return;
        }

        if !self.open.contains(&self.current_valve) {
            let mut new_self_opened = self.clone();
            new_self_opened.open.insert(self.current_valve.clone());
            new_self_opened.total_flow_rate += current_valve.flow_rate;

            queue.push(new_self_opened);
        }

        for next_valve in current_valve.leads_to.iter() {
            let mut new_with_next_as_current = self.clone();
            new_with_next_as_current.current_valve = next_valve.clone();

            queue.push(new_with_next_as_current);
        }
    }

    fn value(&self) -> usize {
        self.total_pressure + self.total_flow_rate * self.minutes_left
    }
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
