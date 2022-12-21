use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    operator: Operator,
}

#[derive(Debug)]
enum Value {
    Val(f64),
    Operation(Operation),
}

impl Value {
    fn get_value(&self, monkeys: &HashMap<String, Value>, is_root: bool) -> (f64, bool) {
        match self {
            Self::Val(val) => (*val, false),
            Self::Operation(Operation { lhs, rhs, operator }) => {
                let (lval, l_human) = monkeys.get(lhs).unwrap().get_value(monkeys, false);
                let (rval, r_human) = monkeys.get(rhs).unwrap().get_value(monkeys, false);
                let is_human_branch = l_human || r_human || lhs == "humn" || rhs == "human";
                if is_root {
                    if l_human {
                        (rval - lval, true)
                    } else {
                        (lval - rval, true)
                    }
                } else {
                    match operator {
                        Operator::Add => (lval + rval, is_human_branch),
                        Operator::Subtract => (lval - rval, is_human_branch),
                        Operator::Multiply => (lval * rval, is_human_branch),
                        Operator::Divide => (lval / rval, is_human_branch),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Day21 {
    monkeys: HashMap<String, Value>,
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^([a-z]{4}): (?:(\d+)|([a-z]{4}) ([\+\-\*/]) ([a-z]{4}))$").unwrap();
}

impl Day21 {
    pub(crate) fn new() -> Self {
        let monkeys = crate::utils::read_day_lines(21)
            .iter()
            .map(|line| REGEX.captures(line).unwrap())
            .map(|capture| {
                (
                    capture.get(1).unwrap().as_str().to_string(),
                    if let Some(value) = capture.get(2) {
                        Value::Val(value.as_str().parse().unwrap())
                    } else {
                        Value::Operation(Operation {
                            lhs: capture.get(3).unwrap().as_str().parse().unwrap(),
                            rhs: capture.get(5).unwrap().as_str().parse().unwrap(),
                            operator: match capture.get(4).unwrap().as_str().chars().next().unwrap()
                            {
                                '+' => Operator::Add,
                                '-' => Operator::Subtract,
                                '*' => Operator::Multiply,
                                '/' => Operator::Divide,
                                char => panic!("Unexpected operator: {}", char),
                            },
                        })
                    },
                )
            })
            .collect();
        return Self { monkeys };
    }
}

impl super::Day for Day21 {
    fn part_1(&mut self) -> String {
        self.monkeys
            .get("root")
            .unwrap()
            .get_value(&self.monkeys, false)
            .0
            .to_string()
    }

    fn part_2(&mut self) -> String {
        let mut val: f64 = 1.0;
        let mut last_positive_val = 0.0;
        let mut last_negative_val = -1.0;
        let mut one_res: f64 = 0.0;
        loop {
            self.monkeys.insert("humn".to_string(), Value::Val(val));
            let res = self
                .monkeys
                .get("root")
                .unwrap()
                .get_value(&self.monkeys, true)
                .0;

            if res == 0.0 {
                return val.to_string();
            }

            if val == 1.0 {
                last_positive_val = val;
                val = 2.0;
                one_res = res;
            } else if val < i64::MAX as f64 {
                if (one_res > 0.0 && res > 0.0) || (one_res < 0.0 && res < 0.0) {
                    last_positive_val = val;
                    if last_negative_val < 0.0 {
                        val *= 2.0;
                    } else {
                        val = last_negative_val + ((last_positive_val - last_negative_val) / 2.0);
                    }
                    val = val.round();
                } else {
                    last_negative_val = val;
                    val = last_positive_val + (val - last_positive_val) / 2.0;
                    val = val.round();
                }
            } else {
                panic!();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day21::new();
        assert_eq!(day.part_1(), "152");
    }

    #[test]
    fn part_2() {
        let mut day = Day21::new();
        assert_eq!(day.part_2(), "301");
    }
}
