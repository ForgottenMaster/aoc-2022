use std::collections::HashSet;

type Coordinate = (i32, i32);

const INPUT: &str = include_str!("../input/day09.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let (_, _, visited) = input.trim().lines().fold(
        ((0, 0), (0, 0), {
            let mut visited = HashSet::new();
            visited.insert((0, 0));
            visited
        }),
        |(mut head, mut tail, mut visited), line| {
            let mut splits = line.split_whitespace();
            let direction = splits.next().unwrap();
            let amount = splits.next().unwrap().parse::<u32>().unwrap();
            (0..amount).for_each(|_| {
                let (hx, hy) = head;
                head = match direction {
                    "R" => (hx + 1, hy),
                    "L" => (hx - 1, hy),
                    "U" => (hx, hy - 1),
                    "D" => (hx, hy + 1),
                    _ => unimplemented!(),
                };
                tail = move_tail(head, tail);
                visited.insert(tail);
            });
            (head, tail, visited)
        },
    );
    visited.len()
}

fn move_tail(head: Coordinate, tail: Coordinate) -> Coordinate {
    match (head, tail) {
        ((hx, hy), (tx, _)) if hx == tx - 2 => (hx + 1, hy),
        ((hx, hy), (tx, _)) if hx == tx + 2 => (hx - 1, hy),
        ((hx, hy), (_, ty)) if hy == ty - 2 => (hx, hy + 1),
        ((hx, hy), (_, ty)) if hy == ty + 2 => (hx, hy - 1),
        _ => tail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: usize = 13;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
