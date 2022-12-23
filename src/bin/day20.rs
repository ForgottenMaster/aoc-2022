const INPUT: &str = include_str!("../input/day20.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> i32 {
    let numbers = parse_input(input);
    let numbers = mix(numbers);
    extract_answer(numbers)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

fn mix(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers.into_iter().enumerate().collect::<Vec<_>>();
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
            let mut index = index as i32 + number;
            while index < 0 {
                index += numbers.len() as i32;
            }
            while index >= numbers.len() as i32 {
                index -= numbers.len() as i32;
            }
            numbers.insert(index as usize, (order, number));
        }
    });
    numbers.into_iter().map(|(_, number)| number).collect()
}

fn extract_answer(numbers: Vec<i32>) -> i32 {
    numbers
        .into_iter()
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
        const EXPECTED: i32 = 3;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
