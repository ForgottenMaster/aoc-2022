use std::collections::HashSet;

type Coordinate = (i32, i32);

const INPUT: &str = include_str!("../input/day09.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    calculate_with_points(input, 2)
}

fn part_2(input: &str) -> usize {
    calculate_with_points(input, 10)
}

fn calculate_with_points(input: &str, knots: usize) -> usize {
    let (_, visited) = input.trim().lines().fold(
        (
            std::iter::repeat((0, 0)).take(knots).collect::<Vec<_>>(),
            HashSet::new(),
        ),
        |(mut positions, mut visited), line| {
            let mut splits = line.split_whitespace();
            let direction = splits.next().unwrap();
            let amount = splits.next().unwrap().parse::<u32>().unwrap();
            (0..amount).for_each(|_| {
                let (hx, hy) = positions[0];
                let head = match direction {
                    "R" => (hx + 1, hy),
                    "L" => (hx - 1, hy),
                    "U" => (hx, hy - 1),
                    "D" => (hx, hy + 1),
                    _ => unimplemented!(),
                };
                positions[0] = head;
                (1..positions.len()).for_each(|index| {
                    positions[index] = move_tail(positions[index - 1], positions[index]);
                });
                visited.insert(positions[positions.len() - 1]);
            });
            (positions, visited)
        },
    );
    visited.len()
}

fn move_tail(head: Coordinate, tail: Coordinate) -> Coordinate {
    // get the distances on the x and y directions.
    let ((hx, hy), (tx, ty)) = (head, tail);
    let x = hx - tx;
    let y = hy - ty;

    // if the distances are both inside 2 then we don't need to move
    // else we move by 1 space on the x and y axes in the appropriate directions.
    if x.abs() < 2 && y.abs() < 2 {
        tail
    } else {
        (tx + x.signum(), ty + y.signum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
        ";
        const EXPECTED: usize = 13;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
        ";
        const EXPECTED: usize = 1;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2_extended() {
        // Arrange
        const INPUT: &str = "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
            ";
        const EXPECTED: usize = 36;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
