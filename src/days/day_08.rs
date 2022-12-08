use std::{sync::Arc, thread};

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

type Forest = Arc<Vec<Vec<u8>>>;

pub(crate) struct Day08 {
    forest: Forest,
}

impl Day08 {
    pub(crate) fn new() -> Self {
        let vals = Arc::new(crate::utils::read_day_digit_lines(8));

        return Self { forest: vals };
    }

    fn visible_from(&self, x: usize, y: usize) -> usize {
        let height = self.forest[y][x];
        let column_len = self.forest.len();
        let row_len = self.forest[0].len();

        let up_forest = self.forest.clone();
        let up_handle = thread::spawn(move || {
            let mut seen = 0;
            let mut curr_y = y;
            while curr_y > 0 {
                seen += 1;
                curr_y -= 1;
                if up_forest[curr_y][x] >= height {
                    break;
                }
            }
            seen
        });

        let down_forest = self.forest.clone();
        let down_handle = thread::spawn(move || {
            let mut seen = 0;
            let mut curr_y = y + 1;
            while curr_y < column_len {
                seen += 1;
                if down_forest[curr_y][x] >= height {
                    break;
                }
                curr_y += 1;
            }
            seen
        });

        let left_forest = self.forest.clone();
        let left_handle = thread::spawn(move || {
            let mut seen = 0;
            let mut curr_x = x;
            while curr_x > 0 {
                seen += 1;
                curr_x -= 1;
                if left_forest[y][curr_x] >= height {
                    break;
                }
            }
            seen
        });

        let right_forest = self.forest.clone();
        let right_handle = thread::spawn(move || {
            let mut seen = 0;
            let mut curr_x = x + 1;
            while curr_x < row_len {
                seen += 1;
                if right_forest[y][curr_x] >= height {
                    break;
                }
                curr_x += 1;
            }
            seen
        });

        up_handle.join().unwrap()
            * down_handle.join().unwrap()
            * left_handle.join().unwrap()
            * right_handle.join().unwrap()
    }
}

impl super::Day for Day08 {
    fn part_1(&mut self) -> String {
        let rows_len = self.forest.len();
        let mut visible: Vec<Vec<bool>> = self
            .forest
            .iter()
            .enumerate()
            .map(|(i, val)| val.iter().map(|_| i == 0 || i == rows_len - 1).collect())
            .collect();

        for (y, row) in self.forest[1..self.forest.len() - 1].iter().enumerate() {
            let len = row.len();

            visible[y + 1][0] = true;
            let mut max_visited = row[0];

            // left to right
            for (x, height) in row[1..len].iter().enumerate() {
                if *height > max_visited {
                    visible[y + 1][x + 1] = true;
                    max_visited = *height;
                    if max_visited == 9 {
                        break;
                    }
                }
            }

            visible[y + 1][len - 1] = true;
            max_visited = row[len - 1];

            // right to left
            for (x, height) in row[0..len - 1].iter().enumerate().rev() {
                if visible[y + 1][x] {
                    break;
                }

                if *height > max_visited {
                    visible[y + 1][x] = true;
                    max_visited = *height;
                    if max_visited == 9 {
                        break;
                    }
                }
            }
        }

        let mut max_visited_row = self.forest[0][1..self.forest[0].len() - 1].to_owned();

        // top to bottom
        for (y, row) in self.forest[1..rows_len].iter().enumerate() {
            let len = row.len();

            if max_visited_row.iter().all(|val| *val == 9) {
                break;
            }

            // left to right
            for (x, height) in row[1..len - 1].iter().enumerate() {
                let max_visited = &mut max_visited_row[x];
                if *max_visited == 9 {
                    continue;
                }

                if *height > *max_visited {
                    visible[y + 1][x + 1] = true;
                    *max_visited = *height;
                }
            }
        }

        max_visited_row = self.forest[rows_len - 1][1..self.forest[0].len() - 1].to_owned();

        // bottom to top
        for (y, row) in self.forest[0..rows_len - 1].iter().enumerate().rev() {
            let len = row.len();

            if max_visited_row.iter().all(|val| *val == 9) {
                break;
            }

            // left to right
            for (x, height) in row[1..len - 1].iter().enumerate() {
                let max_visited = &mut max_visited_row[x];
                if *max_visited == 9 {
                    continue;
                }

                if *height > *max_visited {
                    visible[y][x + 1] = true;
                    *max_visited = *height;
                }
            }
        }

        let sum: u32 = visible
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| if *cell { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum();

        return sum.to_string();
    }

    fn part_2(&mut self) -> String {
        return self
            .forest
            .par_iter()
            .enumerate()
            .map(|(y, row)| {
                row.par_iter()
                    .enumerate()
                    .map(|(x, _)| self.visible_from(x, y))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
            .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day08::new();
        assert_eq!(day.part_1(), "21");
    }

    #[test]
    fn part_2() {
        let mut day = Day08::new();
        assert_eq!(day.part_2(), "8");
    }
}
