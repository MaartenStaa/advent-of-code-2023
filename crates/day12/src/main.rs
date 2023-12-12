use std::collections::HashMap;

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let mut records = parse(input);

    println!(
        "Number of possible states: {}",
        records
            .iter()
            .map(|record| record.get_possible_states_count())
            .sum::<usize>()
    );

    for record in records.iter_mut() {
        record.multiply(5);
    }

    println!(
        "Number of possible states with more expansion: {}",
        records
            .par_iter()
            .map(|record| record.get_possible_states_count())
            .sum::<usize>()
    );
}

fn parse(input: &str) -> Vec<Record> {
    input.lines().map(Record::parse).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParsedState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct Record {
    states: Vec<ParsedState>,
    contigious_damaged_counts: Vec<usize>,
}

impl Record {
    fn parse(line: &str) -> Self {
        let (states, counts) = line.split_once(' ').unwrap();

        Self {
            states: states
                .chars()
                .map(|c| match c {
                    '.' => ParsedState::Operational,
                    '#' => ParsedState::Damaged,
                    '?' => ParsedState::Unknown,
                    _ => unreachable!(),
                })
                .collect(),
            contigious_damaged_counts: counts.split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn multiply(&mut self, n: usize) {
        self.states = (0..n).fold(vec![], |mut acc, i| {
            acc.extend(self.states.iter().cloned());
            if i < n - 1 {
                acc.push(ParsedState::Unknown);
            }
            acc
        });
        self.contigious_damaged_counts = self
            .contigious_damaged_counts
            .iter()
            .cycle()
            .take(self.contigious_damaged_counts.len() * n)
            .copied()
            .collect();
    }

    fn get_possible_states_count(&self) -> usize {
        fn get_possible_states_count_2_inner(
            cached_results: &mut HashMap<(Vec<ParsedState>, Vec<usize>, bool), usize>,
            states: &[ParsedState],
            contigious_damaged_counts: &[usize],
            previous_was_damaged: bool,
            damaged_groups: &[usize],
        ) -> usize {
            let key = (
                states.to_vec(),
                damaged_groups.to_vec(),
                previous_was_damaged,
            );
            if let Some(result) = cached_results.get(&key) {
                return *result;
            }

            let mut result = states.is_empty().then(|| {
                if damaged_groups == contigious_damaged_counts {
                    1
                } else {
                    0
                }
            });

            result = result.or_else(|| {
                if damaged_groups.len() > contigious_damaged_counts.len()
                    || (!previous_was_damaged
                        && damaged_groups != &contigious_damaged_counts[..damaged_groups.len()])
                {
                    Some(0)
                } else {
                    None
                }
            });

            let result = result.unwrap_or_else(|| {
                let mut result = 0;
                if matches!(states[0], ParsedState::Operational | ParsedState::Unknown) {
                    result += get_possible_states_count_2_inner(
                        cached_results,
                        &states[1..],
                        contigious_damaged_counts,
                        false,
                        damaged_groups,
                    );
                }

                // No else if, allow unknown to enter both clauses
                if matches!(states[0], ParsedState::Damaged | ParsedState::Unknown) {
                    let mut damaged_groups = damaged_groups.to_vec();

                    result += if previous_was_damaged {
                        let index = damaged_groups.len() - 1;
                        damaged_groups[index] += 1;
                        get_possible_states_count_2_inner(
                            cached_results,
                            &states[1..],
                            contigious_damaged_counts,
                            true,
                            &damaged_groups,
                        )
                    } else {
                        damaged_groups.push(1);
                        get_possible_states_count_2_inner(
                            cached_results,
                            &states[1..],
                            contigious_damaged_counts,
                            true,
                            &damaged_groups,
                        )
                    };
                }

                result
            });

            cached_results.insert(key, result);
            result
        }

        get_possible_states_count_2_inner(
            &mut HashMap::new(),
            &self.states,
            &self.contigious_damaged_counts,
            false,
            &[],
        )
    }
}

#[test]
fn day12_part1() {
    let records = parse(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    );
    assert_eq!(records.len(), 6);
    assert_eq!(records[0].get_possible_states_count(), 1);
    assert_eq!(records[1].get_possible_states_count(), 4);
    assert_eq!(records[2].get_possible_states_count(), 1);
    assert_eq!(records[3].get_possible_states_count(), 1);
    assert_eq!(records[4].get_possible_states_count(), 4);
    assert_eq!(records[5].get_possible_states_count(), 10);
}

#[test]
fn day12_part2() {
    let mut records = parse(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    );
    for record in records.iter_mut() {
        record.multiply(5);
    }

    assert_eq!(records[0].get_possible_states_count(), 1);
    assert_eq!(records[1].get_possible_states_count(), 16384);
    assert_eq!(records[2].get_possible_states_count(), 1);
    assert_eq!(records[3].get_possible_states_count(), 16);
    assert_eq!(records[4].get_possible_states_count(), 2500);
    assert_eq!(records[5].get_possible_states_count(), 506250);
}
