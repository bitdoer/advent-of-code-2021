use std::fs;

fn update(lanternfish: &mut [u64]) {
    lanternfish.rotate_left(1);
    lanternfish[6] += lanternfish[8];
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read to string");
    let mut lanternfish = vec![0u64; 9];
    for i in input.trim().split(',') {
        lanternfish[i.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..80 {
        update(&mut lanternfish);
    }

    let part1 = lanternfish.iter().sum::<u64>();
    println!("{}", part1);

    for _ in 80..256 {
        update(&mut lanternfish);
    }

    let part2 = lanternfish.iter().sum::<u64>();
    println!("{}", part2);
}
