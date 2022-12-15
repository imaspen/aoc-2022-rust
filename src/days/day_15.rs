use std::{collections::HashSet, ops::Range};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    closest_beacon: (i64, i64),
    beacon_dist: u64,
}

lazy_static! {
    static ref SENSOR_REGEX: Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
}

pub(crate) struct Day15 {
    target_row: i64,
    sensors: Box<[Sensor]>,
}

impl Day15 {
    pub(crate) fn new() -> Self {
        let vals = crate::utils::read_day_grouped_lines(15);
        let target_row = vals[0][0].parse().unwrap();

        let sensors = vals[1]
            .iter()
            .map(|line| {
                let captures = SENSOR_REGEX.captures(line).unwrap();
                let pos: (i64, i64) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
                let closest_beacon = (captures[3].parse().unwrap(), captures[4].parse().unwrap());
                let beacon_dist =
                    pos.0.abs_diff(closest_beacon.0) + pos.1.abs_diff(closest_beacon.1);

                Sensor {
                    pos,
                    closest_beacon,
                    beacon_dist,
                }
            })
            .collect();

        return Self {
            target_row,
            sensors,
        };
    }

    fn get_ranges_for_row(&self, row: i64) -> Vec<Range<i64>> {
        let mut ranges: Vec<_> = self
            .sensors
            .iter()
            .filter_map(|sensor| {
                let offset =
                    i64::try_from(sensor.beacon_dist.checked_sub(sensor.pos.1.abs_diff(row))?)
                        .unwrap();
                Some((sensor.pos.0 - offset)..(1 + sensor.pos.0 + offset))
            })
            .collect();
        ranges.sort_unstable_by_key(|range| range.start);

        let mut minimal_ranges: Vec<Range<i64>> = vec![];
        let mut curr_range = ranges[0].clone();

        for range in &ranges[1..] {
            if range.start > curr_range.end {
                minimal_ranges.push(curr_range);
                curr_range = range.clone();
            } else if range.end > curr_range.end {
                curr_range.end = range.end;
            }
        }

        minimal_ranges.push(curr_range);

        minimal_ranges
    }

    fn get_distress_signal_tuning_frequency(&self) -> Option<i64> {
        for i in 0..=(2 * self.target_row) {
            let ranges = self.get_ranges_for_row(i);
            let mut ranges_iter = ranges.iter();
            let mut range = ranges_iter.next();
            while range.unwrap().end <= 0 {
                range = ranges_iter.next();
            }

            if range.unwrap().end <= 2 * self.target_row {
                return Some(4_000_000 * range.unwrap().end + i);
            }
        }
        return None;
    }
}

impl super::Day for Day15 {
    fn part_1(&mut self) -> String {
        let mut beacons = HashSet::new();
        self.sensors.iter().fold(&mut beacons, |set, sensor| {
            if sensor.closest_beacon.1 == self.target_row {
                set.insert(sensor.closest_beacon.1);
            }
            set
        });

        let minimal_ranges = self.get_ranges_for_row(self.target_row);

        let invalid_points = minimal_ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum::<i64>()
            - i64::try_from(beacons.len()).unwrap();

        invalid_points.to_string()
    }

    fn part_2(&mut self) -> String {
        self.get_distress_signal_tuning_frequency()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day15::new();
        assert_eq!(day.part_1(), "26");
    }

    #[test]
    fn part_2() {
        let mut day = Day15::new();
        assert_eq!(day.part_2(), "56000011");
    }
}
