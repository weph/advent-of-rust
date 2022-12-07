use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn all_directories(path: &Vec<String>) -> Vec<String> {
    return (0..path.len()).map(|i| path[0..i + 1].join("/")).collect();
}

fn directory_sizes(input: &str) -> HashMap<String, u32> {
    let mut tree: HashMap<String, u32> = HashMap::new();

    let directory_regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    let file_regex = Regex::new(r"^(\d+) (.+)$").unwrap();

    let mut path = vec![String::from(".")];

    for line in input.lines() {
        for directory in directory_regex.captures(line) {
            match &directory[1] {
                "/" => path.truncate(1),
                ".." => { path.pop(); }
                _ => path.push(String::from(&directory[1]))
            };
        }

        for file in file_regex.captures(line) {
            let file_size = file[1].parse::<u32>().unwrap();
            for p in all_directories(&path) {
                let new_size = tree.get(&p).unwrap_or(&0) + file_size;
                tree.insert(p, new_size);
            }
        }
    }

    return tree;
}

fn part_one(input: &str) -> u32 {
    return directory_sizes(input).values().filter(|v| *v < &100000).sum();
}

fn part_two(input: &str) -> u32 {
    let sizes = directory_sizes(input);

    let total = 70000000;
    let required = 30000000;

    let unused = total - sizes.get(".").unwrap();
    let missing = required - unused;

    return *sizes.values().filter(|v| *v >= &missing).min().unwrap();
}

fn main() {
    let input = fs::read_to_string("input/day07.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        let vec1 = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        return vec1.join("\n");
    }

    #[test]
    fn part_one_example() {
        assert_eq!(94853 + 584, part_one(&example_input()));
    }

    #[test]
    fn part_two_example() {
        assert_eq!(24933642, part_two(&example_input()));
    }
}
