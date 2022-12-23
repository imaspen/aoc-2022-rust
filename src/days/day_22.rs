use std::ops::Index;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Turn {
    CW,
    CCW,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Turn),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Content {
    Empty,
    Wall,
    Path,
}

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    content: Content,
    up: (usize, usize),
    right: (usize, usize),
    down: (usize, usize),
    left: (usize, usize),
}

impl Index<Direction> for Node {
    type Output = (usize, usize);

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Up => &self.up,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

pub(crate) struct Day22 {
    map: Box<[Box<[Node]>]>,
    instructions: Box<[Instruction]>,
}

impl Day22 {
    pub(crate) fn new() -> Self {
        let mut groups = crate::utils::read_day_grouped_lines(22);

        let grid_height = groups[0].len();
        let grid_width = groups[0].iter().map(|row| row.len()).max().unwrap();
        let content: Box<[Box<[Node]>]> = groups[0]
            .iter_mut()
            .enumerate()
            .map(|(y, row)| {
                while row.len() < grid_width {
                    row.push(' ');
                }
                row.chars()
                    .enumerate()
                    .map(|(x, cell)| {
                        let content = match cell {
                            ' ' => Content::Empty,
                            '.' => Content::Path,
                            '#' => Content::Wall,
                            char => panic!("Unexpected map char: {}", char),
                        };
                        Node {
                            pos: (x, y),
                            content,
                            up: (x, if y > 0 { y - 1 } else { grid_height - 1 }),
                            down: (x, if y < grid_height - 1 { y + 1 } else { 0 }),
                            left: (if x > 0 { x - 1 } else { grid_width - 1 }, y),
                            right: (if x < grid_width - 1 { x + 1 } else { 0 }, y),
                        }
                    })
                    .collect()
            })
            .collect();

        let map: Box<[Box<[Node]>]> = content
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        let mut up = cell.up;
                        while content[up.1][up.0].content == Content::Empty {
                            up = content[up.1][up.0].up;
                        }
                        let mut down = cell.down;
                        while content[down.1][down.0].content == Content::Empty {
                            down = content[down.1][down.0].down;
                        }
                        let mut left = cell.left;
                        while content[left.1][left.0].content == Content::Empty {
                            left = content[left.1][left.0].left;
                        }
                        let mut right = cell.right;
                        while content[right.1][right.0].content == Content::Empty {
                            right = content[right.1][right.0].right;
                        }
                        Node {
                            pos: cell.pos,
                            content: cell.content,
                            up,
                            down,
                            left,
                            right,
                        }
                    })
                    .collect()
            })
            .collect();

        let mut distance = 0;
        let mut instructions: Vec<Instruction> = vec![];
        for char in groups[1][0].chars() {
            match char {
                '0'..='9' => {
                    distance *= 10;
                    distance += char.to_digit(10).unwrap() as usize;
                }
                'R' => {
                    if distance > 0 {
                        instructions.push(Instruction::Move(distance));
                        distance = 0;
                    }
                    instructions.push(Instruction::Turn(Turn::CW));
                }
                'L' => {
                    if distance > 0 {
                        instructions.push(Instruction::Move(distance));
                        distance = 0;
                    }
                    instructions.push(Instruction::Turn(Turn::CCW));
                }
                other => panic!("Unexpected instruction char: {}", other),
            }
        }
        if distance > 0 {
            instructions.push(Instruction::Move(distance));
        }

        Self {
            map,
            instructions: instructions.into_boxed_slice(),
        }
    }

    fn move_on_cube(&self, pos: &Node, direction: Direction) -> (&Node, Direction) {
        let (x, y) = pos.pos;
        if *crate::utils::IS_TEST {
            match direction {
                Direction::Up => {
                    if y == 0 {
                        (&self.map[4][3 - (x - 8)], Direction::Down)
                    } else if self.map[y - 1][x].content != Content::Empty {
                        (&self.map[y - 1][x], direction)
                    } else if x < 4 {
                        (&self.map[0][11 - x], Direction::Down)
                    } else if x < 8 {
                        (&self.map[x - 4][8], Direction::Right)
                    } else {
                        (&self.map[7 - (x - 12)][11], Direction::Left)
                    }
                }
                Direction::Down => {
                    if y == 11 {
                        (&self.map[7][3 - (x - 8)], Direction::Up)
                    } else if self.map[y + 1][x].content != Content::Empty {
                        (&self.map[y + 1][x], direction)
                    } else if x < 4 {
                        (&self.map[11][11 - x], Direction::Up)
                    } else if x < 8 {
                        (&self.map[11 - (x - 4)][8], Direction::Right)
                    } else {
                        (&self.map[7 - (x - 12)][0], Direction::Right)
                    }
                }
                Direction::Left => {
                    if x == 0 {
                        (&self.map[11][15 - (y - 4)], Direction::Up)
                    } else if self.map[y][x - 1].content != Content::Empty {
                        (&self.map[y][x - 1], direction)
                    } else if y < 4 {
                        (&self.map[4][4 + y], Direction::Down)
                    } else {
                        (&self.map[7][7 - (y - 8)], Direction::Up)
                    }
                }
                Direction::Right => {
                    if x == 15 {
                        (&self.map[3 - (y - 8)][11], Direction::Left)
                    } else if self.map[y][x + 1].content != Content::Empty {
                        (&self.map[y][x + 1], direction)
                    } else if y < 4 {
                        (&self.map[11 - y][15], Direction::Left)
                    } else {
                        (&self.map[8][15 - (y - 4)], Direction::Down)
                    }
                }
            }
        } else {
            match direction {
                Direction::Up => {
                    if y == 0 {
                        if x < 100 {
                            (&self.map[100 + x][0], Direction::Right)
                        } else {
                            (&self.map[199][x - 100], Direction::Up)
                        }
                    } else if self.map[y - 1][x].content != Content::Empty {
                        (&self.map[y - 1][x], direction)
                    } else {
                        (&self.map[50 + x][50], Direction::Right)
                    }
                }
                Direction::Down => {
                    if y == 199 {
                        (&self.map[0][100 + x], Direction::Down)
                    } else if self.map[y + 1][x].content != Content::Empty {
                        (&self.map[y + 1][x], direction)
                    } else if x < 100 {
                        (&self.map[100 + x][49], Direction::Left)
                    } else {
                        (&self.map[x - 50][99], Direction::Left)
                    }
                }
                Direction::Left => {
                    if x == 0 {
                        if y < 150 {
                            (&self.map[49 - (y - 100)][50], Direction::Right)
                        } else {
                            (&self.map[0][y - 100], Direction::Down)
                        }
                    } else if self.map[y][x - 1].content != Content::Empty {
                        (&self.map[y][x - 1], direction)
                    } else if y < 50 {
                        (&self.map[149 - y][0], Direction::Right)
                    } else {
                        (&self.map[100][y - 50], Direction::Down)
                    }
                }
                Direction::Right => {
                    if x == 149 {
                        (&self.map[149 - y][99], Direction::Left)
                    } else if self.map[y][x + 1].content != Content::Empty {
                        (&self.map[y][x + 1], direction)
                    } else if y < 100 {
                        (&self.map[49][50 + y], Direction::Up)
                    } else if y < 150 {
                        (&self.map[49 - (y - 100)][149], Direction::Left)
                    } else {
                        (&self.map[149][y - 100], Direction::Up)
                    }
                }
            }
        }
    }
}

impl super::Day for Day22 {
    fn part_1(&mut self) -> String {
        let mut direction = Direction::Right;
        let mut pos: (usize, usize) = (
            self.map[0]
                .iter()
                .enumerate()
                .find(|(_, cell)| cell.content == Content::Path)
                .unwrap()
                .0,
            0,
        );

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Move(x) => {
                    let mut new_pos = &self.map[pos.1][pos.0];
                    let mut next_pos = &self.map[new_pos[direction].1][new_pos[direction].0];
                    for _ in 0..*x {
                        if next_pos.content != Content::Path {
                            break;
                        }
                        new_pos = next_pos;
                        next_pos = &self.map[new_pos[direction].1][new_pos[direction].0];
                    }
                    pos = new_pos.pos;
                }
                Instruction::Turn(turn) => match turn {
                    Turn::CW => {
                        direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        };
                    }
                    Turn::CCW => {
                        direction = match direction {
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Up,
                        };
                    }
                },
            }
        }

        (1000 * (pos.1 + 1)
            + 4 * (pos.0 + 1)
            + match direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            })
        .to_string()
    }

    fn part_2(&mut self) -> String {
        let mut direction = Direction::Right;
        let mut pos = self.map[0]
            .iter()
            .enumerate()
            .find(|(_, cell)| cell.content == Content::Path)
            .unwrap()
            .1;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Move(x) => {
                    let mut new_pos = pos;
                    for _ in 0..*x {
                        let (next_pos, next_dir) = self.move_on_cube(new_pos, direction);
                        if next_pos.content != Content::Path {
                            break;
                        }
                        new_pos = next_pos;
                        direction = next_dir;
                    }
                    pos = new_pos;
                }
                Instruction::Turn(turn) => match turn {
                    Turn::CW => {
                        direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        };
                    }
                    Turn::CCW => {
                        direction = match direction {
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Up,
                        };
                    }
                },
            }
        }

        (1000 * (pos.pos.1 + 1)
            + 4 * (pos.pos.0 + 1)
            + match direction {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Up => 3,
            })
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day22::new();
        assert_eq!(day.part_1(), "6032");
    }

    #[test]
    fn part_2() {
        let mut day = Day22::new();
        assert_eq!(day.part_2(), "5031");
    }
}
