use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    let test = solve(INPUT_TEST);
    assert_eq!(test.0, 13140);
    println!("Test B:\n{}\n", test.1);

    let (a, b) = solve(INPUT);
    println!("Part A: {}", a);
    println!("Part B:\n{}\n", b);
}

fn solve(input: &str) -> (isize, String) {
    let mut register = 1;
    let mut cycle = 1;
    let mut output = vec![];

    let mut signal_strengths = vec![];

    for line in input.lines() {
        write_pixel(register, cycle, &mut output);
        signal_strengths.push(cycle * register);
        cycle += 1;

        if line == "noop" {
            continue;
        }

        let (left, right) = line.split_once(" ").unwrap();
        if left == "addx" {
            write_pixel(register, cycle, &mut output);
            signal_strengths.push(cycle * register);
            cycle += 1;

            register += right.parse::<isize>().unwrap();
        }
    }

    (
        signal_strengths
            .chunks(40)
            .map(|chunk| chunk[19])
            .sum::<isize>(),
        output
            .chunks(40)
            .map(|l| l.join(""))
            .collect_vec()
            .join("\n")
            .to_string(),
    )
}

fn write_pixel(register: isize, cycle: isize, output: &mut Vec<&str>) {
    output.push(
        if register >= ((cycle - 1) % 40) - 1 && register <= ((cycle - 1) % 40) + 1 {
            "#"
        } else {
            "."
        },
    );
}
