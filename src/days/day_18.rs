use std::{collections::VecDeque, rc::Rc};

struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn from_string(string: &String) -> Self {
        let mut parts = string.split(",");
        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}

type Node = Box<[Coord]>;

const SIDE_LENGTH: usize = 25;
type VoxelGrid = [[[bool; SIDE_LENGTH]; SIDE_LENGTH]; SIDE_LENGTH];
pub(crate) struct Day18 {
    voxels: VoxelGrid,
    voxel_count: usize,
}

impl Day18 {
    pub(crate) fn new() -> Self {
        let mut voxels: VoxelGrid = [[[false; SIDE_LENGTH]; SIDE_LENGTH]; SIDE_LENGTH];
        let lines = crate::utils::read_day_lines(18);
        for Coord { x, y, z } in lines.iter().map(Coord::from_string) {
            voxels[z][y][x] = true;
        }

        return Self {
            voxels,
            voxel_count: lines.len(),
        };
    }
}

impl super::Day for Day18 {
    fn part_1(&mut self) -> String {
        let mut count: usize = 0;
        for z in 0..SIDE_LENGTH {
            for y in 0..SIDE_LENGTH {
                for x in 0..SIDE_LENGTH {
                    if !self.voxels[z][y][x] {
                        continue;
                    }
                    if x == 0 || !self.voxels[z][y][x - 1] {
                        count += 1;
                    }
                    if x == SIDE_LENGTH - 1 || !self.voxels[z][y][x + 1] {
                        count += 1;
                    }
                    if y == 0 || !self.voxels[z][y - 1][x] {
                        count += 1;
                    }
                    if y == SIDE_LENGTH - 1 || !self.voxels[z][y + 1][x] {
                        count += 1;
                    }
                    if z == 0 || !self.voxels[z - 1][y][x] {
                        count += 1;
                    }
                    if z == SIDE_LENGTH - 1 || !self.voxels[z + 1][y][x] {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn part_2(&mut self) -> String {
        let mut air_graph: [[[Option<Rc<Node>>; SIDE_LENGTH]; SIDE_LENGTH]; SIDE_LENGTH] =
            Default::default();

        for z in 0..SIDE_LENGTH {
            for y in 0..SIDE_LENGTH {
                for x in 0..SIDE_LENGTH {
                    if self.voxels[z][y][x] {
                        continue;
                    }
                    let mut neighbours = Vec::with_capacity(6);

                    if x > 0 && !self.voxels[z][y][x - 1] {
                        neighbours.push(Coord { x: x - 1, y, z });
                    }
                    if x < SIDE_LENGTH - 1 && !self.voxels[z][y][x + 1] {
                        neighbours.push(Coord { x: x + 1, y, z });
                    }
                    if y > 0 && !self.voxels[z][y - 1][x] {
                        neighbours.push(Coord { x, y: y - 1, z });
                    }
                    if y < SIDE_LENGTH - 1 && !self.voxels[z][y + 1][x] {
                        neighbours.push(Coord { x, y: y + 1, z });
                    }
                    if z > 0 && !self.voxels[z - 1][y][x] {
                        neighbours.push(Coord { x, y, z: z - 1 });
                    }
                    if z < SIDE_LENGTH - 1 && !self.voxels[z + 1][y][x] {
                        neighbours.push(Coord { x, y, z: z + 1 });
                    }

                    air_graph[z][y][x] = Some(Rc::new(neighbours.into_boxed_slice()));
                }
            }
        }

        let mut outer_air = [[[false; SIDE_LENGTH]; SIDE_LENGTH]; SIDE_LENGTH];
        let mut queue: VecDeque<Rc<Node>> =
            VecDeque::with_capacity((SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH) - self.voxel_count);

        outer_air[0][0][0] = true;
        queue.push_back((air_graph[0][0][0]).clone().unwrap());

        while let Some(node) = queue.pop_front() {
            for neighbour in node.iter() {
                if outer_air[neighbour.z][neighbour.y][neighbour.x] {
                    continue;
                }
                outer_air[neighbour.z][neighbour.y][neighbour.x] = true;
                queue.push_back(
                    air_graph[neighbour.z][neighbour.y][neighbour.x]
                        .clone()
                        .unwrap(),
                );
            }
        }

        let mut count: usize = 0;
        for z in 0..SIDE_LENGTH {
            for y in 0..SIDE_LENGTH {
                for x in 0..SIDE_LENGTH {
                    if !self.voxels[z][y][x] {
                        continue;
                    }
                    if x == 0 || outer_air[z][y][x - 1] {
                        count += 1;
                    }
                    if x == SIDE_LENGTH - 1 || outer_air[z][y][x + 1] {
                        count += 1;
                    }
                    if y == 0 || outer_air[z][y - 1][x] {
                        count += 1;
                    }
                    if y == SIDE_LENGTH - 1 || outer_air[z][y + 1][x] {
                        count += 1;
                    }
                    if z == 0 || outer_air[z - 1][y][x] {
                        count += 1;
                    }
                    if z == SIDE_LENGTH - 1 || outer_air[z + 1][y][x] {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day18::new();
        assert_eq!(day.part_1(), "64");
    }

    #[test]
    fn part_2() {
        let mut day = Day18::new();
        assert_eq!(day.part_2(), "58");
    }
}
