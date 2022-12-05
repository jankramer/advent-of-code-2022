mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    println!("{:#?}", day01::run(include_str!("day01.txt")));
    println!("{:#?}", day02::run(include_str!("day02.txt")));
    println!("{:#?}", day03::run(include_str!("day03.txt")));
    println!("{:#?}", day04::run(include_str!("day04.txt")));
    println!("{:#?}", day05::run(include_str!("day05.txt")));
}
