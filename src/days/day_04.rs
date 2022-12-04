use crate::utils::read_day_csv_lines;

struct Assignment {
    lower_bound: u8,
    upper_bound: u8,
}

impl Assignment {
    fn new(str: &String) -> Self {
        let bounds: Vec<u8> = str.split("-").map(|part| part.parse().unwrap()).collect();

        return Self {
            lower_bound: bounds[0],
            upper_bound: bounds[1],
        };
    }

    fn contains(self: &Assignment, rhs: &Assignment) -> bool {
        return self.lower_bound <= rhs.lower_bound && self.upper_bound >= rhs.upper_bound;
    }

    fn overlaps(self: &Assignment, rhs: &Assignment) -> bool {
        return (self.lower_bound >= rhs.lower_bound && self.lower_bound <= rhs.upper_bound)
            || (self.upper_bound <= rhs.upper_bound && self.upper_bound >= rhs.lower_bound);
    }
}

pub(crate) struct Day04 {
    vals: Vec<(Assignment, Assignment)>,
}

impl Day04 {
    pub(crate) fn new() -> Self {
        let vals = read_day_csv_lines(4)
            .iter()
            .map(|parts| (Assignment::new(&parts[0]), Assignment::new(&parts[1])))
            .collect();
        return Day04 { vals };
    }
}

impl super::Day for Day04 {
    fn part_1(&self) -> String {
        let mut count = 0;
        for assignments in &self.vals {
            if assignments.0.contains(&assignments.1) || assignments.1.contains(&assignments.0) {
                count += 1;
            }
        }
        return count.to_string();
    }

    fn part_2(&self) -> String {
        let mut count = 0;
        for assignments in &self.vals {
            if assignments.0.overlaps(&assignments.1) || assignments.1.overlaps(&assignments.0) {
                count += 1;
            }
        }
        return count.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let day = Day04::new();
        assert_eq!(day.part_1(), "2");
    }

    #[test]
    fn part_2() {
        let day = Day04::new();
        assert_eq!(day.part_2(), "4");
    }
}
