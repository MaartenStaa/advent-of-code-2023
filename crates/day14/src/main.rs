fn main() {
    let input = include_str!("input.txt");
    let mut grid = parse(input);
    grid.roll_round_stones_north();

    println!("Part 1: {}", grid.total_load());
}

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    RoundStone,
    SquareStone,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
}

impl Grid {
    fn roll_round_stones_north(&mut self) {
        for i in 0..self.cells.len() {
            if self.cells[i] != Cell::RoundStone {
                continue;
            }

            let x = i % self.width;
            let y = i / self.width;
            let mut target_index = None;

            for offset in 1..=y {
                let index = x + (y - offset) * self.width;
                match self.cells[index] {
                    Cell::Empty => {
                        target_index = Some(index);
                    }
                    Cell::RoundStone | Cell::SquareStone => break,
                }
            }

            if let Some(target_index) = target_index {
                self.cells.swap(i, target_index);
            }
        }
    }

    fn total_load(&self) -> usize {
        let height = self.cells.len() / self.width;

        (0..height)
            .flat_map(|y| {
                let rock_weight = height - y;

                (0..self.width).map(move |x| {
                    let index = x + y * self.width;

                    match self.cells[index] {
                        Cell::RoundStone => rock_weight,
                        Cell::SquareStone => 0,
                        Cell::Empty => 0,
                    }
                })
            })
            .sum()
    }
}

fn parse(input: &str) -> Grid {
    let mut width = 0;

    let cells = input
        .lines()
        .flat_map(|line| {
            width = line.len();
            line.chars()
        })
        .map(|c| match c {
            '.' => Cell::Empty,
            'O' => Cell::RoundStone,
            '#' => Cell::SquareStone,
            _ => panic!("Invalid character"),
        })
        .collect();

    Grid { cells, width }
}

#[cfg(test)]
const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn day14_part1() {
    let mut grid = parse(TEST_INPUT);
    grid.roll_round_stones_north();

    assert_eq!(grid.total_load(), 136);
}
