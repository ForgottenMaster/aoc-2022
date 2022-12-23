use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day21.txt");
type NumericType = u64;

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> NumericType {
    let monkeys = parse_input(input);
    resolve(&monkeys, "root")
}

fn part_2(input: &str) -> NumericType {
    let monkeys = parse_input(input);
    let (operand_1, operand_2) = get_monkey_operands(&monkeys, "root");
    let (mut contains_humn, mut does_not_contain_humn, _) =
        sort_branches(&monkeys, operand_1, operand_2);
    let mut value_to_match = resolve(&monkeys, does_not_contain_humn);
    loop {
        match &monkeys[contains_humn] {
            Monkey::Expression(expression_type, operand_1, operand_2) => {
                let (local_contains_humn, local_does_not_contain_humn, operand_1_contains_humn) =
                    sort_branches(&monkeys, operand_1, operand_2);
                contains_humn = local_contains_humn;
                does_not_contain_humn = local_does_not_contain_humn;
                let does_not_contain_branch_value = resolve(&monkeys, does_not_contain_humn);
                value_to_match = match (expression_type, operand_1_contains_humn) {
                    (ExpressionType::Add, _) => value_to_match - does_not_contain_branch_value,
                    (ExpressionType::Subtract, true) => {
                        value_to_match + does_not_contain_branch_value
                    }
                    (ExpressionType::Subtract, false) => {
                        does_not_contain_branch_value - value_to_match
                    }
                    (ExpressionType::Multiply, _) => value_to_match / does_not_contain_branch_value,
                    (ExpressionType::Divide, true) => {
                        value_to_match * does_not_contain_branch_value
                    }
                    (ExpressionType::Divide, false) => {
                        does_not_contain_branch_value / value_to_match
                    }
                }
            }
            _ => break value_to_match,
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> (&str, Monkey) {
    let (name, expression) = input.trim().split_once(": ").unwrap();
    let monkey = if let Some((operand_1, operand_2)) = expression.split_once(" + ") {
        Monkey::Expression(ExpressionType::Add, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" - ") {
        Monkey::Expression(ExpressionType::Subtract, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" * ") {
        Monkey::Expression(ExpressionType::Multiply, operand_1, operand_2)
    } else if let Some((operand_1, operand_2)) = expression.split_once(" / ") {
        Monkey::Expression(ExpressionType::Divide, operand_1, operand_2)
    } else {
        Monkey::Number(expression.parse().unwrap())
    };
    (name, monkey)
}

// Returns:
// (contains_humn, does_not_contain_humn)
fn sort_branches<'a>(
    monkeys: &HashMap<&str, Monkey>,
    operand_1: &'a str,
    operand_2: &'a str,
) -> (&'a str, &'a str, bool) {
    let operand_1_contains_humn = contains_node(monkeys, operand_1, "humn");
    if operand_1_contains_humn {
        (operand_1, operand_2, true)
    } else {
        (operand_2, operand_1, false)
    }
}

fn contains_node(monkeys: &HashMap<&str, Monkey>, root: &str, needle: &str) -> bool {
    if root == needle {
        true
    } else {
        match monkeys[root] {
            Monkey::Expression(_, operand_1, operand_2) => {
                contains_node(monkeys, operand_1, needle)
                    || contains_node(monkeys, operand_2, needle)
            }
            Monkey::Number(_) => false,
        }
    }
}

fn get_monkey_operands<'a, 'b: 'a>(
    monkeys: &'a HashMap<&'b str, Monkey<'b>>,
    name: &str,
) -> (&'b str, &'b str) {
    match monkeys[name] {
        Monkey::Expression(_, operand_1, operand_2) => (operand_1, operand_2),
        _ => unimplemented!("assuming we aren't getting monkeys that don't exist."),
    }
}

fn resolve(mapping: &HashMap<&str, Monkey>, root: &str) -> NumericType {
    match mapping[root] {
        Monkey::Number(number) => number,
        Monkey::Expression(ExpressionType::Add, operand_1, operand_2) => {
            resolve(mapping, operand_1) + resolve(mapping, operand_2)
        }
        Monkey::Expression(ExpressionType::Subtract, operand_1, operand_2) => {
            resolve(mapping, operand_1) - resolve(mapping, operand_2)
        }
        Monkey::Expression(ExpressionType::Multiply, operand_1, operand_2) => {
            resolve(mapping, operand_1) * resolve(mapping, operand_2)
        }
        Monkey::Expression(ExpressionType::Divide, operand_1, operand_2) => {
            resolve(mapping, operand_1) / resolve(mapping, operand_2)
        }
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Expression(ExpressionType, &'a str, &'a str),
    Number(NumericType),
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

    #[test]
    fn test_part_2() {
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
        const EXPECTED: NumericType = 301;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
