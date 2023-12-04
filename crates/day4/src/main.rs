use std::collections::HashSet;

use chumsky::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let cards = cards_parser().parse(input).unwrap();

    println!(
        "score sum: {}",
        cards.iter().map(|c| c.score()).sum::<u32>()
    );
}

#[derive(Debug)]
struct ScratchCard {
    #[allow(dead_code)]
    id: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl ScratchCard {
    fn score(&self) -> u32 {
        let winning_number_count = self
            .winning_numbers
            .intersection(&self.card_numbers)
            .count() as u32;

        if winning_number_count == 0 {
            0
        } else {
            2u32.pow(winning_number_count - 1)
        }
    }
}

fn cards_parser() -> impl Parser<char, Vec<ScratchCard>, Error = Simple<char>> {
    let numbers = text::int(10)
        .map(|n: String| n.parse().unwrap())
        .separated_by(text::whitespace().at_least(1))
        .collect();

    text::keyword("Card")
        .padded()
        .ignore_then(text::int(10))
        .map(|n: String| n.parse().unwrap())
        .then_ignore(just(':').padded())
        .then(numbers)
        .then_ignore(just('|').padded())
        .then(numbers)
        .map(|((id, winning_numbers), card_numbers)| ScratchCard {
            id,
            winning_numbers,
            card_numbers,
        })
        .separated_by(text::newline())
        .allow_trailing()
        .then_ignore(end())
        .collect()
}

#[cfg(test)]
const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[test]
fn day4_part1() {
    let cards = cards_parser().parse(TEST_INPUT).unwrap();

    assert_eq!(cards.len(), 6);
    assert_eq!(cards[0].score(), 8);
    assert_eq!(cards[1].score(), 2);
    assert_eq!(cards[5].score(), 0);

    assert_eq!(cards.iter().map(|c| c.score()).sum::<u32>(), 13);
}
