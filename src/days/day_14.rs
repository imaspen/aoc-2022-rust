pub(crate) struct Day14 {
    roof: Vec<Vec<bool>>,
}

impl Day14 {
    pub(crate) fn new() -> Self {
        let rocks: Vec<Vec<(usize, usize)>> = crate::utils::read_day_lines(14)
            .iter()
            .map(|line| {
                line.split(" -> ")
                    .map(|str| {
                        let mut parts = str.split(",");
                        let x = parts.next().unwrap().parse::<usize>().unwrap();
                        let y = parts.next().unwrap().parse::<usize>().unwrap();
                        (x, y)
                    })
                    .collect()
            })
            .collect();

        let all_points: Vec<&(usize, usize)> = rocks.iter().flat_map(|path| path.iter()).collect();

        let max_y = all_points.iter().max_by_key(|coord| coord.1).unwrap().1;
        let mut roof = vec![vec![false; 1000]; 3 + max_y];

        for rock in &rocks {
            for path in rock.windows(2) {
                let start = &path[0];
                let end = &path[1];
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    for x in start.0.min(end.0)..=start.0.max(end.0) {
                        roof[y][x] = true;
                    }
                }
            }
        }

        return Self { roof };
    }

    fn drop_sand(&self) -> Option<(usize, usize)> {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == self.roof.len() {
                return None;
            } else if !self.roof[y][x] {
                y += 1;
                continue;
            } else if y == 0 {
                return None;
            } else if x == 0 {
                return None;
            } else if !self.roof[y][x - 1] {
                x -= 1;
                y += 1;
                continue;
            } else if x + 1 == self.roof[y].len() {
                return None;
            } else if !self.roof[y][x + 1] {
                x += 1;
                y += 1;
                continue;
            }
            return Some((x, y - 1));
        }
    }
}

impl super::Day for Day14 {
    fn part_1(&mut self) -> String {
        let mut dropped = 0;
        while let Some(dropped_sand) = self.drop_sand() {
            dropped += 1;
            self.roof[dropped_sand.1][dropped_sand.0] = true;
        }
        dropped.to_string()
    }

    fn part_2(&mut self) -> String {
        let len = self.roof.len();
        self.roof[len - 1].fill(true);

        let mut dropped = 0;
        while let Some(dropped_sand) = self.drop_sand() {
            dropped += 1;
            self.roof[dropped_sand.1][dropped_sand.0] = true;
        }
        dropped.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day14::new();
        assert_eq!(day.part_1(), "24");
    }

    #[test]
    fn part_2() {
        let mut day = Day14::new();
        assert_eq!(day.part_2(), "93");
    }
}
