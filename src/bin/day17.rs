use private::*;

const INPUT: &str = include_str!("../input/day17.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    Chamber::new(block_sequence_iter(), jet_sequence_iter(input)).run_until_n_blocks_frozen(2022)
}

fn part_2(input: &str) -> u64 {
    let mut chamber = Chamber::new(block_sequence_iter(), jet_sequence_iter(input));
    let (number_of_frozen_rocks_at_cycle_detection, cycle_key, tower_height_at_cycle_detection) =
        chamber.run_until_cycle_detected();
    let (number_of_frozen_rocks_between_cycles, tower_height_at_cycle_repetition) =
        chamber.run_until_cycle_key_repeated(cycle_key);
    let tower_height_delta_between_cycles =
        tower_height_at_cycle_repetition - tower_height_at_cycle_detection;

    // calculate results of complete cycles.
    let total_frozen_rocks_so_far =
        number_of_frozen_rocks_at_cycle_detection + number_of_frozen_rocks_between_cycles;
    let remaining_rocks_to_freeze = 1000000000000 - total_frozen_rocks_so_far;
    let cycles_remaining = remaining_rocks_to_freeze / number_of_frozen_rocks_between_cycles;
    let total_frozen_rocks_so_far =
        total_frozen_rocks_so_far + cycles_remaining * number_of_frozen_rocks_between_cycles;
    let tower_height_after_simulation = tower_height_at_cycle_repetition as u64
        + cycles_remaining * tower_height_delta_between_cycles as u64;

    // run standard simulation for outstanding rocks.
    let remaining_rocks_to_freeze = 1000000000000 - total_frozen_rocks_so_far;
    let tower_height_after_remaining =
        chamber.run_until_n_blocks_frozen(remaining_rocks_to_freeze as usize);
    let delta = tower_height_after_remaining - tower_height_at_cycle_repetition;

    // total height is then the height after mathematical simulation plus the delta.
    tower_height_after_simulation + delta as u64
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

        pub fn block_sequence_iter() -> impl Iterator<Item = (usize, Block)> {
            BLOCK_SEQUENCE.iter().copied().enumerate().cycle()
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
            std::{
                collections::HashSet,
                fmt::{Debug, Formatter, Result},
            },
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
            frozen_column_height_offsets: Vec<usize>, // the height difference for each column between the top bit of rock and the highest block
            last_pulled_block_index: usize,
            last_pulled_jet_direction_index: usize,
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
                    frozen_column_height_offsets: vec![],
                    last_pulled_block_index: 0,
                    last_pulled_jet_direction_index: 0,
                }
            }
        }

        impl<BlockIter, JetIter> Chamber<BlockIter, JetIter>
        where
            BlockIter: Iterator<Item = (usize, Block)>,
            JetIter: Iterator<Item = (usize, Direction)>,
        {
            pub fn run_until_cycle_detected(&mut self) -> (u64, (Vec<usize>, usize, usize), usize) {
                let mut key_set = HashSet::new();
                let mut frozen_rocks = 0;
                loop {
                    if let TickState::BlockFroze = self.tick() {
                        frozen_rocks += 1;
                        let cycle_key = (
                            self.frozen_column_height_offsets.clone(),
                            self.last_pulled_block_index,
                            self.last_pulled_jet_direction_index,
                        );
                        if !key_set.insert(cycle_key.clone()) {
                            break (frozen_rocks, cycle_key, self.highest_block);
                        }
                    }
                }
            }

            pub fn run_until_cycle_key_repeated(
                &mut self,
                cycle_key: (Vec<usize>, usize, usize),
            ) -> (u64, usize) {
                let mut key_set = HashSet::new();
                key_set.insert(cycle_key);
                let mut frozen_rocks = 0;
                loop {
                    if let TickState::BlockFroze = self.tick() {
                        frozen_rocks += 1;
                        let cycle_key = (
                            self.frozen_column_height_offsets.clone(),
                            self.last_pulled_block_index,
                            self.last_pulled_jet_direction_index,
                        );
                        if !key_set.insert(cycle_key.clone()) {
                            break (frozen_rocks, self.highest_block);
                        }
                    }
                }
            }

            pub fn run_until_n_blocks_frozen(&mut self, until_after_frozen_rocks: usize) -> usize {
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
                    self.frozen_column_height_offsets = (0..self.width)
                        .map(|x| {
                            self.state
                                .iter()
                                .enumerate()
                                .rev()
                                .filter_map(|(index, line)| match line[x] {
                                    Space::Empty => None,
                                    Space::Rock => Some(self.highest_block - (index + 1)),
                                })
                                .next()
                                .unwrap_or_default()
                        })
                        .collect();
                    TickState::BlockFroze
                } else {
                    self.currently_falling =
                        Some(self.currently_falling.as_ref().unwrap().apply_drop());
                    TickState::BlockDidNotFreeze
                }
            }

            fn push_current_block(&mut self) {
                let (jet_index, push_direction) = self.jet_iter.next().unwrap();
                self.last_pulled_jet_direction_index = jet_index;
                let candidate_block_instance = self
                    .currently_falling
                    .as_ref()
                    .unwrap()
                    .apply_push(push_direction);
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
                let (block_index, block) = self.block_iter.next().unwrap();
                self.last_pulled_block_index = block_index;
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
                    .field("last_pulled_block_index", &self.last_pulled_block_index)
                    .field(
                        "last_pulled_jet_direction_index",
                        &self.last_pulled_jet_direction_index,
                    )
                    .field(
                        "frozen_column_height_offsets",
                        &self.frozen_column_height_offsets,
                    )
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
        #[derive(Clone, Copy, Debug)]
        pub enum Direction {
            Left,
            Right,
        }

        pub fn jet_sequence_iter(input: &str) -> impl Iterator<Item = (usize, Direction)> {
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
                .enumerate()
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

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        const EXPECTED: u64 = 1514285714288;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
