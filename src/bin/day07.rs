use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

const INPUT: &str = include_str!("../input/day07.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

type EntryPointer = Rc<RefCell<Entry>>;
type EntryWeakPointer = Weak<RefCell<Entry>>;

#[derive(Debug)]
struct Entry {
    parent: Option<EntryWeakPointer>,
    entry_type: EntryType,
}

#[derive(Debug)]
enum EntryType {
    Directory {
        children: HashMap<String, EntryPointer>,
    },
    File {
        size: u32,
    },
}

impl Entry {
    fn for_each_directory(&self, func: impl Fn(&Self) + Copy) {
        if let EntryType::Directory { children } = &self.entry_type {
            func(self);
            children
                .values()
                .for_each(|child| child.borrow().for_each_directory(func));
        }
    }

    fn size(&self) -> u32 {
        match &self.entry_type {
            EntryType::Directory { children } => {
                children.values().map(|child| child.borrow().size()).sum()
            }
            EntryType::File { size } => *size,
        }
    }
}

fn part_1(input: &str) -> u32 {
    let dir_structure = calculate_dir_structure(input);
    let total_size = Cell::new(0);
    dir_structure.borrow().for_each_directory(|entry| {
        let size = entry.size();
        if size <= 100_000 {
            total_size.replace(total_size.get() + size);
        }
    });
    total_size.get()
}

fn part_2(input: &str) -> u32 {
    let dir_structure = calculate_dir_structure(input);
    let total_used = dir_structure.borrow().size();
    let total_unused = 70_000_000 - total_used;
    let required_to_free = 30_000_000 - total_unused;
    let minimal_size = Cell::new(u32::MAX);
    dir_structure.borrow().for_each_directory(|entry| {
        let size = entry.size();
        if size >= required_to_free {
            minimal_size.replace(std::cmp::min(minimal_size.get(), size));
        }
    });
    minimal_size.get()
}

fn calculate_dir_structure(input: &str) -> EntryPointer {
    // hold onto the root directory for navigation straight back to root and for returning later.
    let root = create_new_directory(None);

    // starting off at the root as current.
    let mut current = Rc::clone(&root);

    // process the commands line by line.
    input.trim().lines().for_each(|line| match line.trim() {
        "$ cd /" => current = Rc::clone(&root),
        "$ ls" => {} // ls results in the output in the following lines, so nothing to do
        "$ cd .." => {
            let new_current = current.borrow().parent.as_ref().unwrap().upgrade().unwrap();
            current = new_current;
        }
        line => {
            // all of the remaining line types will assume that the current
            // entry is a Directory as it'll need access to the children
            let replacement = match &mut current.borrow_mut().entry_type {
                EntryType::Directory { children } => {
                    if let Some(line) = line.strip_prefix("dir ") {
                        children.insert(
                            line.to_string(),
                            create_new_directory(Some(Rc::downgrade(&current))),
                        );
                        None
                    } else if let Some(line) = line.strip_prefix("$ cd ") {
                        Some(Rc::clone(&children[line]))
                    } else {
                        let mut file_parts = line.split_whitespace();
                        let size = file_parts.next().unwrap().parse::<u32>().unwrap();
                        let name = file_parts.next().unwrap();
                        children.insert(
                            name.to_string(),
                            create_new_file(Some(Rc::downgrade(&current)), size),
                        );
                        None
                    }
                }
                _ => unimplemented!(),
            };
            if let Some(replacement) = replacement {
                current = replacement;
            }
        }
    });

    // return root.
    root
}

fn create_new_directory(parent: Option<EntryWeakPointer>) -> EntryPointer {
    Rc::new(RefCell::new(Entry {
        parent,
        entry_type: EntryType::Directory {
            children: HashMap::new(),
        },
    }))
}

fn create_new_file(parent: Option<EntryWeakPointer>, size: u32) -> EntryPointer {
    Rc::new(RefCell::new(Entry {
        parent,
        entry_type: EntryType::File { size },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 95437;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: u32 = 24933642;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
