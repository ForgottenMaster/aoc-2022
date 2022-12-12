const INPUT: &str = include_str!("../input/day12.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    let puzzle = input.parse::<Puzzle>().unwrap();
    puzzle.get_fewest_steps_from_start()
}

mod private {
    use core::convert::Infallible;
    use core::str::FromStr;
    use itertools::Either;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Default)]
    pub struct Puzzle {
        heightmap: Vec<u32>,
        width: usize,
        height: usize,
        start_coord: (usize, usize),
        end_coord: (usize, usize),
    }

    impl Puzzle {
        pub fn get_fewest_steps_from_start(&self) -> u32 {
            self.get_fewest_steps_from(self.start_coord)
        }

        pub fn get_fewest_steps_from(&self, coord: (usize, usize)) -> u32 {
            // open set is the set of discovered nodes that need to be evaluated
            // and possible expanded.
            let mut open_set = HashSet::with_capacity(self.heightmap.len());
            open_set.insert(coord);

            // mapping which, for a given node coordinate will give the coordinate
            // of the node immediately preceding it on the cheapest path from the start.
            let mut came_from: HashMap<(usize, usize), (usize, usize)> =
                HashMap::with_capacity(self.heightmap.len());

            // for a given node coordinate, g_score[coord] will be the cost of the cheapest path from start to coord
            // that is already known.
            let mut g_score = (0..self.height)
                .flat_map(|y_coord| {
                    (0..self.width).map(move |x_coord| ((x_coord, y_coord), u32::MAX))
                })
                .collect::<HashMap<_, _>>();
            g_score.insert(coord, 0);

            // for a given node coordinate, f_score represents our current best guess to how cheap a complete path from
            // start to finish, through n could be.
            let mut f_score = g_score.clone();
            f_score.insert(coord, self.estimate_remaining_distance(coord));

            // while there are more nodes to expand.
            while !open_set.is_empty() {
                // find the coordinate in the open set with the lowest f_score to expand.
                let (current, _) = open_set.iter().fold(
                    ((0, 0), u32::MAX),
                    |(lowest_coord, lowest_cost), coord| {
                        let cost = f_score[coord];
                        if cost < lowest_cost {
                            (*coord, cost)
                        } else {
                            (lowest_coord, lowest_cost)
                        }
                    },
                );

                // if the current lowest is in fact the end point, we're done.
                if current == self.end_coord {
                    return get_steps_taken(came_from, current);
                }

                // otherwise remove current from the open set to expand it.
                open_set.remove(&current);

                // for each valid neighbor of the current coordinate
                self.get_neighbor_iter(current).for_each(|neighbor| {
                    let tentative_g_score = g_score[&current] + 1; // distance from current to neighbor is always 1.
                    if tentative_g_score < g_score[&neighbor] {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        f_score.insert(
                            neighbor,
                            tentative_g_score + self.estimate_remaining_distance(neighbor),
                        );
                        open_set.insert(neighbor);
                    }
                });
            }

            // failed to find any valid path from start to end.
            u32::MAX
        }

        fn get_neighbor_iter(
            &self,
            coord: (usize, usize),
        ) -> impl Iterator<Item = (usize, usize)> + '_ {
            let (x, y) = coord;
            let left_neigbor = if x > 0 {
                Either::Left(std::iter::once((x - 1, y)))
            } else {
                Either::Right(std::iter::empty())
            };
            let top_neigbor = if y > 0 {
                Either::Left(std::iter::once((x, y - 1)))
            } else {
                Either::Right(std::iter::empty())
            };
            let right_neigbor = if x < self.width - 1 {
                Either::Left(std::iter::once((x + 1, y)))
            } else {
                Either::Right(std::iter::empty())
            };
            let bottom_neigbor = if y < self.height - 1 {
                Either::Left(std::iter::once((x, y + 1)))
            } else {
                Either::Right(std::iter::empty())
            };
            left_neigbor
                .chain(top_neigbor)
                .chain(right_neigbor)
                .chain(bottom_neigbor)
                .filter(move |neighbor| {
                    let current_height = self.heightmap[self.coord_to_index(coord)];
                    let neighbor_height = self.heightmap[self.coord_to_index(*neighbor)];
                    neighbor_height <= current_height + 1
                })
        }

        fn coord_to_index(&self, coord: (usize, usize)) -> usize {
            let (x, y) = coord;
            y * self.width + x
        }

        fn estimate_remaining_distance(&self, from_coord: (usize, usize)) -> u32 {
            let (from_x, from_y) = from_coord;
            let (to_x, to_y) = self.end_coord;
            let horizontal_distance = (from_x as i64 - to_x as i64).unsigned_abs() as u32;
            let vertical_distance = (from_y as i64 - to_y as i64).unsigned_abs() as u32;
            horizontal_distance + vertical_distance
        }
    }

    fn get_steps_taken(
        came_from: HashMap<(usize, usize), (usize, usize)>,
        mut current: (usize, usize),
    ) -> u32 {
        let mut steps_taken = 0;
        while let Some(came_from) = came_from.get(&current) {
            current = *came_from;
            steps_taken += 1;
        }
        steps_taken
    }

    impl FromStr for Puzzle {
        type Err = Infallible;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok(input.trim().lines().enumerate().fold(
                Self::default(),
                |fold_state, (y_coord, line)| {
                    line.trim().chars().enumerate().fold(
                        fold_state,
                        |mut fold_state, (x_coord, character)| {
                            let (character_height, is_start, is_end) = match character {
                                'S' => (0, true, false),
                                'E' => (25, false, true),
                                x => (x as u32 - 'a' as u32, false, false),
                            };
                            let width = x_coord + 1;
                            let height = y_coord + 1;
                            fold_state.heightmap.push(character_height);
                            fold_state.width = std::cmp::max(fold_state.width, width);
                            fold_state.height = std::cmp::max(fold_state.height, height);
                            if is_start {
                                fold_state.start_coord = (x_coord, y_coord);
                            }
                            if is_end {
                                fold_state.end_coord = (x_coord, y_coord);
                            }
                            fold_state
                        },
                    )
                },
            ))
        }
    }
}
use private::*;

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 31;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
