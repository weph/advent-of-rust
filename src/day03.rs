use std::fs;

fn first_intersecting_character(strings: Vec<&str>) -> char {
    'next_char: for c in strings[0].chars() {
        for s in strings[1..].iter() {
            if !s.contains(c) {
                continue 'next_char;
            }
        }

        return c;
    }

    panic!("No intersecting character found");
}

fn priority_sum(items: Vec<char>) -> u32 {
    return items.iter()
        .map(|c| (*c as u32) - if c.is_uppercase() { 38 } else { 96 })
        .sum();
}

fn part_one(input: Vec<&str>) -> u32 {
    return priority_sum(
        input.iter()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                return first_intersecting_character(vec![a, b]);
            })
            .collect()
    );
}

fn part_two(input: Vec<&str>) -> u32 {
    return priority_sum(
        input.chunks(3)
            .map(|lines| first_intersecting_character(lines.to_vec()))
            .collect()
    );
}

fn main() {
    let input = fs::read_to_string("input/day03.txt").unwrap();

    println!("Part One: {}", part_one(input.lines().collect()));
    println!("Part Two: {}", part_two(input.lines().collect()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_intersecting_character_should_return_only_the_first_if_there_are_multiple_intersections() {
        let result = first_intersecting_character(vec!["abcd", "bcde", "cdef"]);

        assert_eq!('c', result);
    }

    #[test]
    #[should_panic]
    fn first_intersecting_character_should_panic_if_there_is_no_intersecting_character() {
        first_intersecting_character(vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn part_one_example() {
        let result = part_one(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);

        assert_eq!(157, result);
    }

    #[test]
    fn part_two_example() {
        let result = part_two(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);

        assert_eq!(70, result);
    }
}