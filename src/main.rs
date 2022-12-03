mod day01;
mod day02;
mod day03;

fn main() {
    println!("{:#?}", day01::run(include_str!("day01.txt")));
    println!("{:#?}", day02::run(include_str!("day02.txt")));
    println!("{:#?}", day03::run(include_str!("day03.txt")));
}
