use std::{
    cmp::{max, min},
    fs,
};

enum Part {
    One,
    Two,
}

fn find_answer(input: &str, part: Part) -> usize {
    let mut ocean_floor = vec![vec![0usize; 1000]; 1000];
    for line in input.trim().lines() {
        let mut first_pair = line.split(" -> ").next().unwrap().split(',');
        let x1 = first_pair.next().unwrap().parse::<usize>().unwrap();
        let y1 = first_pair.next().unwrap().parse::<usize>().unwrap();
        let mut second_pair = line.split(" -> ").nth(1).unwrap().split(',');
        let x2 = second_pair.next().unwrap().parse::<usize>().unwrap();
        let y2 = second_pair.next().unwrap().parse::<usize>().unwrap();
        if x1 == x2 {
            for i in min(y1, y2)..=max(y1, y2) {
                ocean_floor[x1][i] += 1;
            }
        } else if y1 == y2 {
            #[allow(clippy::needless_range_loop)]
            for i in min(x1, x2)..=max(x1, x2) {
                ocean_floor[i][y1] += 1;
            }
        } else if let Part::Two = part {
            if x1 < x2 && y1 < y2 {
                for i in 0..=(x2 - x1) {
                    ocean_floor[x1 + i][y1 + i] += 1;
                }
            } else if x1 < x2 && y1 > y2 {
                for i in 0..=(x2 - x1) {
                    ocean_floor[x1 + i][y1 - i] += 1;
                }
            } else if x1 > x2 && y1 < y2 {
                for i in 0..=(x1 - x2) {
                    ocean_floor[x1 - i][y1 + i] += 1;
                }
            } else {
                for i in 0..=(x1 - x2) {
                    ocean_floor[x1 - i][y1 - i] += 1;
                }
            }
        }
    }
    ocean_floor
        .into_iter()
        .map(|x| x.into_iter().filter(|&x| x > 1).count())
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    println!("{}", find_answer(&input, Part::One));
    println!("{}", find_answer(&input, Part::Two));
}
