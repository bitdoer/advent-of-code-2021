use std::fs;

// this function assumes what is guaranteed in the problem itself---
// namely, that each string contains only one instance of each character
fn count_overlap(a: &str, b: &str) -> usize {
    a.chars().map(|ch| b.contains(ch)).filter(|&x| x).count()
}

fn to_digits(all_digits: &[String]) -> Vec<usize> {
    // basic tactic: use 4 and 7 to pattern-match on the other digits
    let mut four_pattern = String::new();
    let mut seven_pattern = String::new();
    for digit in all_digits {
        match digit.len() {
            3 => seven_pattern = digit.clone(),
            4 => four_pattern = digit.clone(),
            _ => {}
        }
    }
    all_digits
        .iter()
        .map(|x| match x.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                // of 5-segment digits (2, 3, 5), only 3 completely contains 7's segments
                if count_overlap(x, &seven_pattern) == 3 {
                    3
                // of the remaining, only 3 has an overlap of 3 segments with 4
                } else if count_overlap(x, &four_pattern) == 3 {
                    5
                } else {
                    2
                }
            }
            6 => {
                // of 6-segment digits (0, 6, 9), only 9 completely contains 4's segments
                if count_overlap(x, &four_pattern) == 4 {
                    9
                // of the remaining, only 0 completely contains 7's segments
                } else if count_overlap(x, &seven_pattern) == 3 {
                    0
                } else {
                    6
                }
            }
            7 => 8,
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut part1 = 0;
    for line in input.trim().lines() {
        for digit in line.split('|').nth(1).unwrap().trim().split_whitespace() {
            match digit.len() {
                2 | 3 | 4 | 7 => part1 += 1,
                _ => {}
            }
        }
    }
    println!("{}", part1);
    let mut part2: usize = 0;
    for line in input.trim().lines() {
        let all_digits = line
            .split('|')
            .flat_map(|x| x.split_whitespace())
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        part2 += to_digits(&all_digits)
            .into_iter()
            .skip(10) // output digits are only the last 4
            .fold(0, |acc, x| acc * 10 + x); // classic way to roll digits into single number
    }
    println!("{}", part2);
}
