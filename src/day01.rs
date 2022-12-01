use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01.txt").unwrap();

    let mut calories_per_elf: Vec<u32> = input
        .split("\n\n")
        .map(|items| items.split("\n").map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    calories_per_elf.sort();

    println!("Part One: {}", calories_per_elf.iter().max().unwrap());
    println!("Part Two: {}", calories_per_elf.iter().rev().take(3).sum::<u32>());
}