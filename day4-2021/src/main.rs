use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Unmarked(u8),
    Marked(u8),
}

impl Square {
    fn is_marked(&self) -> bool {
        matches!(self, Self::Marked(_))
    }

    fn num(&self) -> u64 {
        match self {
            &Self::Marked(n) | &Self::Unmarked(n) => n as u64,
        }
    }
}

#[derive(Debug)]
struct Bingo {
    card: [Square; 25],
    bingoed: bool,
}

impl Bingo {
    fn new(input: &str) -> Self {
        let mut card = [Square::Unmarked(0); 25];
        for (i, num) in input.trim().split_whitespace().enumerate() {
            card[i] = Square::Unmarked(num.parse().unwrap());
        }
        Self {
            card,
            bingoed: false,
        }
    }

    fn call(&mut self, n: u8) -> Option<u64> {
        for square in self.card.iter_mut() {
            if *square == Square::Unmarked(n) {
                *square = Square::Marked(n);
            }
        }
        self.bingo()
    }

    fn bingo(&mut self) -> Option<u64> {
        if self.row_marked(0)
            || self.row_marked(1)
            || self.row_marked(2)
            || self.row_marked(3)
            || self.row_marked(4)
            || self.column_marked(0)
            || self.column_marked(1)
            || self.column_marked(2)
            || self.column_marked(3)
            || self.column_marked(4)
            || self.diag_marked()
        {
            self.bingoed = true;
            Some(
                self.card
                    .iter()
                    .filter(|x| !x.is_marked())
                    .map(|x| x.num())
                    .sum(),
            )
        } else {
            None
        }
    }

    fn row_marked(&self, n: usize) -> bool {
        self.card[n].is_marked()
            && self.card[n + 1].is_marked()
            && self.card[n + 2].is_marked()
            && self.card[n + 3].is_marked()
            && self.card[n + 4].is_marked()
    }

    fn column_marked(&self, n: usize) -> bool {
        self.card[n].is_marked()
            && self.card[n + 5].is_marked()
            && self.card[n + 10].is_marked()
            && self.card[n + 15].is_marked()
            && self.card[n + 20].is_marked()
    }

    fn diag_marked(&self) -> bool {
        (self.card[0].is_marked()
            && self.card[6].is_marked()
            && self.card[12].is_marked()
            && self.card[18].is_marked()
            && self.card[24].is_marked())
            || (self.card[0].is_marked()
                && self.card[6].is_marked()
                && self.card[12].is_marked()
                && self.card[18].is_marked()
                && self.card[24].is_marked())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let calls = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut bingo_cards = vec![];
    for card_str in input.split("\n\n").skip(1) {
        bingo_cards.push(Bingo::new(card_str));
    }
    let mut part1 = 0;
    'outer1: for &call in &calls {
        for bingo_card in bingo_cards.iter_mut() {
            if let Some(score) = bingo_card.call(call) {
                part1 = score * call as u64;
                break 'outer1;
            }
        }
    }
    println!("{}", part1);

    let mut bingo_cards = vec![];
    for card_str in input.split("\n\n").skip(1) {
        bingo_cards.push(Bingo::new(card_str));
    }
    let mut part2 = 0;
    'outer2: for &call in &calls {
        bingo_cards = bingo_cards.into_iter().filter(|x| !x.bingoed).collect();
        let num_left = bingo_cards.len();
        for bingo_card in bingo_cards.iter_mut() {
            if let Some(score) = bingo_card.call(call) {
                if num_left == 1 {
                    part2 = score * call as u64;
                    break 'outer2;
                }
            }
        }
    }

    println!("{}", part2);
}
