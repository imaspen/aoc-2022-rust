use core::panic;

type Instruction = (char, usize);

pub(crate) struct Day09 {
    instructions: Vec<Instruction>,
}

impl Day09 {
    pub(crate) fn new() -> Self {
        let instructions = crate::utils::read_day_lines(9)
            .iter()
            .map(|line| {
                let mut parts = line.split_ascii_whitespace();
                (
                    parts.next().unwrap().chars().next().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        return Self { instructions };
    }
}

fn update_tail_position(head_x: usize, head_y: usize, tail_x: &mut usize, tail_y: &mut usize) {
    if head_x - 1 > *tail_x {
        *tail_x += 1;
        if *tail_y < head_y {
            *tail_y += 1;
        } else if *tail_y > head_y {
            *tail_y -= 1;
        }
    } else if head_x + 1 < *tail_x {
        *tail_x -= 1;
        if *tail_y < head_y {
            *tail_y += 1;
        } else if *tail_y > head_y {
            *tail_y -= 1;
        }
    } else if head_y - 1 > *tail_y {
        *tail_y += 1;
        if *tail_x < head_x {
            *tail_x += 1;
        } else if *tail_x > head_x {
            *tail_x -= 1;
        }
    } else if head_y + 1 < *tail_y {
        *tail_y -= 1;
        if *tail_x < head_x {
            *tail_x += 1;
        } else if *tail_x > head_x {
            *tail_x -= 1;
        }
    }
}

impl super::Day for Day09 {
    fn part_1(&mut self) -> String {
        let mut visited = vec![vec![false; 4096]; 4096];
        let mut head_x = 2048;
        let mut head_y = 2048;
        let mut tail_x = 2048;
        let mut tail_y = 2048;

        for (dir, count) in self.instructions.iter() {
            for _ in 0..*count {
                match *dir {
                    'R' => head_x += 1,
                    'L' => head_x -= 1,
                    'U' => head_y += 1,
                    'D' => head_y -= 1,
                    _ => panic!("Unrecognized direction: {}", *dir),
                }

                update_tail_position(head_x, head_y, &mut tail_x, &mut tail_y);

                visited[tail_y][tail_x] = true;
            }
        }

        return visited
            .iter()
            .fold(0, |acc, row| {
                acc + row
                    .iter()
                    .fold(0, |acc, val| if *val { acc + 1 } else { acc })
            })
            .to_string();
    }

    fn part_2(&mut self) -> String {
        let mut visited = vec![vec![false; 4096]; 4096];
        let mut knots = [(2048_usize, 2048_usize); 10];

        for (dir, count) in self.instructions.iter() {
            for _ in 0..*count {
                match *dir {
                    'R' => knots[0].0 += 1,
                    'L' => knots[0].0 -= 1,
                    'U' => knots[0].1 += 1,
                    'D' => knots[0].1 -= 1,
                    _ => panic!("Unrecognized direction: {}", *dir),
                }

                for i in 1..knots.len() {
                    let head_x = knots[i - 1].0;
                    let head_y = knots[i - 1].1;
                    let tail_x = &mut knots[i].0;
                    let tail_y = &mut knots[i].1;
                    update_tail_position(head_x, head_y, tail_x, tail_y);
                }

                visited[knots[9].1][knots[9].0] = true;
            }
        }

        return visited
            .iter()
            .fold(0, |acc, row| {
                acc + row
                    .iter()
                    .fold(0, |acc, val| if *val { acc + 1 } else { acc })
            })
            .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day09::new();
        assert_eq!(day.part_1(), "88");
    }

    #[test]
    fn part_2() {
        let mut day = Day09::new();
        assert_eq!(day.part_2(), "36");
    }
}
