fn main() {
    let input = include_str!("input.txt");

    println!("sum (simple): {}", lines_sum_simple(input));
}

fn lines_sum_simple(input: &str) -> u32 {
    lines_sum(input, |line| {
        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        digits[0] * 10 + digits[digits.len() - 1]
    })
}

fn lines_sum(input: &str, digits: impl Fn(&str) -> u32) -> u32 {
    input.lines().map(digits).sum()
}

#[test]
fn part1() {
    assert_eq!(lines_sum_simple("1122"), 12);
    assert_eq!(
        lines_sum_simple(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        ),
        142
    );
}
