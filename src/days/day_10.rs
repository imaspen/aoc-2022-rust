pub(crate) struct Day10 {
    instructions: Vec<String>,
}

impl Day10 {
    pub(crate) fn new() -> Self {
        let instructions = crate::utils::read_day_lines(10);
        return Self { instructions };
    }
}

impl super::Day for Day10 {
    fn part_1(&mut self) -> String {
        let mut cycle = 0;
        let mut acc = 1;
        let mut score = 0;

        for instruction in &self.instructions {
            let mut parts = instruction.split_ascii_whitespace();
            match parts.next().unwrap() {
                "noop" => {
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        score += acc * cycle;
                    }
                }
                "addx" => {
                    cycle += 2;
                    let rem = (cycle + 20) % 40;
                    if rem <= 1 {
                        score += acc * (cycle - rem);
                    }
                    acc += parts.next().unwrap().parse::<i32>().unwrap();
                }
                _ => panic!("unexpected instruction: {}", instruction),
            }
        }
        return score.to_string();
    }

    fn part_2(&mut self) -> String {
        let mut output = [' '; 245];
        output[40] = '\n';
        output[81] = '\n';
        output[122] = '\n';
        output[163] = '\n';
        output[204] = '\n';

        let mut cycle: usize = 0;
        let mut acc: i32 = 1;

        for instruction in &self.instructions {
            let x = i32::try_from(cycle).unwrap() % 40;
            if x >= acc - 1 && x <= acc + 1 {
                output[cycle + (cycle / 40)] = '#';
            }

            let mut parts = instruction.split_ascii_whitespace();
            if parts.next().unwrap() == "addx" {
                cycle += 1;

                let x = i32::try_from(cycle).unwrap() % 40;
                if x >= acc - 1 && x <= acc + 1 {
                    output[cycle + (cycle / 40)] = '#';
                }

                acc += parts.next().unwrap().parse::<i32>().unwrap();
            }
            cycle += 1;
        }
        return output.iter().collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day10::new();
        assert_eq!(day.part_1(), "13140");
    }

    #[test]
    fn part_2() {
        let mut day = Day10::new();
        assert_eq!(
            day.part_2(),
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     "
        );
    }
}
