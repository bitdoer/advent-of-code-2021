use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut depth = 0;
    let mut forward_pos = 0;
    for line in input
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
    {
        let n = line[1].parse::<u64>().expect("Failed to parse to integer");
        match line[0] {
            "forward" => forward_pos += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => unreachable!(),
        }
    }
    let part1 = depth * forward_pos;
    println!("{}", part1);
    depth = 0;
    forward_pos = 0;
    let mut aim = 0;
    for line in input
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
    {
        let n = line[1].parse::<u64>().expect("Failed to parse to integer");
        match line[0] {
            "forward" => {
                forward_pos += n;
                depth += n * aim;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => unreachable!(),
        }
    }
    let part2 = depth * forward_pos;
    println!("{}", part2);
}
