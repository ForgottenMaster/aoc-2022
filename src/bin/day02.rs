const INPUT: &str = include_str!("../input/day02.txt");

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    input.trim().lines().map(process_line_part_1).sum()
}

fn part_2(input: &str) -> u32 {
    input.trim().lines().map(process_line_part_2).sum()
}

fn process_line_part_1(input: &str) -> u32 {
    let mut parts = input.split_whitespace().map(get_shape_from_string);
    let (opponent_shape, my_shape) = (parts.next().unwrap(), parts.next().unwrap());
    let outcome = get_outcome_for_shapes(my_shape, opponent_shape);
    (my_shape as u32) + (outcome as u32)
}

fn process_line_part_2(input: &str) -> u32 {
    let mut parts = input.split_whitespace();
    let (opponent_shape, desired_outcome) = (
        get_shape_from_string(parts.next().unwrap()),
        get_outcome_from_string(parts.next().unwrap()),
    );
    let my_shape = get_shape_for_desired_outcome(opponent_shape, desired_outcome);
    (my_shape as u32) + (desired_outcome as u32)
}

fn get_shape_from_string(input: &str) -> Shape {
    match input {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unimplemented!(),
    }
}

fn get_shape_for_desired_outcome(opponent_shape: Shape, desired_outcome: Outcome) -> Shape {
    match opponent_shape {
        Shape::Rock => match desired_outcome {
            Outcome::Win => Shape::Paper,
            Outcome::Draw => Shape::Rock,
            Outcome::Loss => Shape::Scissors,
        },
        Shape::Paper => match desired_outcome {
            Outcome::Win => Shape::Scissors,
            Outcome::Draw => Shape::Paper,
            Outcome::Loss => Shape::Rock,
        },
        Shape::Scissors => match desired_outcome {
            Outcome::Win => Shape::Rock,
            Outcome::Draw => Shape::Scissors,
            Outcome::Loss => Shape::Paper,
        },
    }
}

fn get_outcome_from_string(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unimplemented!(),
    }
}

fn get_outcome_for_shapes(my_shape: Shape, opponent_shape: Shape) -> Outcome {
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

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: u32 = 12;

        // Act
        let output = part_2(TEST_INPUT);

        // Assert
        assert_eq!(EXPECTED, output);
    }
}
