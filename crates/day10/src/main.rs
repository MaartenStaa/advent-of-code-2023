use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::parse(input);

    println!(
        "Furthest point from start: {}",
        grid.find_loop_furthest_point()
    );
    println!(
        "Number of cells enclosed in loop: {}",
        grid.find_num_cells_enclosed_in_loop()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Empty,      // .
    Start,      // S
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Cell {
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    fn connects_south(&self) -> bool {
        matches!(
            self,
            Self::NorthSouth | Self::SouthWest | Self::SouthEast | Self::Start
        )
    }

    fn connects_north(&self) -> bool {
        matches!(
            self,
            Self::NorthSouth | Self::NorthEast | Self::NorthWest | Self::Start
        )
    }

    fn connects_east(&self) -> bool {
        matches!(
            self,
            Self::EastWest | Self::NorthEast | Self::SouthEast | Self::Start
        )
    }

    fn connects_west(&self) -> bool {
        matches!(
            self,
            Self::EastWest | Self::NorthWest | Self::SouthWest | Self::Start
        )
    }

    fn next_direction_from(&self, previous_direction: Direction) -> Direction {
        match (self, previous_direction) {
            (Self::NorthSouth, Direction::North) => Direction::South,
            (Self::NorthSouth, Direction::South) => Direction::North,
            (Self::EastWest, Direction::East) => Direction::West,
            (Self::EastWest, Direction::West) => Direction::East,
            (Self::NorthEast, Direction::North) => Direction::East,
            (Self::NorthEast, Direction::East) => Direction::North,
            (Self::NorthWest, Direction::North) => Direction::West,
            (Self::NorthWest, Direction::West) => Direction::North,
            (Self::SouthWest, Direction::South) => Direction::West,
            (Self::SouthWest, Direction::West) => Direction::South,
            (Self::SouthEast, Direction::South) => Direction::East,
            (Self::SouthEast, Direction::East) => Direction::South,
            _ => panic!("Invalid direction ({:?} -> {:?})", self, previous_direction),
        }
    }
}

#[derive(Debug)]
struct CellGrid {
    cell: Cell,
    index: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<CellGrid>,
    start_index: usize,
    width: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut start_index = 0;
        let mut width = 0;
        for line in input.lines() {
            width = line.len();
            for c in line.chars() {
                let index = cells.len();
                cells.push(match c {
                    '|' => CellGrid {
                        cell: Cell::NorthSouth,
                        index,
                    },
                    '-' => CellGrid {
                        cell: Cell::EastWest,
                        index,
                    },
                    'L' => CellGrid {
                        cell: Cell::NorthEast,
                        index,
                    },
                    'J' => CellGrid {
                        cell: Cell::NorthWest,
                        index,
                    },
                    '7' => CellGrid {
                        cell: Cell::SouthWest,
                        index,
                    },
                    'F' => CellGrid {
                        cell: Cell::SouthEast,
                        index,
                    },
                    '.' => CellGrid {
                        cell: Cell::Empty,
                        index,
                    },
                    'S' => {
                        start_index = index;
                        CellGrid {
                            cell: Cell::Start,
                            index,
                        }
                    }
                    _ => panic!("Invalid character"),
                });
            }
        }

        Self {
            cells,
            start_index,
            width,
        }
    }

    fn find_loop_furthest_point(&self) -> usize {
        (self.find_loop().len() + 1) / 2
    }

    fn find_num_cells_enclosed_in_loop(&self) -> usize {
        let loop_ = self.find_loop();
        let mut num_enclosed = 0;
        let mut inside_loop = false;
        for cell in &self.cells {
            if loop_.contains(&cell.index) {
                // Flip inside loop if crossing a vertical line.
                match cell.cell {
                    Cell::NorthSouth | Cell::NorthEast | Cell::NorthWest => {
                        inside_loop = !inside_loop;
                    }
                    _ => {}
                }
            } else if inside_loop {
                num_enclosed += 1;
            }
        }

        num_enclosed
    }

    fn find_loop(&self) -> HashSet<usize> {
        // We know where we start, but not in which direction we need to go.
        // We can try all four directions and see which one leads us back to the
        // start, checking if the tiles connect with us.
        for &direction in &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            // Find the cell in that direction.
            let Some(connecting_cell) = self.get_cell(self.start_index, direction) else {
                continue;
            };

            // Ensure we can go in that direction (the cell has a pipe that connects this way).
            match direction {
                Direction::North if !connecting_cell.cell.connects_south() => {
                    continue;
                }
                Direction::East if !connecting_cell.cell.connects_west() => {
                    continue;
                }
                Direction::South if !connecting_cell.cell.connects_north() => {
                    continue;
                }
                Direction::West if !connecting_cell.cell.connects_east() => {
                    continue;
                }
                _ => {}
            }

            // Find the loop from that cell.
            if let Some(loop_) = self.find_loop_from(connecting_cell.index, direction.opposite()) {
                return loop_;
            }
        }

        panic!("No loop found");
    }

    fn find_loop_from(
        &self,
        start_index: usize,
        mut previous_direction: Direction,
    ) -> Option<HashSet<usize>> {
        let mut current_index = start_index;
        let mut loop_ = HashSet::new();
        loop_.insert(current_index);

        loop {
            // What direction are we going?
            let current_cell = &self.cells[current_index];
            let direction = current_cell.cell.next_direction_from(previous_direction);

            // Find the cell in that direction.
            let Some(connecting_cell) = self.get_cell(current_index, direction) else {
                return None;
            };

            // Ensure we can go in that direction (the cell has a pipe that connects this way).
            match direction {
                Direction::North if !connecting_cell.cell.connects_south() => {
                    return None;
                }
                Direction::East if !connecting_cell.cell.connects_west() => {
                    return None;
                }
                Direction::South if !connecting_cell.cell.connects_north() => {
                    return None;
                }
                Direction::West if !connecting_cell.cell.connects_east() => {
                    return None;
                }
                _ => {}
            }

            // Add the cell to the loop.
            loop_.insert(connecting_cell.index);

            // Check if we've reached the start again.
            if connecting_cell.cell.is_start() {
                return Some(loop_);
            }

            // Continue in the same direction.
            current_index = connecting_cell.index;
            previous_direction = direction.opposite();
        }
    }

    fn get_cell(&self, from: usize, direction: Direction) -> Option<&CellGrid> {
        let index = match direction {
            Direction::North => from.checked_sub(self.width),
            Direction::East => {
                if (from + 1) % self.width == 0 {
                    None
                } else {
                    Some(from + 1)
                }
            }
            Direction::South => from.checked_add(self.width),
            Direction::West => {
                if from % self.width == 0 {
                    None
                } else {
                    Some(from - 1)
                }
            }
        };

        index.map(|index| &self.cells[index])
    }
}

#[test]
fn day10_part1() {
    let grid = Grid::parse(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    );
    assert_eq!(grid.find_loop_furthest_point(), 8);
}

#[test]
fn day10_part2() {
    let grid = Grid::parse(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
    );
    assert_eq!(grid.find_num_cells_enclosed_in_loop(), 4);

    let grid = Grid::parse(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    );
    assert_eq!(grid.find_num_cells_enclosed_in_loop(), 10);
}
