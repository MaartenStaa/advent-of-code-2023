fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);

    println!("Sum of predictions: {}", part1(&input));
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
    input.iter().map(|row| predict_next_number(row)).sum()
}

fn predict_next_number(row: &[i64]) -> i64 {
    let differences = row
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();

    if differences.iter().all(|n| *n == 0) {
        row[row.len() - 1]
    } else {
        row[row.len() - 1] + predict_next_number(&differences)
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
