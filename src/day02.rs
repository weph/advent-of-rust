extern crate core;

use std::fs;

use itertools::Itertools;

struct Input {
    player: String,
    opponent: String,
}

struct Game {
    player: Shape,
    opponent: Shape,
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn wins_against(&self) -> Shape {
        return match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        };
    }

    fn loses_against(&self) -> Shape {
        return match *self {
            Shape::Scissors => Shape::Rock,
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
        };
    }
}

fn game_from_input_for_part_one(input: &Input) -> Game {
    let opponent = match input.opponent.as_str() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid value {}", input.opponent)
    };

    let player = match input.player.as_str() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Invalid value {}", input.player)
    };

    return Game { opponent, player };
}

fn game_from_input_for_part_two(input: &Input) -> Game {
    let opponent = match input.opponent.as_str() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid value {}", input.opponent)
    };

    let player = match input.player.as_str() {
        "X" => opponent.wins_against(),
        "Y" => opponent,
        "Z" => opponent.loses_against(),
        _ => panic!("Invalid value {}", input.player)
    };

    return Game { opponent, player };
}

fn player_score(game: &Game) -> u16 {
    let points = match game.player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    if game.player == game.opponent {
        return points + 3;
    }

    if game.player.wins_against() == game.opponent {
        return points + 6;
    }

    return points;
}

fn player_total_score(games: Vec<Game>) -> u16 {
    return games.iter().map(player_score).sum();
}

fn map_input(input: &Vec<Input>, callback: fn(&Input) -> Game) -> Vec<Game> {
    return input.iter().map(callback).collect_vec();
}

fn main() {
    let input = fs::read_to_string("input/day02.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (opponent, player) = line.split_once(' ').unwrap();

            return Input { opponent: String::from(opponent), player: String::from(player) };
        })
        .collect_vec();

    println!("Part One: {}", player_total_score(map_input(&input, game_from_input_for_part_one)));
    println!("Part Two: {}", player_total_score(map_input(&input, game_from_input_for_part_two)));
}
