use std::fs;

fn input_to_cycle_values(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut cycle_values: Vec<i32> = Vec::new();

    for line in input.lines() {
        cycle_values.push(x);

        if line.starts_with("addx") {
            cycle_values.push(x);

            x += line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        }
    }

    cycle_values.push(x);

    return cycle_values;
}

fn part_one(input: &str) -> i32 {
    let cycle_values = input_to_cycle_values(input);

    return (19..cycle_values.len())
        .step_by(40)
        .map(|cycle| (cycle + 1) as i32 * cycle_values[cycle])
        .sum();
}

fn part_two(input: &str) -> String {
    let mut screen = String::new();

    for (cycle, x) in input_to_cycle_values(input).iter().take(240).enumerate() {
        screen.push(if i32::abs(*x - (cycle % 40) as i32) > 1 { '.' } else { '#' });

        if (cycle + 1) % 40 == 0 {
            screen.push('\n');
        }
    }

    return screen;
}

fn main() {
    let input = fs::read_to_string("input/day10.txt").unwrap();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: \n{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        return vec![
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx -35",
            "addx 1",
            "addx 24",
            "addx -19",
            "addx 1",
            "addx 16",
            "addx -11",
            "noop",
            "noop",
            "addx 21",
            "addx -15",
            "noop",
            "noop",
            "addx -3",
            "addx 9",
            "addx 1",
            "addx -3",
            "addx 8",
            "addx 1",
            "addx 5",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx -36",
            "noop",
            "addx 1",
            "addx 7",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "addx 6",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx 7",
            "addx 1",
            "noop",
            "addx -13",
            "addx 13",
            "addx 7",
            "noop",
            "addx 1",
            "addx -33",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "noop",
            "noop",
            "noop",
            "addx 8",
            "noop",
            "addx -1",
            "addx 2",
            "addx 1",
            "noop",
            "addx 17",
            "addx -9",
            "addx 1",
            "addx 1",
            "addx -3",
            "addx 11",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx -13",
            "addx -19",
            "addx 1",
            "addx 3",
            "addx 26",
            "addx -30",
            "addx 12",
            "addx -1",
            "addx 3",
            "addx 1",
            "noop",
            "noop",
            "noop",
            "addx -9",
            "addx 18",
            "addx 1",
            "addx 2",
            "noop",
            "noop",
            "addx 9",
            "noop",
            "noop",
            "noop",
            "addx -1",
            "addx 2",
            "addx -37",
            "addx 1",
            "addx 3",
            "noop",
            "addx 15",
            "addx -21",
            "addx 22",
            "addx -6",
            "addx 1",
            "noop",
            "addx 2",
            "addx 1",
            "noop",
            "addx -10",
            "noop",
            "noop",
            "addx 20",
            "addx 1",
            "addx 2",
            "addx 2",
            "addx -6",
            "addx -11",
            "noop",
            "noop",
            "noop",
        ].join("\n");
    }

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(&example_input()), 13140);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two(&example_input()),
            "##..##..##..##..##..##..##..##..##..##..\n".to_owned() +
                "###...###...###...###...###...###...###.\n" +
                "####....####....####....####....####....\n" +
                "#####.....#####.....#####.....#####.....\n" +
                "######......######......######......####\n" +
                "#######.......#######.......#######.....\n"
        );
    }
}
