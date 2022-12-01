use itertools::*;

const INPUT: &str = include_str!("../input/day01.txt");

fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    input
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
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
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
        const EXPECTED: u32 = 24000;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
