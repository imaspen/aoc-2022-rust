use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
    max_ore: usize,
}

impl Blueprint {
    fn from_string(string: &String) -> Self {
        let mut parts = REGEX
            .captures_iter(string)
            .map(|capture| capture[0].parse().unwrap());

        parts.next();

        let ore_robot_cost = parts.next().unwrap();
        let clay_robot_cost = parts.next().unwrap();
        let obsidian_robot_cost = (parts.next().unwrap(), parts.next().unwrap());
        let geode_robot_cost = (parts.next().unwrap(), parts.next().unwrap());
        Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
            max_ore: ore_robot_cost
                .max(clay_robot_cost.max(obsidian_robot_cost.0.max(geode_robot_cost.0))),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    skipped_making_ore_robots: bool,
    skipped_making_clay_robots: bool,
    skipped_making_obsidian_robots: bool,
}

impl State {
    fn generate_substates(&self, time: usize, target_time: usize, blueprint: &Blueprint) -> usize {
        if time >= target_time {
            return self.geodes;
        }

        let mut substates: Vec<State> = Vec::with_capacity(4);
        let mut base_state = self.clone();
        base_state.ore += self.ore_robots;
        base_state.clay += self.clay_robots;
        base_state.obsidian += self.obsidian_robots;
        base_state.geodes += self.geode_robots;

        let can_make_geode_robot = self.ore >= blueprint.geode_robot_cost.0
            && self.obsidian >= blueprint.geode_robot_cost.1;
        if can_make_geode_robot {
            let mut geode_robot = base_state.clone();
            geode_robot.ore -= blueprint.geode_robot_cost.0;
            geode_robot.obsidian -= blueprint.geode_robot_cost.1;
            geode_robot.geode_robots += 1;
            geode_robot.skipped_making_ore_robots = false;
            geode_robot.skipped_making_clay_robots = false;
            geode_robot.skipped_making_obsidian_robots = false;
            substates.push(geode_robot);
        } else {
            let can_make_ore_robot = self.ore >= blueprint.ore_robot_cost;
            if !self.skipped_making_ore_robots
                && self.ore_robots < blueprint.max_ore
                && can_make_ore_robot
            {
                let mut ore_robot = base_state.clone();
                ore_robot.ore -= blueprint.ore_robot_cost;
                ore_robot.ore_robots += 1;
                ore_robot.skipped_making_ore_robots = false;
                ore_robot.skipped_making_clay_robots = false;
                ore_robot.skipped_making_obsidian_robots = false;
                substates.push(ore_robot);
            }

            let can_make_clay_robot = self.ore >= blueprint.clay_robot_cost;
            if !self.skipped_making_clay_robots
                && self.clay_robots < blueprint.obsidian_robot_cost.1
                && can_make_clay_robot
            {
                let mut clay_robot = base_state.clone();
                clay_robot.ore -= blueprint.clay_robot_cost;
                clay_robot.clay_robots += 1;
                clay_robot.skipped_making_ore_robots = false;
                clay_robot.skipped_making_clay_robots = false;
                clay_robot.skipped_making_obsidian_robots = false;
                substates.push(clay_robot);
            }

            let can_make_obsidian_robot = self.ore >= blueprint.obsidian_robot_cost.0
                && self.obsidian_robots < blueprint.geode_robot_cost.1
                && self.clay >= blueprint.obsidian_robot_cost.1;
            if !self.skipped_making_obsidian_robots && can_make_obsidian_robot {
                let mut obsidian_robot = base_state.clone();
                obsidian_robot.ore -= blueprint.obsidian_robot_cost.0;
                obsidian_robot.clay -= blueprint.obsidian_robot_cost.1;
                obsidian_robot.obsidian_robots += 1;
                obsidian_robot.skipped_making_ore_robots = false;
                obsidian_robot.skipped_making_clay_robots = false;
                obsidian_robot.skipped_making_obsidian_robots = false;
                substates.push(obsidian_robot);
            }

            base_state.skipped_making_ore_robots = can_make_ore_robot;
            base_state.skipped_making_clay_robots = can_make_clay_robot;
            base_state.skipped_making_obsidian_robots = can_make_obsidian_robot;
            substates.push(base_state);
        }

        let mut max = 0;
        for substate in substates.iter_mut() {
            let count = substate.generate_substates(time + 1, target_time, blueprint);
            if count > max {
                max = count;
            }
        }
        return max;
    }
}

pub(crate) struct Day19 {
    blueprints: Box<[Blueprint]>,
}

impl Day19 {
    pub(crate) fn new() -> Self {
        let blueprints = crate::utils::read_day_lines(19)
            .iter()
            .map(Blueprint::from_string)
            .collect();
        return Self { blueprints };
    }
}

impl super::Day for Day19 {
    fn part_1(&mut self) -> String {
        let mut total = 0;
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            let start_state = State {
                ore_robots: 1,
                ..Default::default()
            };
            let count = start_state.generate_substates(0, 24, blueprint);
            total += count * (i + 1);
        }
        total.to_string()
    }

    fn part_2(&mut self) -> String {
        let mut total = 1;
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            if i >= 3 {
                break;
            }
            let start_state = State {
                ore_robots: 1,
                ..Default::default()
            };
            let count = start_state.generate_substates(0, 32, blueprint);
            total *= count;
        }
        total.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day19::new();
        assert_eq!(day.part_1(), "33");
    }

    #[test]
    fn part_2() {
        let mut day = Day19::new();
        assert_eq!(day.part_2(), "3472");
    }
}
