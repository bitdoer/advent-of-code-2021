use std::{cmp::min, fs};

fn part1_deviation(parsed: &[i64], center: i64) -> i64 {
    let mut output = 0;
    for &entry in parsed {
        output += (entry - center).abs()
    }
    output
}

fn part2_deviation(parsed: &[i64], center: i64) -> i64 {
    let mut output = 0;
    for &entry in parsed {
        let n = (entry - center).abs();
        // the deviations described in the problem are exactly triangular
        // numbers---a distance of 3 gives 1 + 2 + 3 fuel, for example---
        // and the below formula gives the general amount of fuel for a
        // distance n
        output += (n * n + n) / 2;
    }
    output
}

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
    let median = if parsed.len() % 2 == 0 {
        (parsed[(parsed.len() / 2) - 1] + parsed[parsed.len() / 2]) as f64 / 2.0
    } else {
        parsed[parsed.len() / 2] as f64
    };
    // if the median is not a whole number, then the proper candidates to answer
    // this problem are the integers directly above and below it
    let part1 = if median.floor() == median {
        part1_deviation(&parsed, median as i64)
    } else {
        min(
            part1_deviation(&parsed, median.floor() as i64),
            part1_deviation(&parsed, median.ceil() as i64),
        )
    };
    println!("{}", part1);
    // part 2, dealing with a quadratic variation, is minimized instead by the mean;
    // since the mean is not available as such (being non-integral), we take a similar
    // tack, considering exactly those two integers immediately surrounding it as
    // candidates
    let mean = parsed.iter().sum::<i64>() as f64 / parsed.len() as f64;
    let part2 = min(
        part2_deviation(&parsed, mean.floor() as i64),
        part2_deviation(&parsed, mean.ceil() as i64),
    );
    println!("{}", part2);
}
