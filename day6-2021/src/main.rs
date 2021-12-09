use std::fs;

fn update(lanternfish: &mut [u64]) {
    // this effectively "ticks down all counters" with rollover; the rollover handles
    // each fish with a timer at 0 giving birth to a fish with a timer at 8.
    // however, the problem says that every fish that's at 0 gets its timer
    // reset to 6---this line does not handle that
    lanternfish.rotate_left(1);
    // this line does! the value in lanternfish[8] is exactly identical to the value that
    // used to be in lanternfish[0], so this does exactly what we want
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
