fn main() {
    let input = include_str!("input.txt");

    println!("sum: {}", lines_sum(input));
}

fn lines_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[test]
fn part1() {
    assert_eq!(lines_sum("1122"), 12);
    assert_eq!(lines_sum(TEST_INPUT), 142);
}
