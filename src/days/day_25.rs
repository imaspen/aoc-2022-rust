use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Clone, Copy, Debug)]
enum Digit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

use Digit::*;

impl From<&Digit> for char {
    fn from(d: &Digit) -> Self {
        match d {
            DoubleMinus => '=',
            Minus => '-',
            Zero => '0',
            One => '1',
            Two => '2',
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(DoubleMinus),
            '-' => Ok(Minus),
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            _ => Err(format!("Unexpected SNAFU char: {}", value)),
        }
    }
}

struct DigitAddResult {
    result: Digit,
    overflow: Option<Digit>,
}

impl From<Digit> for i8 {
    fn from(digit: Digit) -> Self {
        match digit {
            DoubleMinus => -2,
            Minus => -1,
            Zero => 0,
            One => 1,
            Two => 2,
        }
    }
}

impl Add for Digit {
    type Output = DigitAddResult;

    fn add(self, rhs: Self) -> Self::Output {
        match i8::from(self) + i8::from(rhs) {
            -4 => DigitAddResult {
                result: One,
                overflow: Some(Minus),
            },
            -3 => DigitAddResult {
                result: Two,
                overflow: Some(Minus),
            },
            -2 => DigitAddResult {
                result: DoubleMinus,
                overflow: None,
            },
            -1 => DigitAddResult {
                result: Minus,
                overflow: None,
            },
            0 => DigitAddResult {
                result: Zero,
                overflow: None,
            },
            1 => DigitAddResult {
                result: One,
                overflow: None,
            },
            2 => DigitAddResult {
                result: Two,
                overflow: None,
            },
            3 => DigitAddResult {
                result: DoubleMinus,
                overflow: Some(One),
            },
            4 => DigitAddResult {
                result: Minus,
                overflow: Some(One),
            },
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Number {
    digits: Vec<Digit>,
}

impl Index<usize> for Number {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[index]
    }
}

impl IndexMut<usize> for Number {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digits[index]
    }
}

impl AddAssign<&Number> for Number {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0..self.digits.len().max(rhs.digits.len()) {
            let lhs_digit = if i < self.digits.len() { self[i] } else { Zero };
            let rhs_digit = if i < rhs.digits.len() { rhs[i] } else { Zero };
            let DigitAddResult {
                result,
                mut overflow,
            } = lhs_digit + rhs_digit;

            if i < self.digits.len() {
                self[i] = result;
            } else {
                self.digits.push(result);
            }

            let mut j = 0;
            while let Some(carry) = overflow {
                j += 1;
                if i + j >= self.digits.len() {
                    self.digits.push(carry);
                    overflow = None;
                } else {
                    let DigitAddResult {
                        result,
                        overflow: next_overflow,
                    } = self[i + j] + carry;

                    self[i + j] = result;

                    overflow = next_overflow;
                }
            }
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.digits.iter().rev().map(char::from).collect()
    }
}

pub(crate) struct Day25 {
    nums: Box<[Number]>,
}

impl Day25 {
    pub(crate) fn new() -> Self {
        let nums = crate::utils::read_day_lines(25)
            .iter()
            .map(|line| Number {
                digits: line
                    .chars()
                    .map(|char| Digit::try_from(char).unwrap())
                    .rev()
                    .collect(),
            })
            .collect();
        return Self { nums };
    }
}

impl super::Day for Day25 {
    fn part_1(&mut self) -> String {
        let mut total = self.nums[0].clone();
        for num in self.nums[1..].iter() {
            total += num;
        }
        total.to_string()
    }

    fn part_2(&mut self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day25::new();
        assert_eq!(day.part_1(), "2=-1=0");
    }

    #[test]
    fn part_2() {
        let mut day = Day25::new();
        assert_eq!(day.part_2(), "");
    }
}
