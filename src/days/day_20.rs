use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug)]
struct Node {
    value: i64,
    previous: Weak<RefCell<Node>>,
    next: Weak<RefCell<Node>>,
}

pub(crate) struct Day20 {
    nodes: Box<[Rc<RefCell<Node>>]>,
    start: Rc<RefCell<Node>>,
}

impl Day20 {
    pub(crate) fn new() -> Self {
        let nodes: Box<[Rc<RefCell<Node>>]> = crate::utils::read_day_parsed_lines(20)
            .into_iter()
            .map(|value| {
                Rc::new(RefCell::new(Node {
                    value,
                    previous: Weak::new(),
                    next: Weak::new(),
                }))
            })
            .collect();

        let len = nodes.len();
        for i in 0..len {
            nodes[usize::try_from(i).unwrap()].borrow_mut().previous = Rc::downgrade(
                &nodes[usize::try_from(
                    (isize::try_from(i).unwrap() - 1).rem_euclid(isize::try_from(len).unwrap()),
                )
                .unwrap()],
            );
            nodes[usize::try_from(i).unwrap()].borrow_mut().next =
                Rc::downgrade(&nodes[(i + 1).rem_euclid(len)]);
        }

        let start = Rc::clone(&nodes.iter().find(|val| val.borrow().value == 0).unwrap());

        return Self { nodes, start };
    }

    #[allow(dead_code)]
    fn print_nodes(&self) {
        let mut current = Rc::downgrade(&self.start);
        loop {
            println!("{}", current.upgrade().unwrap().borrow().value);

            current = Weak::clone(&current.upgrade().unwrap().borrow().next);

            if current.upgrade().unwrap().borrow().value == 0 {
                break;
            }
        }
        println!("-----");
        loop {
            println!("{}", current.upgrade().unwrap().borrow().value);

            current = Weak::clone(&current.upgrade().unwrap().borrow().previous);

            if current.upgrade().unwrap().borrow().value == 0 {
                break;
            }
        }
    }

    fn mix(&mut self) {
        for node in self.nodes.iter() {
            if node.borrow().value == 0 {
                continue;
            }

            let next = Weak::clone(&node.borrow().next).upgrade().unwrap();
            let previous = Weak::clone(&node.borrow().previous).upgrade().unwrap();
            next.borrow_mut().previous = Rc::downgrade(&previous);
            previous.borrow_mut().next = Rc::downgrade(&next);

            let mut current = Rc::downgrade(node);
            let len = node.borrow().value;
            for _ in 0..(len % i64::try_from(self.nodes.len() - 1).unwrap()).abs() {
                if len > 0 {
                    current = Weak::clone(&current.upgrade().unwrap().borrow().next);
                } else {
                    current = Weak::clone(&current.upgrade().unwrap().borrow().previous);
                }
            }

            if len < 0 {
                current = Weak::clone(&current.upgrade().unwrap().borrow().previous);
            }
            node.borrow_mut().previous = Weak::clone(&current);
            node.borrow_mut().next = Weak::clone(&current.upgrade().unwrap().borrow().next);
            current
                .upgrade()
                .unwrap()
                .borrow()
                .next
                .upgrade()
                .unwrap()
                .borrow_mut()
                .previous = Rc::downgrade(&node);
            current.upgrade().unwrap().borrow_mut().next = Rc::downgrade(&node);
        }
    }

    fn get_total(&self) -> i64 {
        let mut current = Rc::downgrade(&self.start);
        let mut total = 0;
        for _ in 0..3 {
            for _ in 0..1000 {
                current = Weak::clone(&current.upgrade().unwrap().borrow().next);
            }
            total += current.upgrade().unwrap().borrow().value;
        }
        total
    }
}

impl super::Day for Day20 {
    fn part_1(&mut self) -> String {
        self.mix();
        self.get_total().to_string()
    }

    fn part_2(&mut self) -> String {
        for node in self.nodes.iter() {
            node.borrow_mut().value *= 811589153;
        }

        for _ in 0..10 {
            self.mix();
        }

        self.get_total().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day20::new();
        assert_eq!(day.part_1(), "3");
    }

    #[test]
    fn part_2() {
        let mut day = Day20::new();
        assert_eq!(day.part_2(), "1623178306");
    }
}
