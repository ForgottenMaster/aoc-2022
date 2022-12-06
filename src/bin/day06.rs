const INPUT: &str = include_str!("../input/day06.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    process_input(input, 4)
}

fn part_2(input: &str) -> usize {
    process_input(input, 14)
}

fn process_input(input: &str, window_size: usize) -> usize {
    let bytes = input.trim().as_bytes();
    bytes
        .windows(window_size)
        .enumerate()
        .filter_map(|(index, window)| {
            if bytes_in_slice_are_unique(window) {
                Some(index + window_size)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn bytes_in_slice_are_unique(slice: &[u8]) -> bool {
    (0..slice.len() - 1).all(|first_index| {
        let first_byte = slice[first_index];
        !(first_index + 1..slice.len()).any(|second_index| {
            let second_byte = slice[second_index];
            first_byte == second_byte
        })
    })
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

    #[test]
    fn test_part_2_1() {
        // Arrange
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        const EXPECTED: usize = 19;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2_2() {
        // Arrange
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        const EXPECTED: usize = 23;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_3_3() {
        // Arrange
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        const EXPECTED: usize = 23;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
