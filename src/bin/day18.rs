use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day18.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let points = input
        .trim()
        .lines()
        .map(parse_point)
        .collect::<HashSet<_>>();
    points
        .iter()
        .copied()
        .flat_map(get_neighboring_points)
        .filter(|point| !points.contains(point))
        .count()
}

fn get_neighboring_points(point: (u32, u32, u32)) -> impl Iterator<Item = (u32, u32, u32)> {
    let (x, y, z) = point;
    std::iter::once((x - 1, y, z))
        .chain(std::iter::once((x + 1, y, z)))
        .chain(std::iter::once((x, y - 1, z)))
        .chain(std::iter::once((x, y + 1, z)))
        .chain(std::iter::once((x, y, z - 1)))
        .chain(std::iter::once((x, y, z + 1)))
}

fn parse_point(input: &str) -> (u32, u32, u32) {
    let mut iter = input
        .trim()
        .split(',')
        .map(|elem| elem.parse::<u32>().unwrap());
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
        ";
        const EXPECTED: usize = 64;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
