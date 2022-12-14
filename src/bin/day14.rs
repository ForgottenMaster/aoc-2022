use core::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[cfg(not(tarpaulin))]
fn main() {}

//////////////////////////////////////////////////////////////////////////////////////////////

enum Element {
    Rock,
    Air,
    Spawner,
}

//////////////////////////////////////////////////////////////////////////////////////////////

struct Cave {
    data: Box<[Element]>,
    dims: (usize, usize),
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for Cave {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Coordinate((usize, usize));

impl FromStr for Coordinate {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input
            .trim()
            .split(",")
            .map(|elem| elem.trim().parse().unwrap());
        Ok(Self((iter.next().unwrap(), iter.next().unwrap())))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct RockDescriptor(Box<[Coordinate]>);

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
struct RockDescriptors(Box<[RockDescriptor]>);

impl RockDescriptors {
    fn bounds(&self) -> Bounds {
        self.0
            .iter()
            .flat_map(|descriptor| descriptor.0.iter())
            .fold(Bounds::default(), |bounds, coordinate| {
                bounds.encapsulate(coordinate.0)
            })
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
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Bounds {
    min: (usize, usize),
    max: (usize, usize),
}

impl Bounds {
    fn dims(&self) -> (usize, usize) {
        let ((min_x, min_y), (max_x, max_y)) = (self.min, self.max);
        (max_x - min_x + 1, max_y - min_y + 1)
    }

    fn encapsulate(&self, additional: (usize, usize)) -> Self {
        let ((min_x, min_y), (max_x, max_y), (x, y)) = (self.min, self.max, additional);
        let min = (std::cmp::min(min_x, x), std::cmp::min(min_y, y));
        let max = (std::cmp::max(max_x, x), std::cmp::max(max_y, y));
        Self { min, max }
    }
}

impl Default for Bounds {
    fn default() -> Self {
        let min = (usize::MAX, usize::MAX);
        let max = (usize::MIN, usize::MIN);
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
        let expected = RockDescriptors(
            vec![
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
            ]
            .into_boxed_slice(),
        );

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
        const EXPECTED_DIMS: (usize, usize) = (10, 10);

        // Act
        let output = INPUT.parse::<Cave>().unwrap();

        // Assert
        assert_eq!(format!("{}", output), EXPECTED_DISPLAY_STRING);
        assert_eq!(output.dims, EXPECTED_DIMS);
    }
}
