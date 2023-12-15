fn main() {
    let input = include_str!("input.txt").trim_end();

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(|s| hash(s) as usize).sum()
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
