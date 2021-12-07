use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut parsed = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    parsed.sort_unstable();
    // the median is exactly the number which has minimal average deviation from the
    // elements of a data set, and therefore minimal total deviation
    //
    // if the median is not a whole number, then the proper candidates are exactly
    // 
    let median = if parsed.len() % 2 == 0 {
        (parsed[(parsed.len() / 2) - 1] + parsed[parsed.len() / 2]) / 2
    } else {
        parsed[parsed.len() / 2]
    };
    let mut part1 = 0;
    for &entry in &parsed {
        part1 += (entry - median).abs();
    }
    println!("{}", part1);
    // part 2 is more subtle
}
