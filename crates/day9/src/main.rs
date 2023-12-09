fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);

    println!("Sum of next values: {}", part1(&input));
    println!("Sum of previous values: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().expect("Invalid input"))
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| predict_number(row, PredictionMode::Next))
        .sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| predict_number(row, PredictionMode::Previous))
        .sum()
}

enum PredictionMode {
    Next,
    Previous,
}

fn predict_number(row: &[i64], mode: PredictionMode) -> i64 {
    let differences = row
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();

    if differences.iter().all(|n| *n == 0) {
        row[match mode {
            PredictionMode::Next => row.len() - 1,
            PredictionMode::Previous => 0,
        }]
    } else {
        match mode {
            PredictionMode::Next => row[row.len() - 1] + predict_number(&differences, mode),
            PredictionMode::Previous => row[0] - predict_number(&differences, mode),
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn day9_part1() {
    let input = parse(TEST_INPUT);
    assert_eq!(part1(&input), 114);
}

#[test]
fn day9_part2() {
    let input = parse(TEST_INPUT);
    assert_eq!(part2(&input), 2);
}
