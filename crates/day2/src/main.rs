use chumsky::{prelude::*, text::newline};
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let games = games_parser().parse(input).unwrap();

    println!("possible games sum: {}", possible_games_sum(&games));
    println!("game powers sum: {}", game_powers_sum(&games));
}

#[derive(Debug)]
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Sample {
    amounts: HashMap<Color, u32>,
}

fn games_parser() -> impl Parser<char, Vec<Game>, Error = Simple<char>> {
    let red = text::int(10)
        .map(|n: String| n.parse().unwrap())
        .then_ignore(just(" red"))
        .map(|n| (Color::Red, n));
    let green = text::int(10)
        .map(|n: String| n.parse().unwrap())
        .then_ignore(just(" green"))
        .map(|n| (Color::Green, n));
    let blue = text::int(10)
        .map(|n: String| n.parse().unwrap())
        .then_ignore(just(" blue"))
        .map(|n| (Color::Blue, n));

    let sample = choice((red, green, blue))
        .separated_by(just(", "))
        .collect()
        .map(|amounts| Sample { amounts });

    just("Game ")
        .ignore_then(text::int(10))
        .map(|n: String| n.parse().unwrap())
        .then_ignore(just(": "))
        .then(sample.separated_by(just("; ")))
        .map(|(id, samples)| Game { id, samples })
        .separated_by(newline())
        .at_least(5)
        .collect()
}

fn possible_games_sum(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|game| {
            // Find all games that at no point revealed more than 12 red cubes, 13
            // green cubes, or 14 blue cubes.
            is_game_possible(game, 12, 13, 14)
        })
        .map(|game| game.id)
        .sum()
}

fn is_game_possible(game: &Game, red: u32, green: u32, blue: u32) -> bool {
    game.samples.iter().all(|sample| {
        sample.amounts.get(&Color::Red).unwrap_or(&0) <= &red
            && sample.amounts.get(&Color::Green).unwrap_or(&0) <= &green
            && sample.amounts.get(&Color::Blue).unwrap_or(&0) <= &blue
    })
}

fn game_powers_sum(games: &[Game]) -> u32 {
    games.iter().map(game_power).sum()
}

fn game_power(game: &Game) -> u32 {
    // Find the lowest possible number of cubes that could have been used in
    // this game. Then multiply them together to get the game's power.
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for sample in game.samples.iter() {
        if let Some(r) = sample.amounts.get(&Color::Red) {
            red = red.max(*r);
        }
        if let Some(g) = sample.amounts.get(&Color::Green) {
            green = green.max(*g);
        }
        if let Some(b) = sample.amounts.get(&Color::Blue) {
            blue = blue.max(*b);
        }
    }

    red * green * blue
}

#[cfg(test)]
const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[test]
fn part1() {
    let games = games_parser().parse(TEST_INPUT).unwrap();

    assert_eq!(games.len(), 5);
    assert_eq!(possible_games_sum(&games), 8);
}

#[test]
fn part2() {
    let games = games_parser().parse(TEST_INPUT).unwrap();

    assert_eq!(game_powers_sum(&games), 2286);
}
