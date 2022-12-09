use std::collections::HashSet;
use std::fs;

type Point = (i32, i32);

fn main() {
    let input = fs::read_to_string("input/day09.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    return simulate(input, 2);
}

fn part_two(input: &str) -> u32 {
    return simulate(input, 10);
}

fn simulate(input: &str, rope_length: usize) -> u32 {
    let mut knots: Vec<Point> = Vec::new();
    let mut tail_positions: HashSet<Point> = HashSet::new();

    for _ in 0..rope_length {
        knots.push((0, 0));
    }

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();

        for _ in 0..steps.parse::<u32>().unwrap() {
            tail_positions.insert(knots[rope_length - 1]);

            move_head(&mut knots, String::from(direction));

            tail_positions.insert(knots[rope_length - 1]);
        }
    }

    return tail_positions.len() as u32;
}

fn move_head(knots: &mut Vec<Point>, direction: String) {
    knots[0] = point_moved_in_direction(knots[0], direction);

    for i in 1..knots.len() {
        knots[i] = position_following(knots[i], knots[i - 1])
    }
}

fn point_moved_in_direction(point: Point, direction: String) -> Point {
    match direction.as_str() {
        "R" => point_moved_by(point, (1, 0)),
        "L" => point_moved_by(point, (-1, 0)),
        "U" => point_moved_by(point, (0, -1)),
        "D" => point_moved_by(point, (0, 1)),
        _ => point
    }
}

fn point_moved_by(a: Point, b: Point) -> Point {
    return (a.0 + b.0, a.1 + b.1);
}

fn position_following(a: Point, b: Point) -> Point {
    if adjacent_or_overlapping(a, b) {
        return a;
    }

    return point_moved_by(a, direction(b, a));
}

fn adjacent_or_overlapping(a: Point, b: Point) -> bool {
    return i32::abs(a.0 - b.0) < 2 && i32::abs(a.1 - b.1) < 2;
}

fn direction(a: Point, b: Point) -> Point {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    return (
        if dx != 0 { dx / i32::abs(dx) } else { 0 },
        if dy != 0 { dy / i32::abs(dy) } else { 0 }
    );
}

#[cfg(test)]
mod tests {
    use std::char::from_digit;
    use std::ops::Range;

    use super::*;

    fn grid_as_string(grid: &Vec<(i32, i32)>, xr: Range<i32>, yr: Range<i32>) -> String {
        let mut lines: Vec<String> = Vec::new();

        for y in yr.start..yr.end {
            let mut line = String::from("");

            '_next: for x in xr.start..xr.end {
                for (knot, pos) in grid.iter().enumerate() {
                    if *pos == (x, y) {
                        let c = if knot == 0 { 'H' } else { from_digit(knot as u32, 10).unwrap() };
                        line.push(c);
                        continue '_next;
                    }
                }

                line.push('.');
            }

            lines.push(line.clone());
        }

        return lines.join("\n");
    }

    #[test]
    fn mutate_test() {
        let expected = fs::read_to_string("src/day09.sample1.txt").unwrap();
        let mut actual: Vec<String> = Vec::new();

        let mut data = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];

        let instructions = vec![
            ("R", 4),
            ("U", 4),
            ("L", 3),
            ("D", 1),
            ("R", 4),
            ("D", 1),
            ("L", 5),
            ("R", 2),
        ];

        actual.push(grid_as_string(&data, 0..6, -4..1));

        for (direction, steps) in instructions {
            actual.push(format!("== {direction} {steps} =="));

            for _ in 0..steps {
                move_head(&mut data, String::from(direction));

                actual.push(grid_as_string(&data, 0..6, -4..1));
            }
        }

        assert_eq!(expected, actual.join("\n\n"));
    }

    fn example_input() -> String {
        return vec![
            "R 4",
            "U 4",
            "L 3",
            "D 1",
            "R 4",
            "D 1",
            "L 5",
            "R 2",
        ].join("\n");
    }

    #[test]
    fn part_one_example() {
        assert_eq!(13, part_one(&example_input()));
    }

    #[test]
    fn part_two_example_one() {
        assert_eq!(1, part_two(&example_input()));
    }

    #[test]
    fn part_two_example_two() {
        let vec1 = vec![
            "R 5",
            "U 8",
            "L 8",
            "D 3",
            "R 17",
            "D 10",
            "L 25",
            "U 20",
        ];

        assert_eq!(36, part_two(&vec1.join("\n")));
    }
}
