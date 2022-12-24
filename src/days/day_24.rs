use std::collections::HashMap;

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Dir::Up),
            'v' => Ok(Dir::Down),
            '>' => Ok(Dir::Right),
            '<' => Ok(Dir::Left),
            _ => Err(format!("Unexpected dir char: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    pos: Coord,
    dir: Dir,
}

pub(crate) struct Day24 {
    starting_blizzards: Box<[Blizzard]>,
    width: usize,
    height: usize,
}

type Map = Box<[Box<[usize]>]>;

trait BlizzardBox {
    fn next_blizzards(self, width: usize, height: usize) -> Self;
    fn make_map(&self, width: usize, height: usize) -> Map;
}
impl BlizzardBox for Box<[Blizzard]> {
    fn next_blizzards(mut self, width: usize, height: usize) -> Self {
        for blizzard in self.iter_mut() {
            match blizzard.dir {
                Dir::Up => blizzard.pos.1 -= 1,
                Dir::Down => blizzard.pos.1 += 1,
                Dir::Left => blizzard.pos.0 -= 1,
                Dir::Right => blizzard.pos.0 += 1,
            }

            if blizzard.pos.0 == 0 {
                blizzard.pos.0 = width - 2;
            } else if blizzard.pos.0 == width - 1 {
                blizzard.pos.0 = 1;
            }

            if blizzard.pos.1 == 0 {
                blizzard.pos.1 = height - 2;
            } else if blizzard.pos.1 == height - 1 {
                blizzard.pos.1 = 1;
            }
        }
        self
    }

    fn make_map(&self, width: usize, height: usize) -> Map {
        let mut map: Map = (0..height)
            .map(|_| (0..width).map(|_| 0).collect())
            .collect();

        for y in 0..height {
            map[y][0] += 1;
            map[y][width - 1] += 1;
        }

        for x in 1..(width - 1) {
            if x != 1 {
                map[0][x] += 1;
            }
            if x != width - 2 {
                map[height - 1][x] += 1;
            }
        }

        for blizzard in self.iter() {
            map[blizzard.pos.1][blizzard.pos.0] += 1;
        }

        return map;
    }
}

trait BlizzardMap {
    fn print_map(&self);
}
impl BlizzardMap for Map {
    fn print_map(&self) {
        for row in self.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!()
        }
        println!()
    }
}

impl Day24 {
    pub(crate) fn new() -> Self {
        let lines = crate::utils::read_day_lines(24);
        let starting_blizzards: Box<[Blizzard]> = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '#' | '.' => None,
                        _ => Some(Blizzard {
                            pos: (x, y),
                            dir: Dir::try_from(char).unwrap(),
                        }),
                    })
            })
            .collect();

        return Self {
            starting_blizzards,
            height: lines.len(),
            width: lines[0].len(),
        };
    }

    fn search(&self, needs_snacks: bool) -> usize {
        let target = (self.width - 2, self.height - 1);
        let start_node = Node {
            pos: (1, 0),
            time: 0,
            has_visited_end: false,
            has_revisited_start: false,
        };

        let mut maps: HashMap<usize, (Box<[Blizzard]>, Map)> = HashMap::new();
        maps.insert(
            0,
            (
                self.starting_blizzards.clone(),
                self.starting_blizzards.make_map(self.width, self.height),
            ),
        );

        let mut open_set: Vec<Node> = vec![start_node];

        let mut came_from: HashMap<Node, Node> = HashMap::new();

        let mut g_scores: HashMap<Node, usize> = HashMap::new();
        g_scores.insert(start_node, 0);

        let mut f_scores: HashMap<Node, usize> = HashMap::new();
        f_scores.insert(start_node, start_node.h(&target, needs_snacks));

        while !open_set.is_empty() {
            open_set.sort_by_cached_key(|node| usize::MAX - f_scores.get(node).unwrap());
            let current = open_set.pop().unwrap();
            if current.pos == target
                && (!needs_snacks || (current.has_visited_end && current.has_revisited_start))
            {
                return current.time;
            }

            if !maps.contains_key(&(current.time + 1)) {
                let (current_blizzards, _) = maps.get(&current.time).unwrap();
                let next_blizzards = current_blizzards
                    .clone()
                    .next_blizzards(self.width, self.height);
                let next_map = next_blizzards.make_map(self.width, self.height);
                maps.insert(current.time + 1, (next_blizzards, next_map));
            }

            let (_, map) = maps.get(&(current.time + 1)).unwrap();
            let neighbours = current.generate_neighbours(&map, self.width, self.height);

            let next_g_score = g_scores.get(&current).unwrap() + 1;
            for neighbour in neighbours {
                if next_g_score < *g_scores.get(&neighbour).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbour, current);
                    g_scores.insert(neighbour, next_g_score);
                    f_scores.insert(neighbour, next_g_score + neighbour.h(&target, needs_snacks));
                    if !open_set.contains(&neighbour) {
                        open_set.push(neighbour);
                    }
                }
            }
        }

        panic!("Solution could not be found.");
    }
}

// Location, time
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    pos: Coord,
    time: usize,
    has_visited_end: bool,
    has_revisited_start: bool,
}

impl Node {
    fn h(&self, target: &Coord, needs_snacks: bool) -> usize {
        let dist_to_end = target.0.abs_diff(self.pos.0) + target.1.abs_diff(self.pos.1);
        let dist_start_to_end = target.0.abs_diff(1) + target.1;
        if needs_snacks {
            if self.has_visited_end && self.has_revisited_start {
                dist_to_end
            } else if self.has_visited_end {
                dist_start_to_end + self.pos.0.abs_diff(1) + self.pos.1
            } else {
                dist_to_end + (dist_start_to_end * 2)
            }
        } else {
            dist_to_end
        }
    }

    fn generate_neighbours(&self, map: &Map, width: usize, height: usize) -> Vec<Node> {
        let mut out = Vec::with_capacity(5);

        let time = self.time + 1;
        let x = self.pos.0;
        let y = self.pos.1;

        let start = (1, 0);
        let target = (width - 2, height - 1);

        if map[y][x] == 0 {
            out.push(Node {
                pos: self.pos,
                time,
                has_visited_end: self.has_visited_end,
                has_revisited_start: self.has_revisited_start,
            });
        }
        if x > 1 && map[y][x - 1] == 0 {
            out.push(Node {
                pos: (x - 1, y),
                time,
                has_visited_end: self.has_visited_end || (x - 1, y) == target,
                has_revisited_start: self.has_revisited_start
                    || (self.has_visited_end && (x - 1, y) == start),
            });
        }
        if x < width - 2 && map[y][x + 1] == 0 {
            out.push(Node {
                pos: (x + 1, y),
                time,
                has_visited_end: self.has_visited_end || (x + 1, y) == target,
                has_revisited_start: self.has_revisited_start
                    || (self.has_visited_end && (x + 1, y) == start),
            });
        }
        if y > 0 && map[y - 1][x] == 0 {
            out.push(Node {
                pos: (x, y - 1),
                time,
                has_visited_end: self.has_visited_end || (x, y - 1) == target,
                has_revisited_start: self.has_revisited_start
                    || (self.has_visited_end && (x, y - 1) == start),
            });
        }
        if y < height - 1 && map[y + 1][x] == 0 {
            out.push(Node {
                pos: (x, y + 1),
                time,
                has_visited_end: self.has_visited_end || (x, y + 1) == target,
                has_revisited_start: self.has_revisited_start
                    || (self.has_visited_end && (x, y + 1) == start),
            });
        }

        out
    }
}

impl super::Day for Day24 {
    fn part_1(&mut self) -> String {
        self.search(false).to_string()
    }

    fn part_2(&mut self) -> String {
        self.search(true).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day24::new();
        assert_eq!(day.part_1(), "18");
    }

    #[test]
    fn part_2() {
        let mut day = Day24::new();
        assert_eq!(day.part_2(), "54");
    }
}
