fn main() {
    let input = include_str!("input.txt");
    let mut entries = input
        .lines()
        .map(|line| Entry::try_parse(line, false))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort();

    println!(
        "Sum of win amounts: {}",
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| e.bid * (i + 1))
            .sum::<usize>()
    );

    let mut entries = input
        .lines()
        .map(|line| Entry::try_parse(line, true))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort();

    println!(
        "Sum of win amounts with jokers: {}",
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| e.bid * (i + 1))
            .sum::<usize>()
    );
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand([u8; 5]);

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn try_parse(s: &str, jokers: bool) -> Result<Self, ()> {
        let mut cards = [0; 5];
        for (i, c) in s.chars().enumerate() {
            match c {
                'A' => cards[i] = 14,
                'K' => cards[i] = 13,
                'Q' => cards[i] = 12,
                'J' if !jokers => cards[i] = 11,
                'T' => cards[i] = 10,
                n if n.is_digit(10) && n != '0' && n != '1' => {
                    cards[i] = n.to_digit(10).ok_or(())? as u8
                }
                // Joker
                'J' if jokers => cards[i] = 1,
                _ => return Err(()),
            }
        }

        Ok(Hand(cards))
    }

    fn hand_type(&self) -> HandType {
        let mut counts = [0; 15];
        let mut jokers = 0;
        for card in self.0.iter() {
            if *card == 1 {
                jokers += 1;
                continue;
            }
            counts[*card as usize] += 1;
        }

        // Make sure to sort so we look at highest amounts first, and don't
        // consume the jokers too early.
        counts.sort();

        let mut pairs = 0;
        let mut three = false;
        let mut four = false;
        let mut five = false;
        for count in counts.iter().rev() {
            if *count + jokers == 5 {
                five = true;
                jokers = 0;
            } else if *count + jokers == 4 {
                four = true;
                jokers = 0;
            } else if *count + jokers == 3 {
                three = true;
                jokers = 0;
            } else if *count + jokers == 2 {
                pairs += 1;
                jokers = 0;
            }
        }

        if five {
            HandType::FiveOfAKind
        } else if four {
            HandType::FourOfAKind
        } else if three && pairs == 1 {
            HandType::FullHouse
        } else if three {
            HandType::ThreeOfAKind
        } else if pairs == 2 {
            HandType::TwoPair
        } else if pairs == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        if self.hand_type() != other.hand_type() {
            return self_hand_type.partial_cmp(&other_hand_type);
        }

        for (self_card, other_card) in self.0.iter().zip(other.0.iter()) {
            if self_card != other_card {
                return self_card.partial_cmp(other_card);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Entry {
    hand: Hand,
    bid: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Entry {
    fn try_parse(s: &str, jokers: bool) -> Result<Self, ()> {
        let mut parts = s.split_ascii_whitespace();
        let hand = Hand::try_parse(parts.next().ok_or(())?, jokers)?;
        let bid = parts.next().unwrap().parse().unwrap();

        Ok(Entry { hand, bid })
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn day7_part1() {
    let mut entries = TEST_INPUT
        .lines()
        .map(|line| Entry::try_parse(line, false))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(entries.len(), 5);
    assert_eq!(entries[0].hand, Hand([3, 2, 10, 3, 13]));
    assert_eq!(entries[0].bid, 765);
    assert_eq!(entries[0].hand.hand_type(), HandType::OnePair);

    entries.sort();

    assert_eq!(
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| e.bid * (i + 1))
            .sum::<usize>(),
        6440
    );
}

#[test]
fn day7_part2() {
    let mut entries = TEST_INPUT
        .lines()
        .map(|line| Entry::try_parse(line, true))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(entries[3].hand.hand_type(), HandType::FourOfAKind);
    assert_eq!(entries[1].hand, Hand([10, 5, 5, 1, 5]));
    assert_eq!(entries[1].hand.hand_type(), HandType::FourOfAKind);

    entries.sort();

    assert_eq!(
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| e.bid * (i + 1))
            .sum::<usize>(),
        5905
    );
}
