use std::cell::{Cell, RefCell};
use std::fs;

use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: RefCell<Vec<u64>>,
    operation: String,
    divisible: u64,
    divisible_true: usize,
    divisible_false: usize,
    inspections: Cell<u64>,
}

impl Monkey {
    fn next_monkey(&self, worry_level: u64) -> usize {
        if worry_level % self.divisible == 0 { self.divisible_true } else { self.divisible_false }
    }

    fn worry_level(&self, worry_level: &u64) -> u64 {
        let operation = self.operation.replace("old", worry_level.to_string().as_str());
        let mut iter = operation.splitn(3, " ");
        let left = iter.next().unwrap().parse::<u64>().unwrap();
        let operator = iter.next().unwrap();
        let right = iter.next().unwrap().parse::<u64>().unwrap();

        return match operator {
            "+" => left + right,
            "*" => left * right,
            _ => panic!("unexpected operator {operator}")
        };
    }

    fn increment_inspections(&self) {
        self.inspections.set(self.inspections.get() + 1);
    }

    fn drain_items(&self) -> Vec<u64> {
        return self.items.take();
    }

    fn add_item(&self, worry_level: &u64) {
        self.items.borrow_mut().push(*worry_level);
    }
}

fn monkeys_from_input(input: &str) -> Vec<Monkey> {
    let re = Regex::new(r"(?x)
        Monkey\ (?P<monkey>\d+):\n
        \ \ Starting\ items:\ (?P<items>.+)\n
        \ \ Operation:\ new\ =\ (?P<operation>.+)\n
        \ \ Test:\ divisible\ by\ (?P<divisible>.+)\n
        \ \ \ \ If\ true:\ throw\ to\ monkey\ (?P<divisible_true>.+)\n
        \ \ \ \ If\ false:\ throw\ to\ monkey\ (?P<divisible_false>.+)"
    ).unwrap();

    return re.captures_iter(input)
        .map(|item| Monkey {
            items: RefCell::new(item.name("items").unwrap().as_str().split(", ")
                .map(|i| u64::from(i.parse::<u64>().unwrap()))
                .collect()),
            operation: String::from(item.name("operation").unwrap().as_str()),
            divisible: item.name("divisible").unwrap().as_str().parse().unwrap(),
            divisible_true: item.name("divisible_true").unwrap().as_str().parse().unwrap(),
            divisible_false: item.name("divisible_false").unwrap().as_str().parse().unwrap(),
            inspections: Cell::new(0),
        })
        .collect();
}

fn inspections(monkeys: &mut Vec<Monkey>, rounds: u64, divisor: u64, divprod: u64) -> u64 {
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let monkey = monkeys.get(index).unwrap();

            for current_worry_level in monkey.drain_items() {
                monkey.increment_inspections();

                let mut new_worry_level = monkey.worry_level(&current_worry_level) / divisor;

                if divprod != 0 {
                    new_worry_level %= divprod;
                }

                monkeys[monkey.next_monkey(new_worry_level)].add_item(&new_worry_level);
            }
        }
    }

    return monkeys.iter().map(|m| m.inspections.get()).sorted().rev().take(2).product();
}

fn part_one(input: &str) -> u64 {
    let mut monkeys = monkeys_from_input(input);

    return inspections(&mut monkeys, 20, 3, 0);
}

fn part_two(input: &str) -> u64 {
    let mut monkeys = monkeys_from_input(input);
    let divprod: u64 = monkeys.iter().map(|m| m.divisible).product();

    return inspections(&mut monkeys, 10_000, 1, divprod);
}

fn main() {
    let input = fs::read_to_string("input/day11.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        return vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ].join("\n");
    }

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(&example_input()), 10605);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(&example_input()), 2713310158);
    }
}
