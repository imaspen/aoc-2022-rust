use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

lazy_static! {
    static ref SHAPES: [Box<[Box<[bool]>]>; 5] = [
        Box::new([
            Box::new([true, true, true, true]) // ####
        ]),
        Box::new([
            Box::new([false, true, false]), // .#.
            Box::new([true, true, true]),   // ###
            Box::new([false, true, false]), // .#.
        ]),
        Box::new([
            Box::new([true, true, true]),   // ###
            Box::new([false, false, true]), // ..#
            Box::new([false, false, true]), // ..#
        ]),
        Box::new([
            Box::new([true]), // #
            Box::new([true]), // #
            Box::new([true]), // #
            Box::new([true]), // #
        ]),
        Box::new([
            Box::new([true, true]), // ##
            Box::new([true, true]), // ##
        ]),
    ];
}

pub(crate) struct Day17 {
    jet_pattern: Box<[Direction]>,
}

impl Day17 {
    pub(crate) fn new() -> Self {
        let jet_pattern = crate::utils::read_day(17)
            .trim()
            .chars()
            .map(|char| match char {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Unexpected direction character: {}", char.escape_default()),
            })
            .collect();
        return Self { jet_pattern };
    }
}

fn can_move_l(x: usize, y: usize, chamber: &Vec<[bool; 7]>, shape: &Box<[Box<[bool]>]>) -> bool {
    if x == 0 {
        return false;
    }

    for (y_offset, row) in shape.iter().enumerate() {
        let y1 = y + y_offset;
        if y1 >= chamber.len() {
            break;
        }

        for (x_offset, cell) in row.iter().enumerate() {
            if *cell != false && chamber[y1][(x + x_offset - 1)] {
                return false;
            }
        }
    }

    true
}

fn can_move_r(x: usize, y: usize, chamber: &Vec<[bool; 7]>, shape: &Box<[Box<[bool]>]>) -> bool {
    if x + shape[0].len() >= 7 {
        return false;
    }

    for (y_offset, row) in shape.iter().enumerate() {
        let y1 = y + y_offset;
        if y1 >= chamber.len() {
            break;
        }

        for (x_offset, cell) in row.iter().enumerate() {
            if *cell != false && chamber[y1][(x + x_offset + 1)] {
                return false;
            }
        }
    }

    true
}

fn can_move_d(x: usize, y: usize, chamber: &Vec<[bool; 7]>, shape: &Box<[Box<[bool]>]>) -> bool {
    if y == 0 {
        return false;
    }

    for (y_offset, row) in shape.iter().enumerate() {
        let y1 = y + y_offset - 1;
        if y1 >= chamber.len() {
            break;
        }

        for (x_offset, cell) in row.iter().enumerate() {
            if *cell != false && chamber[y1][(x + x_offset)] {
                return false;
            }
        }
    }

    true
}

fn settle(x: usize, y: usize, chamber: &mut Vec<[bool; 7]>, shape: &Box<[Box<[bool]>]>) {
    for (y_offset, row) in shape.iter().enumerate() {
        let y1 = y + y_offset;
        while y1 >= chamber.len() {
            chamber.push([false; 7]);
        }

        for (x_offset, cell) in row.iter().enumerate() {
            let x1 = x + x_offset;
            chamber[y1][x1] |= *cell;
        }
    }
}

fn get_top(chamber: &Vec<[bool; 7]>) -> Option<[usize; 7]> {
    let mut found = [false; 7];
    let mut pos = [0; 7];

    for row in chamber.iter().rev() {
        for i in 0..7 {
            if row[i] {
                found[i] = true;
            }

            if found[i] {
                continue;
            }

            pos[i] += 1;
        }

        if found.iter().all(|f| *f) {
            return Some(pos);
        }
    }

    None
}

#[allow(dead_code)]
fn print_chamber(chamber: &Vec<[bool; 7]>) {
    for row in chamber.iter().rev() {
        for cell in row.iter() {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}

impl super::Day for Day17 {
    fn part_1(&mut self) -> String {
        let mut chamber = vec![];
        let mut jets = self.jet_pattern.iter().cycle();
        let mut shapes = SHAPES.iter().cycle();

        for _ in 0..2022 {
            let shape = shapes.next().unwrap();
            let mut x = 2;
            let mut y = chamber.len() + 3;

            loop {
                let jet = jets.next().unwrap();

                match *jet {
                    Direction::Left => {
                        if can_move_l(x, y, &chamber, shape) {
                            x -= 1;
                        }
                    }
                    Direction::Right => {
                        if can_move_r(x, y, &chamber, shape) {
                            x += 1;
                        }
                    }
                }

                if can_move_d(x, y, &chamber, shape) {
                    y -= 1;
                } else {
                    break;
                }
            }

            settle(x, y, &mut chamber, shape);
        }

        chamber.len().to_string()
    }

    fn part_2(&mut self) -> String {
        let mut period = self.jet_pattern.len();
        while period % SHAPES.len() != 0 {
            period += self.jet_pattern.len();
        }

        let mut chamber = vec![];
        let mut jets = self.jet_pattern.iter().enumerate().cycle();
        let mut shapes = SHAPES.iter().enumerate().cycle();

        let mut seen: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();

        let run_time = 1_000_000_000_000usize;
        let mut i = 0;
        let mut height_offset = 0;
        while i < run_time {
            let (shape_idx, shape) = shapes.next().unwrap();
            let mut x = 2;
            let mut y = chamber.len() + 3;

            if let Some((prev_idx, prev_len)) = loop {
                let (jet_idx, jet) = jets.next().unwrap();

                match *jet {
                    Direction::Left => {
                        if can_move_l(x, y, &chamber, shape) {
                            x -= 1;
                        }
                    }
                    Direction::Right => {
                        if can_move_r(x, y, &chamber, shape) {
                            x += 1;
                        }
                    }
                }

                if can_move_d(x, y, &chamber, shape) {
                    y -= 1;
                } else {
                    settle(x, y, &mut chamber, shape);

                    if height_offset == 0 {
                        if let Some(top) = get_top(&chamber) {
                            break seen.insert((shape_idx, jet_idx, top), (i, chamber.len()));
                        }
                    }

                    break None;
                }
            } {
                let height_per_iteration = chamber.len() - prev_len;
                let cycle_len = i - prev_idx;
                let cycles = (run_time - i) / cycle_len;
                height_offset = cycles * height_per_iteration;
                i += cycles * cycle_len;
            };

            i += 1;
        }

        (chamber.len() + height_offset).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day17::new();
        assert_eq!(day.part_1(), "3068");
    }

    #[test]
    fn part_2() {
        let mut day = Day17::new();
        assert_eq!(day.part_2(), "1514285714288");
    }
}
