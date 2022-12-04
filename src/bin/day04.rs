const INPUT: &str = include_str!("../input/day04.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(line_to_ranges)
        .filter(does_one_fully_contain_other)
        .count() as u32
}

fn line_to_ranges(input: &str) -> ((u32, u32), (u32, u32)) {
    let mut iter = input.trim().split(',').map(text_to_range);
    (iter.next().unwrap(), iter.next().unwrap())
}

fn text_to_range(input: &str) -> (u32, u32) {
    let mut iter = input
        .trim()
        .split('-')
        .map(|num| num.parse::<u32>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn does_one_fully_contain_other(ranges: &((u32, u32), (u32, u32))) -> bool {
    let (a, b) = ranges;
    does_first_fully_contain_second(*a, *b) || does_first_fully_contain_second(*b, *a)
}

fn does_first_fully_contain_second(first: (u32, u32), second: (u32, u32)) -> bool {
    first.0 <= second.0 && first.1 >= second.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 2;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
