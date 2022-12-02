const INPUT: &str = include_str!("../input/day02.txt");

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> u32 {
    input.trim().lines().map(process_line).sum()
}

fn process_line(input: &str) -> u32 {
    let mut parts = input.split_whitespace().map(get_shape);
    let (opponent_shape, my_shape) = (parts.next().unwrap(), parts.next().unwrap());
    let outcome = get_outcome(my_shape, opponent_shape);
    (my_shape as u32) + (outcome as u32)
}

fn get_shape(input: &str) -> Shape {
    match input {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unimplemented!(),
    }
}

fn get_outcome(my_shape: Shape, opponent_shape: Shape) -> Outcome {
    match (my_shape, opponent_shape) {
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => Outcome::Loss,
        (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper)
        | (Shape::Rock, Shape::Scissors) => Outcome::Win,
        _ => Outcome::Draw,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    A Y
    B X
    C Z
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: u32 = 15;

        // Act
        let output = part_1(TEST_INPUT);

        // Assert
        assert_eq!(EXPECTED, output);
    }
}
