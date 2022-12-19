use crate::Action::Build;
use crate::Material::{Clay, Geode, Obsidian, Ore};
use parse_display::{Display, FromStr};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve(INPUT_TEST), 33);
    println!("Part A passes for test input");
    println!("Part A: {}", solve(INPUT));

    assert_eq!(solve_b(INPUT_TEST), 3472);
    println!("Part B passes for test input");
    println!("Part A: {}", solve_b(INPUT));
}

fn solve(input: &str) -> isize {
    let blueprints: Vec<Blueprint> = input.lines().filter_map(|l| l.parse().ok()).collect();

    blueprints
        .par_iter()
        .map(|bp| simulate(bp, 24) * bp.id)
        .sum()
}

fn solve_b(input: &str) -> isize {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .filter_map(|l| l.parse().ok())
        .take(3)
        .collect();

    blueprints.par_iter().map(|bp| simulate(bp, 32)).product()
}

fn simulate(blueprint: &Blueprint, n_minutes: isize) -> isize {
    println!("Simulating blueprint {}", blueprint.id);
    let mut all_states = BinaryHeap::new();
    all_states.push(State::initial(n_minutes));

    for _i in 1..=n_minutes {
        let mut new_states = BinaryHeap::new();
        while let Some(state) = all_states.pop() {
            tick(state, blueprint, &mut new_states);
        }

        for _ in 0..100000 {
            if let Some(new_state) = new_states.pop() {
                all_states.push(new_state);
            } else {
                break;
            }
        }
    }

    all_states.pop().unwrap().geodes.amount
}

fn tick(state: State, blueprint: &Blueprint, all_states: &mut BinaryHeap<State>) {
    for action in state.possible_actions(blueprint) {
        let mut new_state = state.clone();
        new_state.apply(blueprint, action);

        all_states.push(new_state);
    }

    let mut new_state = state;
    new_state.apply(blueprint, Action::Nothing);
    all_states.push(new_state);
}

#[derive(Eq, PartialEq)]
enum Action {
    Nothing,
    Build(Material),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    minutes_left: isize,
    ore: MaterialState,
    clay: MaterialState,
    obsidian: MaterialState,
    geodes: MaterialState,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value()
            .cmp(&other.value())
            .then(self.obsidian.robots.cmp(&other.obsidian.robots))
    }
}

impl State {
    fn initial(minutes_left: isize) -> State {
        State {
            minutes_left,
            ore: MaterialState::new(1, 0),
            clay: MaterialState::new(0, 0),
            obsidian: MaterialState::new(0, 0),
            geodes: MaterialState::new(0, 0),
        }
    }

    fn value(&self) -> isize {
        self.geodes.amount + self.geodes.robots * self.minutes_left
    }

    fn possible_actions(&self, blueprint: &Blueprint) -> Vec<Action> {
        let mut actions = vec![];
        if self.ore.amount >= blueprint.ore_ore {
            actions.push(Build(Ore));
        }

        if self.ore.amount >= blueprint.clay_ore {
            actions.push(Build(Clay));
        }

        if self.ore.amount >= blueprint.obsidian_ore && self.clay.amount >= blueprint.obsidian_clay
        {
            actions.push(Build(Obsidian));
        }

        if self.ore.amount >= blueprint.geode_ore
            && self.obsidian.amount >= blueprint.geode_obsidian
        {
            actions.push(Build(Geode));
        }

        actions
    }

    fn apply(&mut self, blueprint: &Blueprint, action: Action) {
        self.ore.amount += self.ore.robots;
        self.clay.amount += self.clay.robots;
        self.obsidian.amount += self.obsidian.robots;
        self.geodes.amount += self.geodes.robots;

        match action {
            Build(Ore) => {
                self.ore.amount -= blueprint.ore_ore;
                self.ore.robots += 1;
            }
            Build(Clay) => {
                self.ore.amount -= blueprint.clay_ore;
                self.clay.robots += 1;
            }
            Build(Obsidian) => {
                self.ore.amount -= blueprint.obsidian_ore;
                self.clay.amount -= blueprint.obsidian_clay;
                self.obsidian.robots += 1;
            }
            Build(Geode) => {
                self.ore.amount -= blueprint.geode_ore;
                self.obsidian.amount -= blueprint.geode_obsidian;
                self.geodes.robots += 1;
            }
            _ => {}
        }

        self.minutes_left -= 1;
    }
}

#[derive(Eq, PartialEq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MaterialState {
    robots: isize,
    amount: isize,
}

impl MaterialState {
    fn new(robots: isize, amount: isize) -> Self {
        MaterialState { robots, amount }
    }
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("Blueprint {id}: Each ore robot costs {ore_ore} ore. Each clay robot costs {clay_ore} ore. Each obsidian robot costs {obsidian_ore} ore and {obsidian_clay} clay. Each geode robot costs {geode_ore} ore and {geode_obsidian} obsidian.")]
struct Blueprint {
    id: isize,
    ore_ore: isize,
    clay_ore: isize,
    obsidian_ore: isize,
    obsidian_clay: isize,
    geode_ore: isize,
    geode_obsidian: isize,
}
