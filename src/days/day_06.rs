use std::collections::HashSet;

pub(crate) struct Day06 {
    lines: Vec<String>,
}

impl Day06 {
    pub(crate) fn new() -> Self {
        let lines = super::super::utils::read_day_lines(6);
        return Self { lines };
    }

    fn find_markers(&mut self, marker_len: usize) -> String {
        let x = self
            .lines
            .iter()
            .map(|line| {
                return line
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(marker_len)
                    .enumerate()
                    .find_map(|(i, window)| {
                        let mut unique = HashSet::new();
                        if window.iter().all(|char| unique.insert(*char)) {
                            return Some(i + marker_len);
                        } else {
                            return None;
                        }
                    })
                    .unwrap()
                    .to_string();
            })
            .collect::<Vec<_>>();
        return x.join("\n").to_string();
    }
}

impl super::Day for Day06 {
    fn part_1(&mut self) -> String {
        return self.find_markers(4);
    }

    fn part_2(&mut self) -> String {
        return self.find_markers(14);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day06::new();
        assert_eq!(day.part_1(), "7\n5\n6\n10\n11");
    }

    #[test]
    fn part_2() {
        let mut day = Day06::new();
        assert_eq!(day.part_2(), "19\n23\n23\n29\n26");
    }
}
