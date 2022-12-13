#[derive(Debug, PartialEq)]
enum Entry {
    Num(u8),
    Sub(Vec<Entry>),
}

pub(crate) struct Day13 {
    vals: Vec<(Entry, Entry)>,
}

impl Day13 {
    fn parse_line(chars: &mut std::str::Chars) -> Entry {
        let mut vals: Vec<Entry> = vec![];
        let mut should_add_val = false;
        let mut val = 0;
        while let Some(char) = {
            let next = chars.next();
            match next {
                Some(']') => None,
                _ => next,
            }
        } {
            match char {
                '0' => {
                    should_add_val = true;
                    val *= 10;
                }
                ',' => {
                    vals.push(Entry::Num(val));
                    val = 0;
                    should_add_val = false;
                }
                '[' => {
                    vals.push(Self::parse_line(chars));
                    chars.next();
                }
                '1'..='9' => {
                    should_add_val = true;
                    val = u8::try_from(char).unwrap() - 48;
                }
                _ => {
                    panic!("Unexpected character: {}", char);
                }
            }
        }
        if should_add_val {
            vals.push(Entry::Num(val));
        }
        return Entry::Sub(vals);
    }

    pub(crate) fn new() -> Self {
        let vals = crate::utils::read_day_grouped_lines(13)
            .iter()
            .map(|lines| {
                let mut lhs_chars = lines[0].chars();
                let mut rhs_chars = lines[1].chars();
                lhs_chars.next();
                rhs_chars.next();
                (
                    Self::parse_line(&mut lhs_chars),
                    Self::parse_line(&mut rhs_chars),
                )
            })
            .collect();

        return Self { vals };
    }

    fn compare_entries(lhs: &Entry, rhs: &Entry) -> Option<bool> {
        match (lhs, rhs) {
            (Entry::Num(x), Entry::Num(y)) => {
                if *x == *y {
                    return None;
                } else {
                    return Some(*x < *y);
                }
            }
            (Entry::Sub(x), Entry::Sub(y)) => {
                let mut lhs_iter = x.iter();
                let mut rhs_iter = y.iter();
                loop {
                    match (lhs_iter.next(), rhs_iter.next()) {
                        (None, None) => return None,
                        (Some(lhs_next), Some(rhs_next)) => {
                            match Self::compare_entries(lhs_next, rhs_next) {
                                Some(val) => return Some(val),
                                None => (),
                            }
                        }
                        (None, Some(_)) => return Some(true),
                        (Some(_), None) => return Some(false),
                    }
                }
            }
            (Entry::Num(x), Entry::Sub(_)) => {
                return Self::compare_entries(&Entry::Sub(vec![Entry::Num(*x)]), rhs)
            }
            (Entry::Sub(_), Entry::Num(y)) => {
                return Self::compare_entries(lhs, &Entry::Sub(vec![Entry::Num(*y)]))
            }
        }
    }
}

impl super::Day for Day13 {
    fn part_1(&mut self) -> String {
        let mut total = 0;
        for (i, (lhs, rhs)) in self.vals.iter().enumerate() {
            if Self::compare_entries(lhs, rhs).unwrap() {
                total += i + 1;
            }
        }
        total.to_string()
    }

    fn part_2(&mut self) -> String {
        let mut vals: Vec<&Entry> = self
            .vals
            .iter()
            .flat_map(|(lhs, rhs)| vec![lhs, rhs].into_iter())
            .collect();

        let divider_1 = Entry::Sub(vec![Entry::Sub(vec![Entry::Num(2)])]);
        let divider_2 = Entry::Sub(vec![Entry::Sub(vec![Entry::Num(6)])]);

        vals.push(&divider_1);
        vals.push(&divider_2);

        vals.sort_by(|a, b| {
            if Self::compare_entries(a, b).unwrap() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        let mut output = 1;
        for (i, entry) in vals.into_iter().enumerate() {
            if *entry == divider_1 || *entry == divider_2 {
                output *= i + 1;
            }
        }

        output.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day13::new();
        assert_eq!(day.part_1(), "13");
    }

    #[test]
    fn part_2() {
        let mut day = Day13::new();
        assert_eq!(day.part_2(), "140");
    }
}
