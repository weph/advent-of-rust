use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day01.txt").unwrap();

    let calories_per_elf: Vec<u32> = input
        .split("\n\n")
        .map(|items| items.split("\n").map(|x| -> u32 { x.parse().unwrap() }).sum())
        .sorted()
        .rev()
        .collect();

    println!("Part One: {}", calories_per_elf.iter().max().unwrap());
    println!("Part Two: {}", calories_per_elf.iter().take(3).sum::<u32>());
}