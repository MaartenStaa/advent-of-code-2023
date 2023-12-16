fn main() {
    let input = include_str!("input.txt").trim_end();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(|s| hash(s) as usize).sum()
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    name: &'a str,
    focal_length: usize,
}

fn part2(input: &str) -> usize {
    // let mut lens_positions = HashMap::new();
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for s in input.split(',') {
        if let Some(name) = s.strip_suffix('-') {
            // Remove that one.
            let box_id = hash(name) as usize;
            let b: &mut Vec<Lens> = boxes.get_mut(box_id).unwrap();
            b.retain(|l| l.name != name);
        } else if let Some((name, focal_length)) = s.split_once('=') {
            let box_id = hash(name) as usize;
            let strength = focal_length.parse().unwrap();
            let b: &mut Vec<Lens> = boxes.get_mut(box_id).unwrap();

            if let Some(existing_lens) = b.iter_mut().find(|l| l.name == name) {
                existing_lens.focal_length = strength;
            } else {
                let lens = Lens {
                    name,
                    focal_length: strength,
                };
                b.push(lens);
            }
        } else {
            panic!("Invalid input: {}", s);
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_id, b)| {
            b.iter()
                .enumerate()
                .map(move |(n, lens)| (1 + box_id) * (n + 1) * lens.focal_length)
        })
        .sum()
}

fn hash(input: &str) -> u8 {
    let mut hash: usize = 0;
    for c in input.bytes() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }

    hash as u8
}

#[test]
fn day15_part1() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(
        part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    )
}

#[test]
fn day15_part2() {
    assert_eq!(
        part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    )
}
