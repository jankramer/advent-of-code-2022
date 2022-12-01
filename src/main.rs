mod day01;
mod day02;

fn main() {
    println!("{:#?}", day01::run(include_str!("day01.txt")));
    println!("{:#?}", day02::run(include_str!("day02.txt")));
}
