use forest::*;

const INPUT: &str = include_str!("../input/day08.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

mod forest {
    use core::convert::Infallible;
    use core::str::FromStr;
    use itertools::Either;

    #[derive(Debug)]
    pub struct Forest {
        width: usize,
        height: usize,
        data: Box<[u8]>,
    }

    impl Forest {
        pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
            (0..self.data.len()).map(|index| {
                let elem = self.data[index];
                let coord = self.index_to_coord(index);
                (coord, elem)
            })
        }

        pub fn iter_north(
            &self,
            from: (usize, usize),
        ) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
            let (x, y) = from;
            if y == 0 {
                Either::Left(std::iter::empty())
            } else {
                Either::Right((0..=y - 1).rev().map(move |y| {
                    let coord = (x, y);
                    let index = self.coord_to_index(coord);
                    let elem = self.data[index];
                    (coord, elem)
                }))
            }
        }

        pub fn iter_south(
            &self,
            from: (usize, usize),
        ) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
            let (x, y) = from;
            if y == self.height - 1 {
                Either::Left(std::iter::empty())
            } else {
                Either::Right((y + 1..=self.height - 1).map(move |y| {
                    let coord = (x, y);
                    let index = self.coord_to_index(coord);
                    let elem = self.data[index];
                    (coord, elem)
                }))
            }
        }

        pub fn iter_east(
            &self,
            from: (usize, usize),
        ) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
            let (x, y) = from;
            if x == self.width - 1 {
                Either::Left(std::iter::empty())
            } else {
                Either::Right((x + 1..=self.width - 1).map(move |x| {
                    let coord = (x, y);
                    let index = self.coord_to_index(coord);
                    let elem = self.data[index];
                    (coord, elem)
                }))
            }
        }

        pub fn iter_west(
            &self,
            from: (usize, usize),
        ) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
            let (x, y) = from;
            if x == 0 {
                Either::Left(std::iter::empty())
            } else {
                Either::Right((0..=x - 1).rev().map(move |x| {
                    let coord = (x, y);
                    let index = self.coord_to_index(coord);
                    let elem = self.data[index];
                    (coord, elem)
                }))
            }
        }

        fn index_to_coord(&self, index: usize) -> (usize, usize) {
            let y = index / self.width;
            let x = index % self.width;
            (x, y)
        }

        fn coord_to_index(&self, coord: (usize, usize)) -> usize {
            let (x, y) = coord;
            y * self.width + x
        }
    }

    impl FromStr for Forest {
        type Err = Infallible;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            // parse the lines, get the width of the forest from a line
            // along with the flattened data.
            let (width, data) =
                input
                    .trim()
                    .lines()
                    .fold((0, Vec::<u8>::new()), |(_, mut data), line| {
                        let line_length = line.trim().chars().fold(0, |line_length, c| {
                            data.push(c.to_digit(10).unwrap() as u8);
                            line_length + 1
                        });
                        (line_length, data)
                    });
            let data = data.into_boxed_slice();

            // height can be found from length of data divided by width.
            let height = data.len() / width;

            // create the forest.
            Ok(Self {
                width,
                height,
                data,
            })
        }
    }
}

fn part_1(input: &str) -> u32 {
    let forest = input.parse::<Forest>().unwrap();
    forest
        .iter()
        .filter(|(coord, elem_1)| is_tree_visible(&forest, *coord, |(_, elem_2)| elem_2 >= *elem_1))
        .count() as u32
}

fn is_tree_visible(
    forest: &Forest,
    coord: (usize, usize),
    mut func: impl FnMut(((usize, usize), u8)) -> bool,
) -> bool {
    !forest.iter_north(coord).any(&mut func)
        || !forest.iter_east(coord).any(&mut func)
        || !forest.iter_south(coord).any(&mut func)
        || !forest.iter_west(coord).any(&mut func)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    30373
    25512
    65332
    33549
    35390
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 21;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
