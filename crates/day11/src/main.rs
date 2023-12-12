fn main() {
    let input = include_str!("input.txt");
    let galaxies = parse(input, 2);

    println!(
        "Shortest distance pairs sum: {}",
        shortest_distance_pairs_sum(&galaxies)
    );

    let galaxies = parse(input, 1000000);
    println!(
        "Shortest distance pairs sum with more expansion: {}",
        shortest_distance_pairs_sum(&galaxies)
    );
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    position: (usize, usize),
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        ((self.position.0 as isize - other.position.0 as isize).abs() - 1
            + (self.position.1 as isize - other.position.1 as isize).abs()
            + 1) as usize
    }
}

#[allow(dead_code)]
fn parse(input: &str, expansion_ratio: usize) -> Vec<Galaxy> {
    let mut rows_with_galaxies = Vec::new();
    let mut columns_with_galaxies = Vec::new();
    let mut galaxies = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy { position: (x, y) });
                rows_with_galaxies.push(y);
                columns_with_galaxies.push(x);
            }
        }
    }

    // Apply expansion
    for galaxy in &mut galaxies {
        galaxy.position.0 += (0..galaxy.position.0)
            .filter(|x| !columns_with_galaxies.contains(x))
            .count()
            * (expansion_ratio - 1);
        galaxy.position.1 += (0..galaxy.position.1)
            .filter(|y| !rows_with_galaxies.contains(y))
            .count()
            * (expansion_ratio - 1);
    }

    galaxies
}

fn shortest_distance_pairs_sum(galaxies: &[Galaxy]) -> usize {
    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in &galaxies[i + 1..] {
            sum += galaxy.distance(other_galaxy);
        }
    }
    sum
}

#[cfg(test)]
const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn day11_part1() {
    let galaxies = parse(TEST_INPUT, 2);
    assert_eq!(galaxies.len(), 9);
    assert_eq!(galaxies[0].position, (4, 0));
    assert_eq!(galaxies[0].distance(&galaxies[6]), 15);

    assert_eq!(shortest_distance_pairs_sum(&galaxies), 374);
}

#[test]
fn day11_part2() {
    let galaxies = parse(TEST_INPUT, 10);
    assert_eq!(shortest_distance_pairs_sum(&galaxies), 1030);

    let galaxies = parse(TEST_INPUT, 100);
    assert_eq!(shortest_distance_pairs_sum(&galaxies), 8410);
}
