pub(crate) struct Day05 {
    stacks: Box<[Vec<char>]>,
    instructions: Vec<Instruction>,
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Day05 {
    pub(crate) fn new() -> Self {
        let mut vals = crate::utils::read_day_grouped_lines(5);
        let column_defs = vals[0].pop().unwrap();
        let column_count = column_defs
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = vec![];
        stacks.resize(column_count, vec![]);

        vals[0].reverse();
        for row in &vals[0] {
            let mut chars = row.chars();
            chars.next();

            for i in 0..column_count {
                let char = chars.next().unwrap();
                if char != ' ' {
                    stacks[i].push(char);
                }

                chars.next();
                chars.next();
                chars.next();
            }
        }

        let instructions = vals[1]
            .iter()
            .map(|val| {
                let parts: Vec<_> = val.split_whitespace().collect();
                return Instruction {
                    count: parts[1].parse().unwrap(),
                    from: parts[3].parse::<usize>().unwrap() - 1,
                    to: parts[5].parse::<usize>().unwrap() - 1,
                };
            })
            .collect::<Vec<Instruction>>();

        return Self {
            stacks: stacks.into_boxed_slice(),
            instructions,
        };
    }

    fn get_output(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>();
    }
}

impl super::Day for Day05 {
    fn part_1(&mut self) -> String {
        for instruction in &self.instructions {
            for _ in 0..instruction.count {
                let char = self.stacks[instruction.from].pop().unwrap();
                self.stacks[instruction.to].push(char);
            }
        }

        return self.get_output();
    }

    fn part_2(&mut self) -> String {
        let mut tmp: Vec<char> = vec![];
        tmp.reserve(32);

        for instruction in &self.instructions {
            for _ in 0..instruction.count {
                let char = self.stacks[instruction.from].pop().unwrap();
                tmp.push(char);
            }
            for _ in 0..instruction.count {
                let char = tmp.pop().unwrap();
                self.stacks[instruction.to].push(char);
            }
        }

        return self.get_output();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day05::new();
        assert_eq!(day.part_1(), "CMZ");
    }

    #[test]
    fn part_2() {
        let mut day = Day05::new();
        assert_eq!(day.part_2(), "MCD");
    }
}
