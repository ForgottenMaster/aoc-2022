use itertools::*;

const INPUT: &str = include_str!("../input/day01.txt");

macro_rules! calorie_count_iter {
    ($input:expr) => {
        $input
            .trim()
            .lines()
            .group_by(|line| !line.trim().is_empty())
            .into_iter()
            .filter(|(key, _)| *key)
            .map(|(_, group)| {
                group
                    .map(|line| line.trim().parse::<u32>().unwrap())
                    .sum::<u32>()
            })
    };
}

fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    calorie_count_iter!(input).max().unwrap()
}

fn part_2(input: &str) -> u32 {
    let mut counts = calorie_count_iter!(input).collect::<Vec<_>>();
    counts.sort();
    counts.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    1000
    2000
    3000
  
    4000
  
    5000
    6000
  
    7000
    8000
    9000
  
    10000
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 24000;

        // Act
        let output = part_1(TEST_INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: u32 = 45000;

        // Act
        let output = part_2(TEST_INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
