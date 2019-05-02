extern crate regex;

mod day01;
mod day02;

fn main() {
    println!("Advent of Code 2015");

    let (part1, part2) = day01::solution::run();
    println!("Day 1 ---- Part1: {}, Part2: {}", part1, part2);

    let (part1, part2) = day02::solution::run();
    println!("Day 2 ---- Part1: {}, Part2: {}", part1, part2);


}

