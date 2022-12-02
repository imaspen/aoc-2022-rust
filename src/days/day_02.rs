pub(crate) struct Day02 {
    vals: Vec<(i32, i32)>,
}

impl Day02 {
    pub(crate) fn new() -> Self {
        let vals = crate::utils::read_day_lines(2);
        let parts = vals
            .iter()
            .map(|str| {
                let their_play: i32;
                match str.chars().nth(0).unwrap_or('?') {
                    'A' => their_play = 0,
                    'B' => their_play = 1,
                    'C' => their_play = 2,
                    _ => panic!(
                        "Unrecognized opponent play: {}",
                        str.chars().nth(0).unwrap_or('_')
                    ),
                }
                let our_play: i32;
                match str.chars().nth(2).unwrap_or('?') {
                    'X' => our_play = 0,
                    'Y' => our_play = 1,
                    'Z' => our_play = 2,
                    _ => panic!(
                        "Unrecognized player play: {}",
                        str.chars().nth(0).unwrap_or('_')
                    ),
                }
                return (their_play, our_play);
            })
            .collect();
        return Self { vals: parts };
    }
}

impl super::Day for Day02 {
    fn part_1(&self) -> String {
        let score = self
            .vals
            .iter()
            .fold(0, |total_score, &(their_play, our_play)| {
                let mut round_score = 1 + our_play;

                if their_play == our_play {
                    round_score += 3;
                } else if (their_play + 1) % 3 == our_play {
                    round_score += 6;
                }

                return total_score + round_score;
            });

        return score.to_string();
    }

    fn part_2(&self) -> String {
        let score = self
            .vals
            .iter()
            .fold(0, |total_score, &(their_play, outcome)| {
                let mut round_score = 1;

                match outcome {
                    0 => round_score += (their_play - 1).rem_euclid(3),
                    1 => round_score += 3 + their_play,
                    2 => round_score += 6 + (their_play + 1) % 3,
                    _ => panic!(),
                }

                return total_score + round_score;
            });

        return score.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let day = Day02::new();
        assert_eq!(day.part_1(), "15");
    }

    #[test]
    fn part_2() {
        let day = Day02::new();
        assert_eq!(day.part_2(), "12");
    }
}
