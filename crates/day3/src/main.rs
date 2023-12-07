use std::collections::HashSet;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));

    println!(
        "part 1: {}",
        grid.numbers_adjacent_to_symbols().sum::<u32>(),
    );
    println!("part 2: {}", grid.gear_ratios().sum::<u32>(),);
}

#[derive(Debug)]
struct Grid {
    numbers: Vec<u32>,
    grid_width: usize,
    grid: Vec<GridCell>,
}

#[derive(Debug)]
enum GridCell {
    Number(usize),
    Symbol(char),
    Empty,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut numbers = vec![];
        let mut lines = input.lines().peekable();
        let grid_width = lines.peek().map(|l| l.len()).unwrap_or(0);
        let mut grid = Vec::with_capacity(input.len());

        let mut number_buffer = String::new();

        for line in lines {
            for c in line.chars() {
                if c.is_ascii_digit() {
                    number_buffer.push(c);
                    grid.push(GridCell::Number(numbers.len()));
                } else {
                    if !number_buffer.is_empty() {
                        numbers.push(number_buffer.parse().unwrap());
                        number_buffer.clear();
                    }

                    grid.push(match c {
                        '.' => GridCell::Empty,
                        '*' | '+' | '#' | '$' | '-' | '&' | '@' | '/' | '=' | '%' => {
                            GridCell::Symbol(c)
                        }
                        _ => panic!("unexpected character: {}", c),
                    });
                }
            }
        }

        Self {
            numbers,
            grid_width,
            grid,
        }
    }

    fn numbers_adjacent_to_symbols(&self) -> impl Iterator<Item = u32> + '_ {
        let mut number_indices = HashSet::new();

        for (i, cell) in self.grid.iter().enumerate() {
            if let GridCell::Symbol(_) = cell {
                // Find the numbers adjacent to this symbol.
                for j in self.adjacent_indices(i) {
                    if let GridCell::Number(n) = self.grid[j] {
                        number_indices.insert(n);
                    }
                }
            }
        }

        number_indices.into_iter().map(move |i| self.numbers[i])
    }

    fn gear_ratios(&self) -> impl Iterator<Item = u32> + '_ {
        // Find all '*' cells that are adjacent to exactly two numbers. Multiply
        // those together to get the gear ratio.
        self.grid.iter().enumerate().filter_map(move |(i, cell)| {
            if let GridCell::Symbol('*') = cell {
                let adjacent_numbers: HashSet<usize> = self
                    .adjacent_indices(i)
                    .into_iter()
                    .filter_map(|j| {
                        if let GridCell::Number(n) = self.grid[j] {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .collect();

                if adjacent_numbers.len() == 2 {
                    Some(
                        adjacent_numbers
                            .into_iter()
                            .map(|n| self.numbers[n])
                            .product(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn adjacent_indices(&self, i: usize) -> Vec<usize> {
        let x = i % self.grid_width;
        let y = i / self.grid_width;

        let mut indices = Vec::with_capacity(8);

        if x > 0 {
            indices.push(i - 1);
        }

        if x < self.grid_width - 1 {
            indices.push(i + 1);
        }

        if y > 0 {
            indices.push(i - self.grid_width);

            if x > 0 {
                indices.push(i - self.grid_width - 1);
            }

            if x < self.grid_width - 1 {
                indices.push(i - self.grid_width + 1);
            }
        }

        if y < self.grid.len() / self.grid_width - 1 {
            indices.push(i + self.grid_width);

            if x > 0 {
                indices.push(i + self.grid_width - 1);
            }

            if x < self.grid_width - 1 {
                indices.push(i + self.grid_width + 1);
            }
        }

        indices
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn day3_part1() {
    let grid = Grid::parse(TEST_INPUT);

    assert_eq!(grid.numbers_adjacent_to_symbols().sum::<u32>(), 4361);
}

#[test]
fn day3_part2() {
    let grid = Grid::parse(TEST_INPUT);

    assert_eq!(grid.gear_ratios().sum::<u32>(), 467835);
}
