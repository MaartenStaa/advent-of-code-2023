use std::collections::HashSet;

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::parse(input);

    let energized_fields = grid.simulate_part1();
    println!("Part 1: {}", energized_fields.len());
    println!("Part 2: {}", grid.simulate_part2());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn move_(self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    VerticalSplit,
    HorizontalSplit,
    LeftAngleMirror,
    RightAngleMirror,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let cells = input
            .lines()
            .flat_map(|line| {
                width = line.len();
                line.chars().map(|c| match c {
                    '.' => Cell::Empty,
                    '|' => Cell::VerticalSplit,
                    '-' => Cell::HorizontalSplit,
                    '\\' => Cell::LeftAngleMirror,
                    '/' => Cell::RightAngleMirror,
                    _ => panic!("Invalid character: {}", c),
                })
            })
            .collect();

        Self { cells, width }
    }

    fn simulate_part1(&self) -> HashSet<Position> {
        self.simulate(Beam {
            position: Position { x: 0, y: 0 },
            direction: Direction::Right,
        })
    }

    fn simulate_part2(&self) -> usize {
        // Find the starting beam that will go through the most cells
        let height = self.cells.len() / self.width;

        // Top row going down
        (0..self.width)
            .into_par_iter()
            .map(|x| Beam {
                position: Position { x, y: 0 },
                direction: Direction::Down,
            })
            // Left column going right
            .chain((0..height).into_par_iter().map(|y| Beam {
                position: Position { x: 0, y },
                direction: Direction::Right,
            }))
            // Bottom row going up
            .chain((0..self.width).into_par_iter().map(|x| Beam {
                position: Position { x, y: height - 1 },
                direction: Direction::Up,
            }))
            // Right column going left
            .chain((0..height).into_par_iter().map(|y| Beam {
                position: Position {
                    x: self.width - 1,
                    y,
                },
                direction: Direction::Left,
            }))
            .map(|beam| self.simulate(beam).len())
            .max()
            .unwrap()
    }

    fn simulate(&self, initial_beam: Beam) -> HashSet<Position> {
        let mut energized_fields = HashSet::new();
        let mut beams_seen = HashSet::new();
        let mut beams = vec![initial_beam];

        let mut beams_to_remove = vec![];
        let height = self.cells.len() / self.width;
        loop {
            let beam_count = beams.len();
            if beam_count == 0 {
                break;
            }

            for i in 0..beam_count {
                let beam = &mut beams[i];

                // If we've already seen a beam in this position, going in the same direction, remove it
                // as continuing to simulate it would be pointless
                if !beams_seen.insert((beam.position, beam.direction)) {
                    beams_to_remove.push(i);
                    continue;
                }

                energized_fields.insert(beam.position);

                let cell = self.cells[beam.position.y * self.width + beam.position.x];
                let mut continue_in = |beam: &mut Beam, direction: Direction| {
                    let new_position = beam.position.move_(direction);
                    match new_position {
                        Some(new_position)
                            if new_position.x < self.width && new_position.y < height =>
                        {
                            beam.position = new_position;
                            beam.direction = direction;
                        }
                        _ => {
                            beams_to_remove.push(i);
                        }
                    }
                };
                match cell {
                    Cell::Empty => {
                        continue_in(beam, beam.direction);
                    }
                    Cell::VerticalSplit => {
                        // If going right or left, split into two beams going up and down
                        // If going up or down, pass through
                        match beam.direction {
                            Direction::Up | Direction::Down => {
                                continue_in(beam, beam.direction);
                            }
                            Direction::Left | Direction::Right => {
                                let position = beam.position;
                                continue_in(beam, Direction::Up);
                                match position.move_(Direction::Down) {
                                    Some(new_position)
                                        if new_position.x < self.width
                                            && new_position.y < height =>
                                    {
                                        beams.push(Beam {
                                            position: new_position,
                                            direction: Direction::Down,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Cell::HorizontalSplit => {
                        // If going up or down, split into two beams going left and right
                        // If going left or right, pass through
                        match beam.direction {
                            Direction::Left | Direction::Right => {
                                continue_in(beam, beam.direction);
                            }
                            Direction::Up | Direction::Down => {
                                let position = beam.position;
                                continue_in(beam, Direction::Left);
                                match position.move_(Direction::Right) {
                                    Some(new_position)
                                        if new_position.x < self.width
                                            && new_position.y < height =>
                                    {
                                        beams.push(Beam {
                                            position: new_position,
                                            direction: Direction::Right,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Cell::LeftAngleMirror => {
                        let new_direction = match beam.direction {
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Up => Direction::Left,
                        };
                        continue_in(beam, new_direction);
                    }
                    Cell::RightAngleMirror => {
                        let new_direction = match beam.direction {
                            Direction::Right => Direction::Up,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Up => Direction::Right,
                        };
                        continue_in(beam, new_direction);
                    }
                }
            }

            for i in beams_to_remove.drain(..).rev() {
                beams.swap_remove(i);
            }
        }

        energized_fields
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

#[test]
fn day16_part1() {
    let grid = Grid::parse(TEST_INPUT);
    let energized_fields = grid.simulate_part1();
    assert_eq!(energized_fields.len(), 46);
}

#[test]
fn day16_part2() {
    let grid = Grid::parse(TEST_INPUT);
    let energized_fields = grid.simulate_part2();
    assert_eq!(energized_fields, 51);
}
