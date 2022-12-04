use std::fs;
use std::ops::RangeInclusive;

use regex::Regex;

type Sections = RangeInclusive<u32>;

fn line_to_sections_pair(line: &str) -> (Sections, Sections) {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let captures = re.captures(line).unwrap();

    return (
        Sections::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
        Sections::new(captures[3].parse().unwrap(), captures[4].parse().unwrap())
    );
}

fn is_either_contained(a: &Sections, b: &Sections) -> bool {
    return (b.contains(a.start()) && b.contains(a.end())) ||
        (a.contains(b.start()) && a.contains(b.end()));
}

fn is_overlapping(a: &Sections, b: &Sections) -> bool {
    return b.contains(a.start()) || b.contains(a.end()) ||
        a.contains(b.start()) || a.contains(b.end());
}

fn number_of_matching_pairs(input: Vec<&str>, filter: fn(a: &Sections, b: &Sections) -> bool) -> u32 {
    return input.iter()
        .map(|l| line_to_sections_pair(l))
        .filter(|(a, b)| filter(a, b))
        .count() as u32;
}

fn part_one(input: Vec<&str>) -> u32 {
    return number_of_matching_pairs(input, is_either_contained);
}

fn part_two(input: Vec<&str>) -> u32 {
    return number_of_matching_pairs(input, is_overlapping);
}

fn main() {
    let input = fs::read_to_string("input/day04.txt").unwrap();

    println!("Part One: {}", part_one(input.lines().collect()));
    println!("Part Two: {}", part_two(input.lines().collect()));
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1..=2, 3..=4, false)]
    #[case(1..=3, 3..=4, false)]
    #[case(4..=6, 3..=4, false)]
    #[case(5..=6, 3..=4, false)]
    #[case(1..=4, 3..=4, true)]
    #[case(3..=6, 3..=4, true)]
    #[case(3..=7, 2..=8, true)]
    #[case(2..=4, 2..=4, true)]
    fn is_either_contained_test(#[case] a: Sections, #[case] b: Sections, #[case] expected: bool) {
        assert_eq!(
            expected,
            is_either_contained(&a, &b),
            "{a:?} {} be contained in {b:?}", if expected { "should" } else { "should not" }
        );
    }

    #[rstest]
    #[case(1..=2, 3..=4, false)]
    #[case(5..=6, 3..=4, false)]
    #[case(1..=3, 3..=4, true)]
    #[case(1..=5, 3..=4, true)]
    #[case(4..=5, 3..=4, true)]
    fn is_overlapping_test(#[case] a: Sections, #[case] b: Sections, #[case] expected: bool) {
        assert_eq!(
            expected,
            is_overlapping(&a, &b),
            "{a:?} {} overlap with {b:?}", if expected { "should" } else { "should not" }
        );
    }

    #[test]
    fn part_one_example() {
        let result = part_one(vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ]);

        assert_eq!(2, result);
    }

    #[test]
    fn part_two_example() {
        let result = part_two(vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ]);

        assert_eq!(4, result);
    }
}