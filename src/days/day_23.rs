use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Coord = (isize, isize);

#[derive(Debug)]
struct Elf {
    pos: Coord,
    proposed_move: Option<Coord>,
}

impl Elf {
    fn is_lonely(&self, map: &HashMap<(isize, isize), Rc<RefCell<Elf>>>) -> bool {
        for y in (self.pos.1 - 1)..=(self.pos.1 + 1) {
            for x in (self.pos.0 - 1)..=(self.pos.0 + 1) {
                if x == self.pos.0 && y == self.pos.1 {
                    continue;
                }
                if map.contains_key(&(x, y)) {
                    return false;
                }
            }
        }
        true
    }

    fn get_direction_move(
        &self,
        dir: &Dir,
        map: &HashMap<(isize, isize), Rc<RefCell<Elf>>>,
    ) -> Option<(isize, isize)> {
        let x = self.pos.0;
        let y = self.pos.1;
        match dir {
            Dir::North => {
                if map.contains_key(&(x - 1, y - 1))
                    || map.contains_key(&(x, y - 1))
                    || map.contains_key(&(x + 1, y - 1))
                {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Dir::East => {
                if map.contains_key(&(x + 1, y - 1))
                    || map.contains_key(&(x + 1, y))
                    || map.contains_key(&(x + 1, y + 1))
                {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Dir::South => {
                if map.contains_key(&(x - 1, y + 1))
                    || map.contains_key(&(x, y + 1))
                    || map.contains_key(&(x + 1, y + 1))
                {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Dir::West => {
                if map.contains_key(&(x - 1, y - 1))
                    || map.contains_key(&(x - 1, y))
                    || map.contains_key(&(x - 1, y + 1))
                {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

const DIRS: [[Dir; 4]; 4] = [
    [Dir::North, Dir::South, Dir::West, Dir::East],
    [Dir::South, Dir::West, Dir::East, Dir::North],
    [Dir::West, Dir::East, Dir::North, Dir::South],
    [Dir::East, Dir::North, Dir::South, Dir::West],
];

pub(crate) struct Day23 {
    elves: Box<[Rc<RefCell<Elf>>]>,
}

impl Day23 {
    pub(crate) fn new() -> Self {
        let elves = crate::utils::read_day_lines(23)
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '#' => Some(Rc::new(RefCell::new(Elf {
                            pos: (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()),
                            proposed_move: None,
                        }))),
                        _ => None,
                    })
            })
            .collect();
        return Self { elves };
    }
}

trait ElfMap {
    fn print_map(&self);
    fn get_empty_space(&self) -> usize;
}

impl ElfMap for HashMap<(isize, isize), Rc<RefCell<Elf>>> {
    fn print_map(&self) {
        let min_x = self.keys().min_by_key(|coord| coord.0).unwrap().0;
        let min_y = self.keys().min_by_key(|coord| coord.1).unwrap().1;
        let max_x = self.keys().max_by_key(|coord| coord.0).unwrap().0;
        let max_y = self.keys().max_by_key(|coord| coord.1).unwrap().1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", if self.contains_key(&(x, y)) { '#' } else { '.' })
            }
            println!();
        }
        println!();
    }

    fn get_empty_space(&self) -> usize {
        let min_x = self.keys().min_by_key(|coord| coord.0).unwrap().0;
        let min_y = self.keys().min_by_key(|coord| coord.1).unwrap().1;
        let max_x = self.keys().max_by_key(|coord| coord.0).unwrap().0;
        let max_y = self.keys().max_by_key(|coord| coord.1).unwrap().1;

        usize::try_from((1 + max_x - min_x) * (1 + max_y - min_y)).unwrap() - self.len()
    }
}

impl super::Day for Day23 {
    fn part_1(&mut self) -> String {
        let mut map: HashMap<(isize, isize), Rc<RefCell<Elf>>> = self
            .elves
            .iter()
            .map(|elf| (elf.borrow().pos, Rc::clone(elf)))
            .collect();

        let mut dirs_iter = DIRS.iter().cycle();
        let mut i: usize = 0;

        loop {
            let mut proposed_moves: HashMap<(isize, isize), bool> = HashMap::new();
            let dirs = dirs_iter.next().unwrap();
            for elf_rc in self.elves.iter() {
                let mut elf = elf_rc.borrow_mut();
                elf.proposed_move = None;
                if elf.is_lonely(&map) {
                    continue;
                }
                for dir in dirs.iter() {
                    if let Some(proposed_move) = elf.get_direction_move(dir, &map) {
                        elf.proposed_move = Some(proposed_move);
                        if let Some(can_move) = proposed_moves.get_mut(&proposed_move) {
                            *can_move = false;
                        } else {
                            proposed_moves.insert(proposed_move, true);
                        }
                        break;
                    }
                }
            }

            for elf_rc in self.elves.iter() {
                let mut elf = elf_rc.borrow_mut();
                if let Some(proposed_move) = elf.proposed_move {
                    if *proposed_moves.get(&proposed_move).unwrap() {
                        map.remove(&elf.pos);
                        elf.pos = proposed_move;
                        map.insert(elf.pos, Rc::clone(elf_rc));
                    }
                }
            }

            i += 1;
            if i >= 10 {
                break;
            }
        }

        map.get_empty_space().to_string()
    }

    fn part_2(&mut self) -> String {
        let mut map: HashMap<(isize, isize), Rc<RefCell<Elf>>> = self
            .elves
            .iter()
            .map(|elf| (elf.borrow().pos, Rc::clone(elf)))
            .collect();

        let mut dirs_iter = DIRS.iter().cycle();
        let mut i: usize = 0;

        loop {
            let mut proposed_moves: HashMap<(isize, isize), bool> = HashMap::new();
            let dirs = dirs_iter.next().unwrap();
            for elf_rc in self.elves.iter() {
                let mut elf = elf_rc.borrow_mut();
                elf.proposed_move = None;
                if elf.is_lonely(&map) {
                    continue;
                }
                for dir in dirs.iter() {
                    if let Some(proposed_move) = elf.get_direction_move(dir, &map) {
                        elf.proposed_move = Some(proposed_move);
                        if let Some(can_move) = proposed_moves.get_mut(&proposed_move) {
                            *can_move = false;
                        } else {
                            proposed_moves.insert(proposed_move, true);
                        }
                        break;
                    }
                }
            }

            let mut any_moved = false;
            for elf_rc in self.elves.iter() {
                let mut elf = elf_rc.borrow_mut();
                if let Some(proposed_move) = elf.proposed_move {
                    if *proposed_moves.get(&proposed_move).unwrap() {
                        map.remove(&elf.pos);
                        elf.pos = proposed_move;
                        map.insert(elf.pos, Rc::clone(elf_rc));
                        any_moved = true;
                    }
                }
            }

            i += 1;
            if !any_moved {
                break;
            }
        }

        i.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day23::new();
        assert_eq!(day.part_1(), "110");
    }

    #[test]
    fn part_2() {
        let mut day = Day23::new();
        assert_eq!(day.part_2(), "20");
    }
}
