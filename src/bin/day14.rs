use core::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

const INPUT: &str = include_str!("../input/day14.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    count_grains::<false>(input)
}

fn part_2(input: &str) -> u32 {
    count_grains::<true>(input)
}

fn count_grains<const ADD_FLOOR: bool>(input: &str) -> u32 {
    let mut cave = input.trim().parse::<Cave<ADD_FLOOR>>().unwrap();
    let mut counter = 0;
    loop {
        match cave.drop_one_grain() {
            StepResult::Abort => break,
            StepResult::Rest => counter += 1,
            _ => unreachable!(),
        }
    }
    counter
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
enum Element {
    Rock,
    Air,
    Spawner,
    Sand,
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum StepResult {
    Continue,
    Abort,
    Rest,
}

//////////////////////////////////////////////////////////////////////////////////////////////

struct Cave<const ADD_FLOOR: bool = false> {
    data: Box<[Element]>,
    stride: usize,
    spawner_index: usize,
    current_grain_index: Option<usize>,
}

impl<const ADD_FLOOR: bool> Cave<ADD_FLOOR> {
    fn step(&mut self) -> StepResult {
        if let Some(old_grain_index) = self.current_grain_index {
            self.current_grain_index = None;
            self.data[old_grain_index] = if old_grain_index == self.spawner_index {
                Element::Spawner
            } else {
                Element::Air
            };
            let old_grain_x = old_grain_index % self.stride;
            let new_grain_index = old_grain_index + self.stride;

            // abort if straight down takes us out of the simulation zone.
            if new_grain_index >= self.data.len() {
                return StepResult::Abort;
            }

            // if straight down is free then move there.
            if self.data[new_grain_index] == Element::Air {
                self.current_grain_index = Some(new_grain_index);
                self.data[new_grain_index] = Element::Sand;
                return StepResult::Continue;
            }

            // if the previous grain x is 0 then we have to abort.
            if old_grain_x == 0 {
                return StepResult::Abort;
            }
            let new_grain_index = new_grain_index - 1;

            // if the space to the left is free then move there.
            if self.data[new_grain_index] == Element::Air {
                self.current_grain_index = Some(new_grain_index);
                self.data[new_grain_index] = Element::Sand;
                return StepResult::Continue;
            }

            // if the previous grain x is stride-1 then we have to abort.
            if old_grain_x == self.stride - 1 {
                return StepResult::Abort;
            }
            let new_grain_index = new_grain_index + 2;

            // if the space to the right is free then move there.
            if self.data[new_grain_index] == Element::Air {
                self.current_grain_index = Some(new_grain_index);
                self.data[new_grain_index] = Element::Sand;
                StepResult::Continue
            } else {
                self.data[old_grain_index] = Element::Sand;
                StepResult::Rest
            }
        } else if self.data[self.spawner_index] == Element::Spawner {
            self.current_grain_index = Some(self.spawner_index);
            self.data[self.spawner_index] = Element::Sand;
            StepResult::Continue
        } else {
            StepResult::Abort
        }
    }

    fn drop_one_grain(&mut self) -> StepResult {
        loop {
            match self.step() {
                r @ (StepResult::Rest | StepResult::Abort) => break r,
                _ => continue,
            }
        }
    }
}

impl<const ADD_FLOOR: bool> Display for Cave<ADD_FLOOR> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for i in 0..self.data.len() {
            if i != 0 && i % self.stride == 0 {
                writeln!(f)?;
            }
            write!(
                f,
                "{}",
                match self.data[i] {
                    Element::Rock => '#',
                    Element::Air => '.',
                    Element::Spawner => '+',
                    Element::Sand => 'o',
                }
            )?;
        }
        Ok(())
    }
}

impl<const ADD_FLOOR: bool> FromStr for Cave<ADD_FLOOR> {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut descriptors = input.parse::<RockDescriptors>().unwrap();
        let spawner_position = (500, 0);
        let mut bounds = descriptors.bounds().encapsulate(spawner_position);
        let mut min = bounds.min;
        if ADD_FLOOR {
            let y = bounds.max.1 + 2;
            let min_x = min.0 - 1000;
            let max_x = bounds.max.0 + 1000;
            let start = (min_x, y);
            let end = (max_x, y);
            descriptors.add_descriptor(start, end);
            bounds = bounds.encapsulate(start).encapsulate(end);
            min = bounds.min;
        }
        let spawner_position = (
            (spawner_position.0 - min.0) as usize,
            (spawner_position.1 - min.1) as usize,
        );
        let dims = bounds.dims();
        let total = dims.0 * dims.1;
        let mut data = std::iter::repeat(Element::Air)
            .take(total)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let stride = dims.0;
        let spawner_index = spawner_position.1 * stride + spawner_position.0;
        data[spawner_index] = Element::Spawner;
        descriptors
            .iter_segments()
            .for_each(|((start_x, start_y), (end_x, end_y))| {
                let (min_x, min_y) = (std::cmp::min(start_x, end_x), std::cmp::min(start_y, end_y));
                let (max_x, max_y) = (std::cmp::max(start_x, end_x), std::cmp::max(start_y, end_y));
                (min_x..=max_x).for_each(|x| {
                    (min_y..=max_y).for_each(|y| {
                        let (x, y) = ((x - min.0) as usize, (y - min.1) as usize);
                        let index = y * stride + x;
                        data[index] = Element::Rock;
                    });
                });
            });
        let current_grain_index = None;
        Ok(Self {
            data,
            stride,
            spawner_index,
            current_grain_index,
        })
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Coordinate((isize, isize));

impl FromStr for Coordinate {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input
            .trim()
            .split(',')
            .map(|elem| elem.trim().parse().unwrap());
        Ok(Self((iter.next().unwrap(), iter.next().unwrap())))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct RockDescriptor(Box<[Coordinate]>);

impl RockDescriptor {
    fn iter_segments(&self) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
        self.0.windows(2).map(|slice| (slice[0].0, slice[1].0))
    }
}

impl FromStr for RockDescriptor {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .trim()
                .split("->")
                .map(|elem| elem.trim().parse().unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct RockDescriptors(Vec<RockDescriptor>);

impl RockDescriptors {
    fn bounds(&self) -> Bounds {
        self.0
            .iter()
            .flat_map(|descriptor| descriptor.0.iter())
            .fold(Bounds::default(), |bounds, coordinate| {
                bounds.encapsulate(coordinate.0)
            })
    }

    fn iter_segments(&self) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
        self.0
            .iter()
            .flat_map(|descriptor| descriptor.iter_segments())
    }

    fn add_descriptor(&mut self, start: (isize, isize), end: (isize, isize)) {
        self.0.push(RockDescriptor(
            vec![Coordinate(start), Coordinate(end)].into_boxed_slice(),
        ));
    }
}

impl FromStr for RockDescriptors {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .trim()
                .lines()
                .map(|line| line.trim().parse().unwrap())
                .collect(),
        ))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Bounds {
    min: (isize, isize),
    max: (isize, isize),
}

impl Bounds {
    fn dims(&self) -> (usize, usize) {
        let ((min_x, min_y), (max_x, max_y)) = (self.min, self.max);
        ((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1)
    }

    fn encapsulate(&self, additional: (isize, isize)) -> Self {
        let ((min_x, min_y), (max_x, max_y), (x, y)) = (self.min, self.max, additional);
        let min = (std::cmp::min(min_x, x), std::cmp::min(min_y, y));
        let max = (std::cmp::max(max_x, x), std::cmp::max(max_y, y));
        Self { min, max }
    }
}

impl Default for Bounds {
    fn default() -> Self {
        let min = (isize::MAX, isize::MAX);
        let max = (isize::MIN, isize::MIN);
        Self { min, max }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        // Arrange
        const INPUT: &str = "498,12";
        const EXPECTED: Coordinate = Coordinate((498, 12));

        // Act
        let output = INPUT.parse::<Coordinate>().unwrap();

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_rock_descriptor() {
        // Arrange
        const INPUT: &str = "498,4 -> 498,6 -> 496,6";
        let expected = RockDescriptor(
            vec![
                Coordinate((498, 4)),
                Coordinate((498, 6)),
                Coordinate((496, 6)),
            ]
            .into_boxed_slice(),
        );

        // Act
        let output = INPUT.parse::<RockDescriptor>().unwrap();

        // Assert
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_rock_descriptors() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let expected = RockDescriptors(vec![
            RockDescriptor(
                vec![
                    Coordinate((498, 4)),
                    Coordinate((498, 6)),
                    Coordinate((496, 6)),
                ]
                .into_boxed_slice(),
            ),
            RockDescriptor(
                vec![
                    Coordinate((503, 4)),
                    Coordinate((502, 4)),
                    Coordinate((502, 9)),
                    Coordinate((494, 9)),
                ]
                .into_boxed_slice(),
            ),
        ]);

        // Act
        let output = INPUT.parse::<RockDescriptors>().unwrap();

        // Assert
        assert_eq!(output, expected);
    }

    #[test]
    fn test_bounds() {
        // Arrange
        const INPUT: &str = "
                    498,4 -> 498,6 -> 496,6
                    503,4 -> 502,4 -> 502,9 -> 494,9";
        const EXPECTED: Bounds = Bounds {
            min: (494, 4),
            max: (503, 9),
        };

        // Act
        let output = INPUT.parse::<RockDescriptors>().unwrap().bounds();

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_dims() {
        // Arrange
        const INPUT: Bounds = Bounds {
            min: (494, 4),
            max: (503, 9),
        };
        const EXPECTED: (usize, usize) = (10, 6);

        // Act
        let output = INPUT.dims();

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_bounds_encapsulate() {
        // Arrange
        const INPUT: Bounds = Bounds {
            min: (494, 4),
            max: (503, 9),
        };
        const EXPECTED: Bounds = Bounds {
            min: (494, 0),
            max: (503, 9),
        };

        // Act
        let output = INPUT.encapsulate((500, 0));

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_parse_cave() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            ";
        const EXPECTED_DISPLAY_STRING: &str = "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.";
        const EXPECTED_STRIDE: usize = 10;

        // Act
        let output = INPUT.parse::<Cave>().unwrap();

        // Assert
        assert_eq!(format!("{}", output), EXPECTED_DISPLAY_STRING);
        assert_eq!(output.stride, EXPECTED_STRIDE);
    }

    #[test]
    fn test_drop_one_grain() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            ";
        const EXPECTED: &str = "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.";
        let mut cave = INPUT.parse::<Cave>().unwrap();

        // Act
        cave.drop_one_grain();
        let output = format!("{}", cave);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_drop_twenty_two_grains() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            ";
        const EXPECTED: &str = "......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########.";
        let mut cave = INPUT.parse::<Cave>().unwrap();

        // Act
        (0..22).for_each(|_| {
            cave.drop_one_grain();
        });
        let output = format!("{}", cave);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            ";
        const EXPECTED: u32 = 24;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            ";
        const EXPECTED: u32 = 93;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
