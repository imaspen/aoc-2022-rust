pub(crate) struct Day01 {
    vals: Vec<Vec<i32>>,
}

impl Day01 {
    pub(crate) fn new() -> Self {
        let vals = super::super::utils::read_day_grouped_ints(1);
        return Self { vals };
    }
}

impl super::Day for Day01 {
    fn part_1(&mut self) -> String {
        let max_sum = self
            .vals
            .iter()
            .map(|group| group.into_iter().fold(0, |acc, val| acc + val))
            .max();

        return max_sum.unwrap_or_default().to_string();
    }

    fn part_2(&mut self) -> String {
        let mut sums = self
            .vals
            .iter()
            .map(|group| group.into_iter().fold(0, |acc, val| acc + val))
            .collect::<Vec<_>>();

        sums.sort();
        sums.reverse();
        return (sums[0] + sums[1] + sums[2]).to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day01::new();
        assert_eq!(day.part_1(), "24000");
    }

    #[test]
    fn part_2() {
        let mut day = Day01::new();
        assert_eq!(day.part_2(), "45000");
    }
}
