#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Node {
    position: Coord,
    height: u8,
    neighbours: Box<[Coord]>,
    dist: usize,
    prev: Option<Coord>,
}

pub(crate) struct Day12 {
    grid: Vec<Vec<Node>>,
    start_pos: Coord,
    end_pos: Coord,
}

impl Day12 {
    pub(crate) fn new() -> Self {
        let mut start_pos: Option<Coord> = None;
        let mut end_pos: Option<Coord> = None;

        let vals = crate::utils::read_day_lines(12)
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        'S' => {
                            start_pos = Some(Coord { x, y });
                            u8::try_from('a').unwrap()
                        }
                        'E' => {
                            end_pos = Some(Coord { x, y });
                            u8::try_from('z').unwrap()
                        }
                        _ => u8::try_from(char).unwrap(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = vals
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, height)| {
                        let mut neighbours = Vec::with_capacity(4);
                        if y > 0 && vals[y - 1][x] <= *height + 1 {
                            neighbours.push(Coord { x, y: y - 1 });
                        }
                        if y < vals.len() - 1 && vals[y + 1][x] <= *height + 1 {
                            neighbours.push(Coord { x, y: y + 1 });
                        }
                        if x > 0 && vals[y][x - 1] <= *height + 1 {
                            neighbours.push(Coord { x: x - 1, y });
                        }
                        if x < row.len() - 1 && vals[y][x + 1] <= *height + 1 {
                            neighbours.push(Coord { x: x + 1, y });
                        }

                        Node {
                            position: Coord { x, y },
                            height: *height,
                            neighbours: neighbours.into_boxed_slice(),
                            dist: usize::MAX,
                            prev: None,
                        }
                    })
                    .collect()
            })
            .collect();

        return Self {
            grid,
            start_pos: start_pos.unwrap(),
            end_pos: end_pos.unwrap(),
        };
    }

    fn run_search(&mut self) {
        let mut queue: Vec<Coord> = self
            .grid
            .iter()
            .flat_map(|row| row.iter().map(|node| node.position))
            .collect();

        while let Some(coord) = {
            queue.sort_by_cached_key(|coord| usize::MAX - self.grid[coord.y][coord.x].dist);
            queue.pop()
        } {
            let node = &self.grid[coord.y][coord.x];
            let node_dist = node.dist;
            let node_pos = node.position;
            let node_neighbours = node.neighbours.clone();

            if node_pos == self.end_pos {
                break;
            }

            for neighbour_coords in node_neighbours.iter() {
                if !queue.contains(neighbour_coords) {
                    continue;
                }

                let neighbour = &mut self.grid[neighbour_coords.y][neighbour_coords.x];

                let alt = node_dist + 1;
                if alt < neighbour.dist {
                    neighbour.dist = alt;
                    neighbour.prev = Some(node_pos);
                }
            }
        }
    }
}

impl super::Day for Day12 {
    fn part_1(&mut self) -> String {
        self.grid[self.start_pos.y][self.start_pos.x].dist = 0;

        self.run_search();

        self.grid[self.end_pos.y][self.end_pos.x].dist.to_string()
    }

    fn part_2(&mut self) -> String {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|node| {
                if node.height == u8::try_from('a').unwrap() {
                    node.dist = 0;
                }
            })
        });

        self.run_search();

        self.grid[self.end_pos.y][self.end_pos.x].dist.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day12::new();
        assert_eq!(day.part_1(), "31");
    }

    #[test]
    fn part_2() {
        let mut day = Day12::new();
        assert_eq!(day.part_2(), "29");
    }
}
