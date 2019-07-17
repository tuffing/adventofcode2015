extern crate regex;
mod loadinput;
mod day01;
mod day02;
mod day03;
//mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    println!("Advent of Code 2015");

    let (part1, part2) = day01::solution::run();
    println!("Day 1 ---- Part1: {}, Part2: {}", part1, part2);

    let (part1, part2) = day02::solution::run();
    println!("Day 2 ---- Part1: {}, Part2: {}", part1, part2);

    let (part1, part2) = day03::solution::run();
    println!("Day 3 ---- Part1: {}, Part2: {}", part1, part2);

    //let (part1, part2) = day04::solution::run("yzbqklnj");
    //println!("Day 4 ---- Part1: {}, Part2: {}", part1, part2);

    let (part1, part2) = day05::solution::run();
    println!("Day 5 ---- Part1: {}, Part2: {}", part1, part2);

    //let (part1, part2) = day06::solution::run();
    //println!("Day 6 ---- Part1: {}, Part2: {}", part1, part2);

    let (part1, part2) = day07::solution::run();
    println!("Day 7 ---- Part1: {}, Part2: {}", part1, part2);
}

