fn main() {
    let input = include_str!("input.txt");
    let grids = parse(input);

    println!("Part 1: {}", part1(&grids));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<bool>,
    width: usize,
}

impl Grid {
    fn get_reflection_position(&self) -> Reflection {
        let height = self.cells.len() / self.width;
        for x in 1..self.width {
            // Check if all cells to the left are the same as the ones to the right
            let mut all_same = true;
            for offset in 0..(self.width - x) {
                for y in 0..height {
                    let left = x.checked_sub(offset + 1);
                    let right = x + offset;
                    match (left, right) {
                        (None, _) => break,
                        (_, right) if right >= self.width => break,
                        (Some(left), right) => {
                            if self.cells[left + y * self.width]
                                != self.cells[right + y * self.width]
                            {
                                all_same = false;
                                break;
                            }
                        }
                    }
                }
            }

            if all_same {
                return Reflection::Vertical { before_column: x };
            }
        }

        for y in 1..height {
            // Check if all cells above are the same as the ones below
            let mut all_same = true;
            for offset in 0..(height - y) {
                for x in 0..self.width {
                    let top = y.checked_sub(offset + 1);
                    let bottom = y + offset;
                    match (top, bottom) {
                        (None, _) => break,
                        (_, bottom) if bottom >= height => break,
                        (Some(top), bottom) => {
                            // let top = top * self.width + x;
                            // let bottom = bottom * self.width + x;
                            if self.cells[top * self.width + x]
                                != self.cells[bottom * self.width + x]
                            {
                                all_same = false;
                                break;
                            }
                        }
                    }
                }
            }

            if all_same {
                return Reflection::Horizontal { before_row: y };
            }
        }

        panic!("No reflection found: {:#?}", self);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Reflection {
    Vertical { before_column: usize },
    Horizontal { before_row: usize },
}

fn parse(input: &str) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut current_grid = Grid {
        cells: Vec::new(),
        width: 0,
    };

    for line in input.lines() {
        if line.is_empty() {
            grids.push(current_grid);
            current_grid = Grid {
                cells: Vec::new(),
                width: 0,
            };
        } else {
            current_grid.cells.extend(line.chars().map(|c| c == '#'));
            current_grid.width = line.len();
        }
    }

    if !current_grid.cells.is_empty() {
        grids.push(current_grid);
    }

    grids
}

fn part1(grids: &[Grid]) -> usize {
    grids
        .iter()
        .map(|grid| grid.get_reflection_position())
        .map(|reflection| match reflection {
            Reflection::Vertical { before_column } => before_column,
            Reflection::Horizontal { before_row } => before_row * 100,
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn day13_part1() {
    let grids = parse(TEST_INPUT);

    assert_eq!(grids.len(), 2);
    assert_eq!(
        grids[0].get_reflection_position(),
        Reflection::Vertical { before_column: 5 }
    );
    assert_eq!(
        grids[1].get_reflection_position(),
        Reflection::Horizontal { before_row: 4 }
    );
    assert_eq!(part1(&grids), 405);
}
