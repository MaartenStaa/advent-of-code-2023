use chumsky::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let network = Network::parser().parse(input).unwrap();

    println!("Number of steps: {}", count_steps(&network));
}

fn count_steps(network: &Network) -> usize {
    let mut current_node = network
        .nodes
        .get("AAA")
        .expect("All nodes must be reachable");

    network
        .directions
        .iter().cycle()
        .take_while(|direction| {
            let destination = match direction {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right,
            };
            current_node = network
                .nodes
                .get(destination)
                .expect("All nodes must be reachable");

            destination != "ZZZ"
        })
        .count()
        // Add 1 to account for the final step to ZZZ
        + 1
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Network {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        let direction = choice([
            just('L').to(Direction::Left),
            just('R').to(Direction::Right),
        ])
        .labelled("direction");
        let node_name =
            filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                .repeated()
                .exactly(3)
                .collect::<String>();

        let node = node_name
            .labelled("source")
            .then_ignore(just(',').padded())
            .then(node_name)
            .labelled("destination")
            .delimited_by(just('('), just(')'))
            .map(|(left, right)| Node { left, right })
            .boxed();

        direction
            .repeated()
            .then_ignore(text::newline().repeated().exactly(2))
            .then(
                node_name
                    .labelled("node name")
                    .then_ignore(just('=').padded())
                    .then(node)
                    .separated_by(text::newline())
                    .collect::<HashMap<_, _>>(),
            )
            .then_ignore(text::newline().repeated())
            .then_ignore(end())
            .map(|(directions, nodes)| Network { directions, nodes })
    }
}

#[cfg(test)]
const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[cfg(test)]
const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[test]
fn day8_part1() {
    let network = Network::parser().parse(TEST_INPUT_1).unwrap();
    assert_eq!(count_steps(&network), 2);

    let network = Network::parser().parse(TEST_INPUT_2).unwrap();
    assert_eq!(count_steps(&network), 6);
}
