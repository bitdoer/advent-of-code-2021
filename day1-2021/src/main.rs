use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let parsed = input
        .trim()
        .lines()
        .map(|n| n.parse::<u64>().expect("Failed to parse"))
        .collect::<Vec<_>>();
    let mut part1 = 0;
    for pair in parsed.windows(2) {
        if pair[1] > pair[0] {
            part1 += 1;
        }
    }

    println!("{}", part1);

    let mut part2 = 0;
    for quad in parsed.windows(4) {
        if quad[3] > quad[0] {
            part2 += 1;
        }
    }

    println!("{}", part2);
}
