use std::fs;

struct ParseBracketsError(Bracket);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Bracket {
    Paren,
    Square,
    Curly,
    Angle,
}

impl From<char> for Bracket {
    fn from(ch: char) -> Self {
        match ch {
            '(' | ')' => Self::Paren,
            '[' | ']' => Self::Square,
            '{' | '}' => Self::Curly,
            '<' | '>' => Self::Angle,
            _ => unreachable!(),
        }
    }
}

struct Stack {
    inner: Vec<Bracket>,
}

impl Stack {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn push(&mut self, bracket: Bracket) {
        self.inner.push(bracket);
    }

    fn pop(&mut self, bracket: Bracket) -> Result<Bracket, ParseBracketsError> {
        if self.inner.is_empty() || *self.inner.last().unwrap() != bracket {
            Err(ParseBracketsError(bracket))
        } else {
            self.inner.pop().ok_or(ParseBracketsError(bracket))
        }
    }

    fn pop_correct(&mut self) -> Option<Bracket> {
        self.inner.pop()
    }
}

impl std::default::Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

const BAD_PAREN_VAL: usize = 3;
const BAD_SQUARE_VAL: usize = 57;
const BAD_CURLY_VAL: usize = 1197;
const BAD_ANGLE_VAL: usize = 25137;
const GOOD_PAREN_VAL: usize = 1;
const GOOD_SQUARE_VAL: usize = 2;
const GOOD_CURLY_VAL: usize = 3;
const GOOD_ANGLE_VAL: usize = 4;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut part1 = 0;
    let mut part2_arr = vec![];

    'outer: for line in input.trim().lines() {
        let mut stack = Stack::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(Bracket::from(ch)),
                ')' | ']' | '}' | '>' => {
                    if let Err(ParseBracketsError(bracket)) = stack.pop(Bracket::from(ch)) {
                        part1 += match bracket {
                            Bracket::Paren => BAD_PAREN_VAL,
                            Bracket::Square => BAD_SQUARE_VAL,
                            Bracket::Curly => BAD_CURLY_VAL,
                            Bracket::Angle => BAD_ANGLE_VAL,
                        };
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }
        let mut acc = 0;
        while let Some(bracket) = stack.pop_correct() {
            acc *= 5;
            acc += match bracket {
                Bracket::Paren => GOOD_PAREN_VAL,
                Bracket::Square => GOOD_SQUARE_VAL,
                Bracket::Curly => GOOD_CURLY_VAL,
                Bracket::Angle => GOOD_ANGLE_VAL,
            }
        }
        part2_arr.push(acc);
    }
    println!("{}", part1);
    part2_arr.sort_unstable();
    let part2 = part2_arr[part2_arr.len() / 2];
    println!("{:?}", part2_arr);

    println!("{}", part2_arr.len());
    println!("{}", part2);
}
