use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Rhs {
    Old,
    Val(u64),
}

#[derive(Debug)]
struct Monkey {
    inspection_count: usize,
    held_items: VecDeque<u64>,
    operator: char,
    rhs: Rhs,
    test: u64,
    true_target: usize,
    false_target: usize,
}

pub(crate) struct Day11 {
    monkeys: Vec<Monkey>,
}

lazy_static! {
    static ref STARTING_ITEMS: Regex = Regex::new(r"\d{2}").unwrap();
    static ref OPERATION: Regex =
        Regex::new(r"^  Operation: new = old (\+|\*) (old|\d+)$").unwrap();
    static ref TEST: Regex = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    static ref TARGET: Regex =
        Regex::new(r"^    If (?:true|false): throw to monkey (\d+)$").unwrap();
}

impl Day11 {
    pub(crate) fn new() -> Self {
        let monkeys = crate::utils::read_day_grouped_lines(11)
            .iter()
            .map(|lines| {
                let held_items = STARTING_ITEMS
                    .captures_iter(&lines[1])
                    .map(|capture| capture[0].to_string().parse().unwrap())
                    .collect();

                let operation = OPERATION.captures(&lines[2]).unwrap();
                let operator = operation[1].chars().next().unwrap();
                let rhs;
                if operation[2] == *"old" {
                    rhs = Rhs::Old;
                } else {
                    rhs = Rhs::Val(operation[2].parse().unwrap());
                }

                let test = TEST.captures(&lines[3]).unwrap()[1].parse().unwrap();

                let true_target = TARGET.captures(&lines[4]).unwrap()[1].parse().unwrap();
                let false_target = TARGET.captures(&lines[5]).unwrap()[1].parse().unwrap();

                Monkey {
                    inspection_count: 0,
                    held_items,
                    operator,
                    rhs,
                    test,
                    true_target,
                    false_target,
                }
            })
            .collect();

        Self { monkeys }
    }
}

impl super::Day for Day11 {
    fn part_1(&mut self) -> String {
        for _ in 0..20 {
            for i in 0..self.monkeys.len() {
                while let Some(mut val) = self.monkeys[i].held_items.pop_front() {
                    self.monkeys[i].inspection_count += 1;

                    match (&self.monkeys[i].operator, &self.monkeys[i].rhs) {
                        ('+', Rhs::Old) => val += val,
                        ('*', Rhs::Old) => val *= val,
                        ('+', Rhs::Val(modifier)) => val += *modifier,
                        ('*', Rhs::Val(modifier)) => val *= *modifier,
                        _ => panic!("Unrecognized operator: {}", self.monkeys[i].operator),
                    }
                    val /= 3;

                    let target;
                    if val % self.monkeys[i].test == 0 {
                        target = self.monkeys[i].true_target;
                    } else {
                        target = self.monkeys[i].false_target;
                    }
                    self.monkeys[target].held_items.push_back(val);
                }
            }
        }
        self.monkeys
            .sort_by_cached_key(|monkey| monkey.inspection_count);
        self.monkeys.reverse();

        return (self.monkeys[0].inspection_count * self.monkeys[1].inspection_count).to_string();
    }

    fn part_2(&mut self) -> String {
        let modder = self.monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);
        for _ in 0..10_000 {
            for i in 0..self.monkeys.len() {
                while let Some(mut val) = self.monkeys[i].held_items.pop_front() {
                    self.monkeys[i].inspection_count += 1;

                    match (&self.monkeys[i].operator, &self.monkeys[i].rhs) {
                        ('+', Rhs::Old) => val += val,
                        ('*', Rhs::Old) => val *= val,
                        ('+', Rhs::Val(modifier)) => val += *modifier,
                        ('*', Rhs::Val(modifier)) => val *= *modifier,
                        _ => panic!("Unrecognized operator: {}", self.monkeys[i].operator),
                    }

                    val %= modder;
                    let target;
                    if val % self.monkeys[i].test == 0 {
                        target = self.monkeys[i].true_target;
                    } else {
                        target = self.monkeys[i].false_target;
                    }
                    self.monkeys[target].held_items.push_back(val);
                }
            }
        }

        self.monkeys
            .sort_by_cached_key(|monkey| monkey.inspection_count);
        self.monkeys.reverse();

        return (self.monkeys[0].inspection_count * self.monkeys[1].inspection_count).to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day11::new();
        assert_eq!(day.part_1(), "10605");
    }

    #[test]
    fn part_2() {
        let mut day = Day11::new();
        assert_eq!(day.part_2(), "2713310158");
    }
}
