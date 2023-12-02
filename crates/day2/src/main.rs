use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let games = parse_games(input);

    println!("possible games sum: {}", possible_games_sum(&games));
}

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

struct Sample {
    amounts: HashMap<Color, u32>,
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn parse_game(input: &str) -> Game {
    let (prefix, samples_text) = input.split_once(": ").unwrap();
    let id = prefix.strip_prefix("Game ").unwrap().parse().unwrap();

    let samples = samples_text
        .split("; ")
        .map(|sample| {
            let amounts = sample
                .split(", ")
                .map(|amount| {
                    let (amount, color) = amount.split_once(" ").unwrap();
                    let amount = amount.parse().unwrap();
                    let color = match color {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => panic!("unknown color: {}", color),
                    };

                    (color, amount)
                })
                .collect();

            Sample { amounts }
        })
        .collect();

    Game { id, samples }
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

#[test]
fn part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let games = parse_games(input);

    assert_eq!(games.len(), 5);
    assert_eq!(possible_games_sum(&games), 8);
}
