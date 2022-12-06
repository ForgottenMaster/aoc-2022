use itertools::*;

const INPUT: &str = include_str!("../input/day06.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .chars()
        .tuple_windows()
        .enumerate()
        .filter_map(|(index, tuple)| {
            if chars_in_tuple_are_unique(tuple) {
                Some(index + 4)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn chars_in_tuple_are_unique(tuple: (char, char, char, char)) -> bool {
    let (first, second, third, fourth) = tuple;
    first != second
        && first != third
        && first != fourth
        && second != third
        && second != fourth
        && third != fourth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        // Arrange
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        const EXPECTED: usize = 7;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_1_2() {
        // Arrange
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        const EXPECTED: usize = 5;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_1_3() {
        // Arrange
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        const EXPECTED: usize = 6;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
