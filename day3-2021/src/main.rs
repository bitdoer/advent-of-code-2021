use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let len = Iterator::count(input.trim().lines());
    let num_bits = input.trim().lines().next().unwrap().len();

    // Basic strategy for part 1:
    // 1. count the number of ones in each bit position
    // 2. if the number of ones is over half the number of total entries, then
    //    ones are the most common; otherwise, zeroes are
    // 3. reconstruct gamma from the bits
    // 4. since there are only two options for bits, epsilon is just 0x11...1 - gamma

    let mut one_count = vec![0usize; num_bits];

    // 1.
    for line in input.trim().lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                one_count[i] += 1;
            }
        }
    }

    let gamma = one_count
        .iter()
        // 2.
        .map(|&x| if x > len / 2 { 1 } else { 0 })
        // 3.
        .fold(0, |acc, x| acc * 2 + x);
    // 4.
    let epsilon = 2usize.pow(num_bits as u32) - 1 - gamma;

    let part1 = gamma * epsilon;
    println!("{}", part1);

    // Basic strategy for part 2:
    // 1. we've already done the counting for the first entry; use this count to inform what
    //    we're looking for
    // 2. pare down the list to only include those that contain the looked-for bit in the
    //    current bit position
    // 3. count the number of ones in the next bit position
    // 4. compare number of ones to half the length of the pared-down list to determine next
    //    looked for number
    // 5. repeat from 2 until only 1 entry
    // 6. reconstruct number from this entry
    // 7. repeat from 1 to get the other rating

    // 1.
    let mut looking_for = if one_count[0] * 2 >= len { '1' } else { '0' };
    let mut filtered = input.trim().lines().collect::<Vec<_>>();
    for i in 0..num_bits {
        // 2.
        filtered = filtered
            .iter()
            .cloned()
            .filter(|line| line.chars().nth(i).unwrap() == looking_for)
            .collect();

        // 5.
        if filtered.len() == 1 {
            break;
        }

        // 3.
        let mut count = 0;
        for &line in &filtered {
            if line.chars().nth(i + 1).unwrap() == '1' {
                count += 1;
            }
        }

        // 4. (for oxygen generator rating, we want the most common bit, and if there's a tie, it should be '1')
        looking_for = if count * 2 >= filtered.len() {
            '1'
        } else {
            '0'
        };
    }
    
    // 6.
    let generator_rating = filtered[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 2 + x);

    // 7.
    
    // 1.
    let mut looking_for = if one_count[0] * 2 >= len { '0' } else { '1' };
    let mut filtered = input.trim().lines().collect::<Vec<_>>();

    for i in 0..num_bits {
        // 2.
        filtered = filtered
            .iter()
            .cloned()
            .filter(|line| line.chars().nth(i).unwrap() == looking_for)
            .collect();

        // 5.
        if filtered.len() == 1 {
            break;
        }

        // 3.
        let mut count = 0;
        for &line in &filtered {
            if line.chars().nth(i + 1).unwrap() == '1' {
                count += 1;
            }
        }
        
        // 4. (for CO2 scrubber rating, we want the least common bit, and if there's a tie, it should be '0')
        looking_for = if count * 2 >= filtered.len() {
            '0'
        } else {
            '1'
        };
    }
    
    // 6.
    let scrubber_rating = filtered[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 2 + x);

    let part2 = generator_rating * scrubber_rating;
    println!("{}", part2);
}
