use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct File {
    parent: Weak<RefCell<File>>,
    children: Vec<Rc<RefCell<File>>>,
    name: String,
    size: u32,
    is_directory: bool,
}

impl File {
    fn find_or_create_child(
        strong_self: &Rc<RefCell<File>>,
        name: String,
        is_directory: bool,
    ) -> Rc<RefCell<File>> {
        for child in &strong_self.borrow().children {
            if child.borrow().name == name {
                return Rc::clone(child);
            }
        }
        let new_file = Rc::new(RefCell::new(File {
            parent: Rc::downgrade(&strong_self),
            children: vec![],
            name,
            size: 0,
            is_directory,
        }));
        strong_self.borrow_mut().children.push(Rc::clone(&new_file));
        return new_file;
    }

    fn add_size(&mut self, size: u32) {
        self.size += size;
        let parent = self.parent.upgrade();
        if parent.is_none() {
            return;
        }
        parent.unwrap().borrow_mut().add_size(size);
    }

    fn get_small_dirs_size(&self) -> u32 {
        let sub = self.children.iter().fold(0, |acc, val| {
            let file = val.borrow();
            if !file.is_directory {
                return acc;
            }

            return acc + file.get_small_dirs_size();
        });

        if self.size < 100_000 {
            return self.size + sub;
        } else {
            return sub;
        }
    }

    fn get_smallest_dir_bigger_than(&self, target: u32) -> u32 {
        let min_contained = self
            .children
            .iter()
            .map(|val| {
                let file = val.borrow();
                if !file.is_directory {
                    return u32::MAX;
                }

                return file.get_smallest_dir_bigger_than(target);
            })
            .min()
            .unwrap_or(u32::MAX);

        if self.size >= target && self.size < min_contained {
            return self.size;
        } else {
            return min_contained;
        }
    }
}

pub(crate) struct Day07 {
    root: Rc<RefCell<File>>,
}

impl Day07 {
    pub(crate) fn new() -> Self {
        let lines = crate::utils::read_day_lines(7);

        let root = Rc::new(RefCell::new(File {
            parent: Weak::new(),
            children: vec![],
            name: "/".to_string(),
            size: 0,
            is_directory: true,
        }));
        let curr = &mut Rc::clone(&root);

        for line in &lines {
            if line.starts_with('$') {
                let parts: Vec<&str> = line.split_ascii_whitespace().collect();
                if parts[1] == "cd" {
                    if parts[2] == ".." {
                        let tmp = Rc::clone(&(curr.borrow().parent.upgrade().unwrap()));
                        *curr = tmp;
                    } else if parts[2] != "/" {
                        *curr = File::find_or_create_child(curr, parts[2].to_string(), true);
                    }
                }
            } else {
                let parts: Vec<&str> = line.split_ascii_whitespace().collect();
                let is_directory = parts[0] == "dir";
                let child = File::find_or_create_child(curr, parts[1].to_string(), is_directory);
                if !is_directory {
                    child
                        .borrow_mut()
                        .add_size(parts[0].parse::<u32>().unwrap());
                }
            }
        }

        return Self { root };
    }
}

impl super::Day for Day07 {
    fn part_1(&mut self) -> String {
        return self.root.borrow().get_small_dirs_size().to_string();
    }

    fn part_2(&mut self) -> String {
        let to_free = 30_000_000 - (70_000_000 - self.root.borrow().size);
        return self
            .root
            .borrow()
            .get_smallest_dir_bigger_than(to_free)
            .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day;

    #[test]
    fn part_1() {
        let mut day = Day07::new();
        assert_eq!(day.part_1(), "95437");
    }

    #[test]
    fn part_2() {
        let mut day = Day07::new();
        assert_eq!(day.part_2(), "24933642");
    }
}
