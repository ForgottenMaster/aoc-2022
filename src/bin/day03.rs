use itertools::*;

const INPUT: &str = include_str!("../input/day03.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    input.trim().lines().map(process_line).sum()
}

fn part_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .map(process_chunk)
        .sum()
}

fn process_line(input: &str) -> u32 {
    let input = input.trim();
    let mid_point = input.len() / 2;
    let (first_half, second_half) = input.split_at(mid_point);
    first_half
        .chars()
        .filter(|character| second_half.contains(*character))
        .map(char_to_priority)
        .next()
        .unwrap()
}

fn process_chunk<'a>(chunk: impl Iterator<Item = &'a str>) -> u32 {
    let mut chunk = chunk.map(|line| line.trim());
    let (elf_1, elf_2, elf_3) = (
        chunk.next().unwrap(),
        chunk.next().unwrap(),
        chunk.next().unwrap(),
    );
    elf_1
        .chars()
        .filter(|character| elf_2.contains(*character) && elf_3.contains(*character))
        .map(char_to_priority)
        .next()
        .unwrap()
}

fn char_to_priority(character: char) -> u32 {
    let ascii_value = character as u32;
    if character.is_lowercase() {
        ascii_value - ('a' as u32) + 1
    } else {
        ascii_value - ('A' as u32) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn test_char_to_priority_lowercase() {
        // Arrange
        const EXPECTED: u32 = 5;

        // Act
        let output = char_to_priority('e');

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_char_to_priority_uppercase() {
        // Arrange
        const EXPECTED: u32 = 31;

        // Act
        let output = char_to_priority('E');

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 157;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: u32 = 70;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
