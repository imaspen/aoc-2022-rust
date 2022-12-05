pub(crate) struct Day03 {
    vals: Vec<Vec<u8>>,
}

fn str_to_priorities(str: &str) -> Vec<u8> {
    return str
        .as_bytes()
        .iter()
        .map(|byte| match *byte {
            65..=90 => *byte - 65 + 26,
            97..=122 => *byte - 97,
            _ => panic!("Unrecognized priority in {}", str),
        })
        .collect();
}

impl Day03 {
    pub(crate) fn new() -> Self {
        // convert the item labels a..zA..Z to their priority (0-indexed)
        let vals = crate::utils::read_day_lines(3)
            .iter()
            .map(|val| str_to_priorities(val))
            .collect();
        return Self { vals };
    }
}

impl super::Day for Day03 {
    fn part_1(&mut self) -> String {
        return self
            .vals
            .iter()
            .fold(0, |acc, vals| {
                let (left, right) = vals.split_at(vals.len() / 2);

                // map of seen items from the left side of the bag, to check for on the right side.
                let mut seen = [false; 52];
                for priority in left {
                    seen[*priority as usize] = true;
                }
                for priority in right {
                    if seen[*priority as usize] {
                        return acc + (*priority) as u32 + 1;
                    }
                }
                panic!("No duplicate priority found in: {:?},{:?}", left, right);
            })
            .to_string();
    }

    fn part_2(&mut self) -> String {
        let sum = self.vals.chunks_exact(3).fold(0, |acc, chunk| {
            let matched_bit = chunk.iter().fold(u64::MAX, |matched_bits, bag| {
                // convert each priority into its relative power of 2, and create a bitmask of all items in the bag
                let bag_bits = bag.iter().fold(0, |bits, item| {
                    let w = u64::pow(2, (*item) as u32);
                    return bits | w;
                });
                // AND the bitmasks to find the exclusive item in the bag
                return matched_bits & bag_bits;
            });

            // get the priority back from the bitmask by finding the 1s position
            return acc + (64 - matched_bit.leading_zeros());
        });

        return sum.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day03::new();
        assert_eq!(day.part_1(), "157");
    }

    #[test]
    fn part_2() {
        let mut day = Day03::new();
        assert_eq!(day.part_2(), "70");
    }
}
