use private::*;

const INPUT: &str = include_str!("../input/day17.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    Chamber::new(block_sequence_iter(), jet_sequence_iter(input)).run_simulation(2022)
}

mod private {
    pub use {blocks::block_sequence_iter, chamber::Chamber, jets::jet_sequence_iter};

    mod blocks {
        use super::jets::Direction;

        const BLOCK_SEQUENCE: [Block; 5] = [
            Block {
                rock_deltas: [Some((0, 0)), Some((1, 0)), Some((2, 0)), Some((3, 0)), None],
                width: 4,
                height: 1,
            },
            Block {
                rock_deltas: [
                    Some((1, 0)),
                    Some((0, 1)),
                    Some((1, 1)),
                    Some((2, 1)),
                    Some((1, 2)),
                ],
                width: 3,
                height: 3,
            },
            Block {
                rock_deltas: [
                    Some((0, 0)),
                    Some((1, 0)),
                    Some((2, 0)),
                    Some((2, 1)),
                    Some((2, 2)),
                ],
                width: 3,
                height: 3,
            },
            Block {
                rock_deltas: [Some((0, 0)), Some((0, 1)), Some((0, 2)), Some((0, 3)), None],
                width: 1,
                height: 3,
            },
            Block {
                rock_deltas: [Some((0, 0)), Some((1, 0)), Some((0, 1)), Some((1, 1)), None],
                width: 2,
                height: 2,
            },
        ];

        pub fn block_sequence_iter() -> impl Iterator<Item = Block> {
            BLOCK_SEQUENCE.iter().copied().cycle()
        }

        #[derive(Clone, Copy, Debug)]
        pub struct Block {
            rock_deltas: [Option<(usize, usize)>; 5], // the local x and y offsets from the bottom left coordinate of all rock spaces
            width: usize,                             // the width of the block
            height: usize,                            // the height of the block
        }

        impl Block {
            pub fn dims(&self) -> (usize, usize) {
                (self.width, self.height)
            }
        }

        #[derive(Debug)]
        pub struct BlockInstance {
            bottom_left: (usize, usize), // the bottom left coordinate in world space
            block: Block,                // a description of the block type
        }

        impl BlockInstance {
            pub fn new(bottom_left: (usize, usize), block: Block) -> Self {
                Self { bottom_left, block }
            }

            pub fn apply_push(&self, direction: Direction) -> Self {
                let new_x = match (direction, self.bottom_left.0) {
                    (Direction::Left, 0) => 0,
                    (Direction::Left, x) => x - 1,
                    (Direction::Right, x) => x + 1,
                };
                Self {
                    bottom_left: (new_x, self.bottom_left.1),
                    block: self.block,
                }
            }

            pub fn apply_drop(&self) -> Self {
                let new_y = self.bottom_left.1.saturating_sub(1);
                Self {
                    bottom_left: (self.bottom_left.0, new_y),
                    block: self.block,
                }
            }

            pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
                self.block.rock_deltas.iter().filter_map(|delta| {
                    delta.and_then(|delta| {
                        Some((self.bottom_left.0 + delta.0, self.bottom_left.1 + delta.1))
                    })
                })
            }
        }
    }

    mod chamber {
        use {
            super::blocks::{Block, BlockInstance},
            super::jets::Direction,
            std::fmt::{Debug, Formatter, Result},
        };

        #[derive(Clone, Debug, PartialEq)]
        enum Space {
            Empty,
            Rock,
        }

        #[derive(Debug)]
        enum Operation {
            Push,
            Fall,
        }

        enum TickState {
            BlockFroze,
            BlockDidNotFreeze,
        }

        pub struct Chamber<BlockIter, JetIter> {
            block_iter: BlockIter,
            jet_iter: JetIter,
            currently_falling: Option<BlockInstance>,
            next_operation: Operation,
            width: usize,
            appearance_offset: (usize, usize),
            highest_block: usize,
            state: Vec<Vec<Space>>,
        }

        impl<BlockIter, JetIter> Chamber<BlockIter, JetIter> {
            pub fn new(block_iter: BlockIter, jet_iter: JetIter) -> Self {
                Self {
                    block_iter,
                    jet_iter,
                    currently_falling: None,
                    next_operation: Operation::Push,
                    width: 7,
                    appearance_offset: (2, 3),
                    highest_block: 0,
                    state: vec![],
                }
            }
        }

        impl<BlockIter, JetIter> Chamber<BlockIter, JetIter>
        where
            BlockIter: Iterator<Item = Block>,
            JetIter: Iterator<Item = Direction>,
        {
            pub fn run_simulation(&mut self, until_after_frozen_rocks: usize) -> usize {
                let mut frozen_rocks = 0;
                while frozen_rocks < until_after_frozen_rocks {
                    if let TickState::BlockFroze = self.tick() {
                        frozen_rocks += 1;
                    }
                }
                self.highest_block
            }

            fn tick(&mut self) -> TickState {
                match &self.currently_falling {
                    None => {
                        self.spawn_next_block();
                        TickState::BlockDidNotFreeze
                    }
                    Some(_) => {
                        let (next_operation, tick_state) = match self.next_operation {
                            Operation::Fall => (Operation::Push, self.fall_current_block()),
                            Operation::Push => {
                                self.push_current_block();
                                (Operation::Fall, TickState::BlockDidNotFreeze)
                            }
                        };
                        self.next_operation = next_operation;
                        tick_state
                    }
                }
            }

            fn fall_current_block(&mut self) -> TickState {
                let mut freeze_new_block = self
                    .currently_falling
                    .as_ref()
                    .unwrap()
                    .iter_coords()
                    .any(|coord| coord.1 == 0);
                if !freeze_new_block {
                    let test_block_instance = self.currently_falling.as_ref().unwrap().apply_drop();
                    freeze_new_block = test_block_instance
                        .iter_coords()
                        .any(|coord| self.state[coord.1][coord.0] == Space::Rock);
                }

                if freeze_new_block {
                    let frozen_block = self.currently_falling.take().unwrap();
                    frozen_block.iter_coords().for_each(|coord| {
                        self.highest_block = std::cmp::max(self.highest_block, coord.1 + 1);
                        self.state[coord.1][coord.0] = Space::Rock;
                    });
                    TickState::BlockFroze
                } else {
                    self.currently_falling =
                        Some(self.currently_falling.as_ref().unwrap().apply_drop());
                    TickState::BlockDidNotFreeze
                }
            }

            fn push_current_block(&mut self) {
                let candidate_block_instance = self
                    .currently_falling
                    .as_ref()
                    .unwrap()
                    .apply_push(self.jet_iter.next().unwrap());
                if candidate_block_instance.iter_coords().all(|coord| {
                    coord.0 < self.width && self.state[coord.1][coord.0] == Space::Empty
                }) {
                    self.currently_falling = Some(candidate_block_instance);
                }
            }

            fn spawn_next_block(&mut self) {
                let spawn_position = (
                    self.appearance_offset.0,
                    self.highest_block + self.appearance_offset.1,
                );
                let block = self.block_iter.next().unwrap();
                let (_, block_height) = block.dims();
                let chamber_height = spawn_position.1 + block_height + 1;
                self.ensure_chamber_height(chamber_height);
                self.next_operation = Operation::Push;
                self.currently_falling = Some(BlockInstance::new(spawn_position, block));
            }

            fn ensure_chamber_height(&mut self, height: usize) {
                let current_height = self.state.len();
                if height > current_height {
                    self.state.extend(
                        std::iter::repeat(
                            std::iter::repeat(Space::Empty)
                                .take(self.width)
                                .collect::<Vec<_>>(),
                        )
                        .take(height - current_height),
                    );
                }
            }
        }

        impl<BlockIter, JetIter> Debug for Chamber<BlockIter, JetIter>
        where
            BlockIter: Iterator,
            JetIter: Iterator,
            <BlockIter as Iterator>::Item: Debug,
            <JetIter as Iterator>::Item: Debug,
        {
            fn fmt(&self, f: &mut Formatter) -> Result {
                f.debug_struct("Chamber")
                    .field("block_iter", &DebugOpaqueIterator("Block"))
                    .field("jet_iter", &DebugOpaqueIterator("Direction"))
                    .field("currently_falling", &self.currently_falling)
                    .field("next_operation", &self.next_operation)
                    .field("width", &self.width)
                    .field("appearance_offset", &self.appearance_offset)
                    .field("highest_block", &self.highest_block)
                    .field("state", &self.state)
                    .finish()
            }
        }

        struct DebugOpaqueIterator<'a>(&'a str);

        impl Debug for DebugOpaqueIterator<'_> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "impl Iterator<Item = {}>", self.0)
            }
        }
    }

    mod jets {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Direction {
            Left,
            Right,
        }

        pub fn jet_sequence_iter(input: &str) -> impl Iterator<Item = Direction> {
            input
                .trim()
                .chars()
                .map(|c| match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => unimplemented!(
                        "Only '>' and '<' are allowed characters in the input string."
                    ),
                })
                .collect::<Vec<_>>()
                .into_iter()
                .cycle()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        const EXPECTED: usize = 3068;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
