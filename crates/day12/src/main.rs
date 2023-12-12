fn main() {
    let input = include_str!("input.txt");
    let records = parse(input);

    println!(
        "Number of possible states: {}",
        records
            .iter()
            .map(|record| record.get_possible_states().len())
            .sum::<usize>()
    );
}

fn parse(input: &str) -> Vec<Record> {
    input.lines().map(Record::parse).collect()
}

#[derive(Debug, PartialEq)]
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

    fn get_possible_states(&self) -> Vec<Vec<State>> {
        #[derive(Debug, Clone)]
        struct PossibleState {
            states: Vec<State>,
            damaged_groups: Vec<usize>,
            previous_was_damaged: bool,
        }

        let mut possible_states = vec![PossibleState {
            states: Vec::with_capacity(self.states.len()),
            damaged_groups: vec![],
            previous_was_damaged: false,
        }];
        for s in &self.states {
            dbg!(s);
            match s {
                ParsedState::Operational => possible_states.iter_mut().for_each(|state| {
                    if state.previous_was_damaged {
                        state.previous_was_damaged = false;
                    }
                    state.states.push(State::Operational);
                }),
                ParsedState::Damaged => possible_states.iter_mut().for_each(|state| {
                    if state.previous_was_damaged {
                        let index = state.damaged_groups.len() - 1;
                        state.damaged_groups[index] += 1;
                    } else {
                        state.damaged_groups.push(1);
                        state.previous_was_damaged = true;
                    }
                    state.states.push(State::Damaged);
                }),
                ParsedState::Unknown => {
                    // Branch
                    let damaged_state = possible_states.clone();

                    // Always safe to add an operational state
                    possible_states.iter_mut().for_each(|state| {
                        if state.previous_was_damaged {
                            state.previous_was_damaged = false;
                        }
                        state.states.push(State::Operational);
                    });

                    // Add a damaged state to the damaged state branch, but only if:
                    // - It would not exceed the contigious damaged count
                    // - We have not yet exceeded the blocks of contigious damaged counts
                    for mut state in damaged_state.into_iter().filter(|state| {
                        if state.previous_was_damaged {
                            state.damaged_groups.len() <= self.contigious_damaged_counts.len()
                                && state.damaged_groups[state.damaged_groups.len() - 1]
                                    < self.contigious_damaged_counts[state.damaged_groups.len() - 1]
                        } else {
                            // Need to start a new group, so we need to make sure we have not
                            // exceeded the number of groups
                            state.damaged_groups.len() < self.contigious_damaged_counts.len()
                        }
                    }) {
                        if state.previous_was_damaged {
                            let index = state.damaged_groups.len() - 1;
                            state.damaged_groups[index] += 1;
                        } else {
                            state.damaged_groups.push(1);
                            state.previous_was_damaged = true;
                        }
                        state.states.push(State::Damaged);

                        possible_states.push(state);
                    }
                }
            }
        }

        possible_states
            .into_iter()
            .filter_map(|state| {
                if state.damaged_groups == self.contigious_damaged_counts {
                    Some(state.states)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Operational,
    Damaged,
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
    assert_eq!(records[0].get_possible_states().len(), 1);
    assert_eq!(records[1].get_possible_states().len(), 4);
    assert_eq!(records[2].get_possible_states().len(), 1);
    assert_eq!(records[3].get_possible_states().len(), 1);
    assert_eq!(records[4].get_possible_states().len(), 4);
    assert_eq!(records[5].get_possible_states().len(), 10);
}
