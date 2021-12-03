use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let len = Iterator::count(input.trim().lines());
    let num_bits = input.trim().lines().next().unwrap().len();
    let mut one_count = vec![0usize; num_bits];
    
    for line in input.trim().lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                one_count[i] += 1;
            }
        }
    }

    let gamma = one_count
        .iter()
        .map(|&x| if x > len / 2 { 1 } else { 0 })
        .fold(0, |acc, x| acc * 2 + x);
    let epsilon = 2usize.pow(num_bits as u32) - gamma - 1;

    let part1 = gamma * epsilon;
    println!("{}", part1);

    let mut looking_for = if one_count[0] * 2 >= len { '1' } else { '0' };
    let mut filtered = input.trim().lines().collect::<Vec<_>>();
    for i in 0..num_bits {
        filtered = filtered
            .iter()
            .cloned()
            .filter(|line| line.chars().nth(i).unwrap() == looking_for)
            .collect();
        if filtered.len() == 1 {
            break;
        }

        let mut count = 0;
        for &line in &filtered {
            if line.chars().nth(i + 1).unwrap() == '1' {
                count += 1;
            }
        }

        looking_for = if count * 2 >= filtered.len() {
            '1'
        } else {
            '0'
        };
    }

    let generator_rating = filtered[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 2 + x);

    let mut looking_for = if one_count[0] * 2 >= len { '0' } else { '1' };
    let mut filtered = input.trim().lines().collect::<Vec<_>>();

    for i in 0..num_bits {
        filtered = filtered
            .iter()
            .cloned()
            .filter(|line| line.chars().nth(i).unwrap() == looking_for)
            .collect();
        
        if filtered.len() == 1 {
            break;
        }
        
        let mut count = 0;
        for &line in &filtered {
            if line.chars().nth(i + 1).unwrap() == '1' {
                count += 1;
            }
        }
        
        looking_for = if count * 2 >= filtered.len() {
            '0'
        } else {
            '1'
        };
    }

    let scrubber_rating = filtered[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 2 + x);

    let part2 = generator_rating * scrubber_rating;
    println!("{}", part2);
}
