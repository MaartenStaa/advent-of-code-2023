fn main() {
    let input = include_str!("input.txt");
    let races = parse_part1(input);

    println!(
        "Product of number of ways to win: {}",
        races.iter().map(count_ways_to_win).product::<usize>()
    );
}

struct Race {
    time: u64,
    distance: u64,
}

fn count_ways_to_win(race: &Race) -> usize {
    for button_press_time in 1..race.time {
        let speed = button_press_time;
        let remaining_time = race.time - button_press_time;
        let distance_traveled = speed * remaining_time;

        // The first time we encounter this, we know enough to determine the
        // total number of ways to win, as it's symmetrical.
        if distance_traveled > race.distance {
            return (button_press_time..=(race.time - button_press_time)).count();
        }
    }

    0
}

fn parse_part1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    let mut distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    assert_eq!(lines.next(), None);

    times
        .map(|time| Race {
            time,
            distance: distances.next().unwrap(),
        })
        .collect()
}

#[cfg(test)]
const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[test]
fn day6_part1() {
    let races = parse_part1(TEST_INPUT);

    assert_eq!(count_ways_to_win(&races[0]), 4);
    assert_eq!(races.iter().map(count_ways_to_win).product::<usize>(), 288);
}
