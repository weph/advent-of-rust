use std::fs;

use itertools::Itertools;

fn column_values(map: &Vec<Vec<u32>>, col: usize) -> Vec<u32> {
    return map.iter().map(|r| r[col]).collect_vec();
}

fn visible(values: &Vec<u32>, index: usize) -> bool {
    return index == 0 ||
        index == values.len() - 1 ||
        values.iter().take(index).max().unwrap_or(&0) < &values[index] ||
        values.iter().skip(index + 1).max().unwrap_or(&0) < &values[index];
}

fn visible_in_row(map: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    return visible(&map[row], col);
}

fn visible_in_col(map: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    return visible(&column_values(&map, col), row);
}

fn part_one(input: &str) -> u32 {
    let map = map_from_input(input);

    let mut result = 0;

    for (row, line) in map.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            if visible_in_row(&map, col, row) || visible_in_col(&map, col, row) {
                result += 1;
            }
        }
    }

    return result;
}

fn count(values: Vec<&u32>, limit: u32) -> u32 {
    let mut count = 0;

    for value in values.iter() {
        count += 1;

        if *value >= &limit {
            break;
        }
    }

    return count;
}

fn scenic_score(map: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    let column = column_values(map, col);

    let value = map[row][col];

    return count(map[row][0..col].iter().rev().collect_vec(), value) *
        count(map[row][col + 1..map[row].len()].iter().collect_vec(), value) *
        count(column[0..row].iter().rev().collect_vec(), value) *
        count(column[row + 1..map[row].len()].iter().collect_vec(), value);
}

fn part_two(input: &str) -> u32 {
    let map = map_from_input(input);

    let mut products: Vec<u32> = Vec::new();
    for (row, line) in map.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            products.push(scenic_score(&map, col, row));
        }
    }

    return *products.iter().max().unwrap();
}

fn map_from_input(input: &str) -> Vec<Vec<u32>> {
    return input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
}

fn main() {
    let input = fs::read_to_string("input/day08.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        let vec1 = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];

        return vec1.join("\n");
    }

    #[test]
    fn part_one_example() {
        assert_eq!(21, part_one(&example_input()));
    }

    #[test]
    fn part_two_example() {
        assert_eq!(8, part_two(&example_input()));
    }
}
