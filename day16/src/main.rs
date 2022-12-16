use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::repeat;

use itertools::Itertools;
use parse_display::{Display, FromStr};

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), 1651);

    println!("Part A: {}", solve(INPUT));
}

fn solve(input: &str) -> usize {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|l| l.parse::<ValveInput>().unwrap().into())
        .map(|x: Valve| (x.id.clone(), x))
        .collect();

    let mut volcanos: Vec<Volcano> = vec![Volcano::new()];

    for i in 1..=30 {
        println!(
            "Minute {}, total flow rate {}, valves open {}",
            i,
            volcanos[0].total_flow_rate,
            volcanos[0].open.iter().join(", ")
        );

        for mut volcano in volcanos.clone() {
            for new_volcano in volcano.tick(&valves) {
                volcanos.push(new_volcano);
            }
        }

        volcanos.sort();
        volcanos.reverse();
        volcanos.truncate(300000);
    }

    volcanos[0].total_pressure
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Volcano {
    open: HashSet<String>,
    current_valve: String,
    total_pressure: usize,
    total_flow_rate: usize,
}

impl Volcano {
    fn new() -> Self {
        Volcano {
            open: HashSet::new(),
            current_valve: "AA".to_string(),
            total_pressure: 0,
            total_flow_rate: 0,
        }
    }

    fn tick(&mut self, valves: &HashMap<String, Valve>) -> Vec<Volcano> {
        let current_valve = valves.get(&self.current_valve).unwrap();

        self.total_pressure += self.total_flow_rate;

        if self.open.len() == valves.len() {
            return vec![];
        }

        let mut new_volcanos = vec![];

        if !self.open.contains(&self.current_valve) {
            let mut new_self_opened = self.clone();
            new_self_opened.open.insert(self.current_valve.clone());
            new_self_opened.total_flow_rate += current_valve.flow_rate;

            new_volcanos.push(new_self_opened);
        }

        for next_valve in current_valve.valves.iter() {
            let mut new_with_next_as_current = self.clone();
            new_with_next_as_current.current_valve = next_valve.clone();

            new_volcanos.push(new_with_next_as_current);
        }

        new_volcanos
    }
}
impl PartialOrd<Self> for Volcano {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Volcano {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_flow_rate
            .cmp(&other.total_flow_rate)
            .then(self.total_pressure.cmp(&other.total_pressure))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    id: String,
    flow_rate: usize,
    valves: Vec<String>,
}

#[derive(FromStr, Debug)]
#[from_str(
    regex = "Valve (?P<id>[A-Z]{2}) has flow rate=(?P<flow_rate>[0-9]+); tunnels? leads? to valves? (?P<leads_to>.+)"
)]
struct ValveInput {
    id: String,
    flow_rate: usize,
    leads_to: String,
}

impl From<ValveInput> for Valve {
    fn from(x: ValveInput) -> Self {
        Valve {
            id: x.id,
            flow_rate: x.flow_rate,
            valves: x.leads_to.split(", ").map(|v| v.to_string()).collect_vec(),
        }
    }
}
