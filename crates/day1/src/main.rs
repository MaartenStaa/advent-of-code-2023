use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("sum (simple): {}", lines_sum_simple(input));
    println!("sum (complex): {}", lines_sum_complex(input));
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

fn lines_sum_complex(input: &str) -> u32 {
    let digit_names = {
        let mut hashmap = HashMap::new();
        hashmap.insert("one", 1);
        hashmap.insert("two", 2);
        hashmap.insert("three", 3);
        hashmap.insert("four", 4);
        hashmap.insert("five", 5);
        hashmap.insert("six", 6);
        hashmap.insert("seven", 7);
        hashmap.insert("eight", 8);
        hashmap.insert("nine", 9);
        hashmap
    };

    lines_sum(input, |line| {
        let mut remaining_str = line;
        let mut digits = Vec::new();

        'outer: while remaining_str.len() > 0 {
            for (name, digit) in &digit_names {
                if remaining_str.starts_with(name) {
                    digits.push(*digit);

                    remaining_str = &remaining_str[1..];
                    continue 'outer;
                }
            }

            if let Some(digit) = remaining_str.chars().next().and_then(|c| c.to_digit(10)) {
                digits.push(digit);
            }

            remaining_str = &remaining_str[1..];
        }

        digits[0] * 10 + digits[digits.len() - 1]
    })
}

fn lines_sum(input: &str, digits: impl Fn(&str) -> u32) -> u32 {
    input.lines().map(digits).sum()
}

#[test]
fn day1_part1() {
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

#[test]
fn day1_part2() {
    assert_eq!(
        lines_sum_complex(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        ),
        281
    )
}
