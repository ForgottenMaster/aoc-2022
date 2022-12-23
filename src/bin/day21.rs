use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day21.txt");
type NumericType = u64;

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> NumericType {
    let mut monkeys = parse_input(input);
    let mut root = monkeys.remove("root").unwrap();
    root.resolve(&mut monkeys)
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> (&str, Monkey) {
    let (name, expression) = input.trim().split_once(": ").unwrap();
    let monkey = if let Some((operand_1, operand_2)) = expression.split_once(" + ") {
        Monkey::Unresolved(ExpressionType::Add, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" - ") {
        Monkey::Unresolved(ExpressionType::Subtract, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" * ") {
        Monkey::Unresolved(ExpressionType::Multiply, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" / ") {
        Monkey::Unresolved(ExpressionType::Divide, operand_1, operand_2)
    } else {
        Monkey::Resolved(expression.parse().unwrap())
    };
    (name, monkey)
}

#[derive(Debug)]
enum Monkey<'a> {
    Unresolved(ExpressionType, &'a str, &'a str),
    Resolved(NumericType),
}

impl<'a> Monkey<'a> {
    fn resolve(&mut self, other_monkeys: &'_ mut HashMap<&'a str, Monkey<'a>>) -> NumericType {
        match self {
            Self::Resolved(value) => *value,
            Self::Unresolved(expression_type, operand_1_name, operand_2_name) => {
                let mut operand_1 = other_monkeys.remove(operand_1_name).unwrap();
                let mut operand_2 = other_monkeys.remove(operand_2_name).unwrap();
                let value_1 = operand_1.resolve(other_monkeys);
                let value_2 = operand_2.resolve(other_monkeys);
                other_monkeys.insert(operand_1_name, operand_1);
                other_monkeys.insert(operand_2_name, operand_2);
                let value = match expression_type {
                    ExpressionType::Add => value_1 + value_2,
                    ExpressionType::Subtract => value_1 - value_2,
                    ExpressionType::Multiply => value_1 * value_2,
                    ExpressionType::Divide => value_1 / value_2,
                };
                *self = Self::Resolved(value);
                value
            }
        }
    }
}

#[derive(Debug)]
enum ExpressionType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
        ";
        const EXPECTED: NumericType = 152;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
