const INPUT: &str = include_str!("../input/day20.txt");

type NumberType = i64;

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> NumberType {
    let numbers = parse_input(input);
    let numbers = mix(numbers);
    extract_answer(numbers)
}

fn part_2(input: &str) -> NumberType {
    let numbers = parse_input(input);
    let mut numbers = numbers
        .into_iter()
        .map(|(index, value)| (index, value * 811589153))
        .collect();
    for _ in 0..10 {
        numbers = mix(numbers);
    }
    extract_answer(numbers)
}

fn parse_input(input: &str) -> Vec<(usize, NumberType)> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .enumerate()
        .collect()
}

fn mix(mut numbers: Vec<(usize, NumberType)>) -> Vec<(usize, NumberType)> {
    (0..numbers.len()).for_each(|order_to_find| {
        let (index, number, order) = numbers
            .iter()
            .enumerate()
            .filter_map(|(index, (order, number))| {
                if order_to_find == *order {
                    Some((index, *number, *order))
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        if number != 0 {
            numbers.retain(|(other_order, _)| *other_order != order);
            let mut index = (index as NumberType + number) % numbers.len() as NumberType;
            if index < 0 {
                index += numbers.len() as NumberType;
            }
            numbers.insert(index as usize, (order, number));
        }
    });
    numbers
}

fn extract_answer(numbers: Vec<(usize, NumberType)>) -> NumberType {
    numbers
        .into_iter()
        .map(|(_, number)| number)
        .cycle()
        .skip_while(|number| *number != 0)
        .skip(1000)
        .step_by(1000)
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        1
        2
        -3
        3
        -2
        0
        4
        ";
        const EXPECTED: NumberType = 3;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        1
        2
        -3
        3
        -2
        0
        4
        ";
        const EXPECTED: NumberType = 1623178306;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
