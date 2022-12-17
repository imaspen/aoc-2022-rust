use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

type NodeIndex = u64;
type Time = i64;

#[derive(Clone, Debug)]
struct Node {
    flow: i64,
    neighbours: Box<[NodeIndex]>,
}

pub(crate) struct Day16 {
    nodes: HashMap<NodeIndex, Node>,
    start_id: NodeIndex,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]{2}, )*[A-Z]{2})$"
    )
    .unwrap();
}

fn get_index(str: &String, indexes: &mut HashMap<String, NodeIndex>, index: &mut u8) -> NodeIndex {
    if !indexes.contains_key(str) {
        indexes.insert(str.to_owned(), 1 << *index);
        *index += 1;
    }

    *indexes.get(str).unwrap()
}

impl Day16 {
    pub(crate) fn new() -> Self {
        let mut nodes: HashMap<NodeIndex, Node> = HashMap::new();
        let mut indexes: HashMap<String, NodeIndex> = HashMap::new();
        let mut index = 0;

        for line in crate::utils::read_day_lines(16).iter() {
            let captures = REGEX.captures(line).unwrap();
            nodes.insert(
                get_index(&captures[1].to_owned(), &mut indexes, &mut index),
                Node {
                    flow: captures[2].parse().unwrap(),
                    neighbours: captures[3]
                        .split(", ")
                        .map(|str| get_index(&str.to_owned(), &mut indexes, &mut index))
                        .collect(),
                },
            );
        }

        return Self {
            nodes,
            start_id: *indexes.get(&"AA".to_owned()).unwrap(),
        };
    }
}

struct P1State(NodeIndex, NodeIndex, i64);
struct P2State(NodeIndex, NodeIndex, NodeIndex, i64);

impl super::Day for Day16 {
    fn part_1(&mut self) -> String {
        let run_time: Time = 30;
        let mut states: Vec<P1State> = vec![P1State(self.start_id, 0, 0)];
        let mut best: HashMap<(NodeIndex, NodeIndex), i64> = HashMap::new();

        for time in 1..=run_time {
            let mut new_states: Vec<P1State> = vec![];

            for P1State(node_index, opened_mask, pressure) in states {
                let key = (node_index, opened_mask);
                if *best.get(&key).unwrap_or(&-1) >= pressure {
                    continue;
                }

                best.insert(key, pressure);
                let node = self.nodes.get(&node_index).unwrap();

                if node_index & opened_mask == 0 && node.flow > 0 {
                    new_states.push(P1State(
                        node_index,
                        opened_mask | node_index,
                        pressure + node.flow * (run_time - time),
                    ));
                }

                for neighbour in node.neighbours.iter() {
                    new_states.push(P1State(*neighbour, opened_mask, pressure));
                }
            }

            states = new_states;
        }

        best.iter().map(|state| state.1).max().unwrap().to_string()
    }

    fn part_2(&mut self) -> String {
        let run_time: Time = 26;
        let mut states: Vec<P2State> = vec![P2State(self.start_id, self.start_id, 0, 0)];
        let mut best: HashMap<(NodeIndex, NodeIndex, NodeIndex), i64> = HashMap::new();

        for time in 1..=run_time {
            println!("{}, {}", time, states.len());

            let mut new_states: Vec<P2State> = vec![];

            for P2State(node_index_1, node_index_2, opened_mask, pressure) in states {
                let key = (
                    node_index_1.min(node_index_2),
                    node_index_1.max(node_index_2),
                    opened_mask,
                );
                if *best.get(&key).unwrap_or(&-1) >= pressure {
                    continue;
                }
                best.insert(key, pressure);

                let node_1 = self.nodes.get(&node_index_1).unwrap();
                let node_2 = self.nodes.get(&node_index_2).unwrap();

                let can_open_1 = node_index_1 & opened_mask == 0 && node_1.flow > 0;
                let can_open_2 = (node_index_1 != node_index_2)
                    && (node_index_2 & opened_mask == 0)
                    && (node_2.flow > 0);

                if can_open_1 {
                    if can_open_2 {
                        new_states.push(P2State(
                            node_index_1,
                            node_index_2,
                            opened_mask | node_index_1 | node_index_2,
                            pressure
                                + node_1.flow * (run_time - time)
                                + node_2.flow * (run_time - time),
                        ));
                    }
                    for neighbour in node_2.neighbours.iter() {
                        new_states.push(P2State(
                            node_index_1,
                            *neighbour,
                            opened_mask | node_index_1,
                            pressure + node_1.flow * (run_time - time),
                        ));
                    }
                }

                if can_open_2 {
                    for neighbour in node_1.neighbours.iter() {
                        new_states.push(P2State(
                            *neighbour,
                            node_index_2,
                            opened_mask | node_index_2,
                            pressure + node_2.flow * (run_time - time),
                        ));
                    }
                }

                for neighbour_1 in node_1.neighbours.iter() {
                    for neighbour_2 in node_2.neighbours.iter() {
                        new_states.push(P2State(*neighbour_1, *neighbour_2, opened_mask, pressure));
                    }
                }
            }

            states = new_states;
        }

        best.iter().map(|state| *state.1).max().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day16::new();
        assert_eq!(day.part_1(), "1651");
    }

    #[test]
    fn part_2() {
        let mut day = Day16::new();
        assert_eq!(day.part_2(), "1707");
    }
}
