use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use regex::Regex;

type Stacks = HashMap<u32, Vec<String>>;

struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let captures = re.captures(input).unwrap();

        return Instruction {
            count: captures[1].parse().unwrap(),
            from: captures[2].parse().unwrap(),
            to: captures[3].parse().unwrap(),
        };
    }
}

fn stacks_from_input(input: &str) -> Stacks {
    let re = Regex::new(r"( {3}|\[.])").unwrap();

    let mut stacks: Stacks = HashMap::new();

    for line in input.lines().rev().dropping(1) {
        for (index, capture) in re.captures_iter(line).enumerate() {
            if capture[1].trim() == "" {
                continue;
            }

            let key = (index + 1) as u32;
            if !stacks.contains_key(&key) {
                stacks.insert(key, Vec::new());
            }

            let string = String::from(&capture[1][1..=1]);

            stacks.get_mut(&key).unwrap().push(string);
        }
    }

    return stacks;
}

fn crane_mover_9000(stacks: &mut Stacks, instruction: Instruction) {
    for _i in 0..instruction.count {
        let item = stacks.get_mut(&instruction.from).unwrap().pop().unwrap();
        stacks.get_mut(&instruction.to).unwrap().push(item);
    }
}

fn crane_mover_9001(stacks: &mut Stacks, instruction: Instruction) {
    let from_stack = stacks.get_mut(&instruction.from).unwrap();
    let from_size = from_stack.len() - instruction.count as usize;

    let items: Vec<String> = from_stack.drain(from_size..).collect();

    stacks.get_mut(&instruction.to).unwrap().extend_from_slice(&items);
}

fn process_stacks_and_return_top_items(input: &str, processor: fn(stacks: &mut Stacks, instruction: Instruction)) -> String {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks_from_input(stack_input);

    instruction_input.lines()
        .map(Instruction::from_string)
        .for_each(|ins| processor(&mut stacks, ins));

    return stacks.keys()
        .sorted()
        .map(|k| stacks.get(k).unwrap().last().unwrap())
        .join("");
}

fn part_one(input: &str) -> String {
    return process_stacks_and_return_top_items(input, crane_mover_9000);
}

fn part_two(input: &str) -> String {
    return process_stacks_and_return_top_items(input, crane_mover_9001);
}

fn main() {
    let input = fs::read_to_string("input/day05.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let vec1 = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let string = vec1.join("\n");

        let result = part_one(&string);

        assert_eq!("CMZ", result);
    }

    #[test]
    fn part_two_example() {
        let vec1 = vec![
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let string = vec1.join("\n");

        let result = part_two(&string);

        assert_eq!("MCD", result);
    }
}
