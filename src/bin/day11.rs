type Number = u64;

const INPUT: &str = include_str!("../input/day11.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> Number {
    input
        .parse::<VecMonkey>()
        .unwrap()
        .run_simulation(20, |value| value / 3)
}

fn part_2(input: &str) -> Number {
    let monkeys = input.parse::<VecMonkey>().unwrap();
    let modulus = monkeys.iter_test_divisible_by_values().product::<Number>();
    monkeys.run_simulation(10_000, |value| value % modulus)
}

mod private {
    use super::*;
    use core::convert::Infallible;
    use core::str::FromStr;
    use itertools::*;

    #[derive(Debug)]
    enum Operation {
        Squared,
        Add(Number),
        Multiply(Number),
    }

    impl Operation {
        fn apply(&self, item: Number) -> Number {
            match self {
                Self::Squared => item * item,
                Self::Add(amount) => item + amount,
                Self::Multiply(amount) => item * amount,
            }
        }
    }

    impl FromStr for Operation {
        type Err = Infallible;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let input = input.trim();
            if input == "old * old" {
                Ok(Self::Squared)
            } else if let Some(suffix) = input.strip_prefix("old + ") {
                Ok(Self::Add(suffix.parse().unwrap()))
            } else if let Some(suffix) = input.strip_prefix("old * ") {
                Ok(Self::Multiply(suffix.parse().unwrap()))
            } else {
                unimplemented!()
            }
        }
    }

    #[derive(Debug)]
    pub struct Monkey {
        items: Vec<Number>,
        operation: Operation,
        test_divisible_by: Number,
        if_true_throw_to_monkey: usize,
        if_false_throw_to_monkey: usize,
    }

    impl Monkey {
        fn catch_item(&mut self, item: Number) {
            self.items.push(item);
        }

        fn try_throw_first_item(
            &mut self,
            mut capping_func: impl FnMut(Number) -> Number,
        ) -> Option<(Number, usize)> {
            if self.items.is_empty() {
                None
            } else {
                let item = self.items.remove(0);
                let item = capping_func(self.operation.apply(item));
                let throw_to_index = if item % self.test_divisible_by == 0 {
                    self.if_true_throw_to_monkey
                } else {
                    self.if_false_throw_to_monkey
                };
                Some((item, throw_to_index))
            }
        }
    }

    impl<'a> FromIterator<&'a str> for Monkey {
        fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = &'a str>,
        {
            let mut iter = iter.into_iter().skip(1).map(|line| line.trim()); // skip first element because it's the monkey name/index which we can get from the index in the vector we'll place them in.
            let items = iter
                .next()
                .unwrap()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|worry| worry.parse().unwrap())
                .collect();
            let operation = iter
                .next()
                .unwrap()
                .strip_prefix("Operation: new = ")
                .unwrap()
                .parse()
                .unwrap();
            let test_divisible_by = iter
                .next()
                .unwrap()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let if_true_throw_to_monkey = iter
                .next()
                .unwrap()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let if_false_throw_to_monkey = iter
                .next()
                .unwrap()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            Self {
                items,
                operation,
                test_divisible_by,
                if_true_throw_to_monkey,
                if_false_throw_to_monkey,
            }
        }
    }

    #[derive(Debug)]
    pub struct VecMonkey(Vec<Monkey>);

    impl VecMonkey {
        pub fn run_simulation(
            mut self,
            rounds: u16,
            mut capping_func: impl FnMut(Number) -> Number,
        ) -> Number {
            let mut counts = std::iter::repeat(0).take(self.0.len()).collect::<Vec<_>>();
            (0..rounds).for_each(|_| {
                (0..self.0.len()).for_each(|index| {
                    while let Some((item, throw_to_index)) =
                        self.0[index].try_throw_first_item(&mut capping_func)
                    {
                        counts[index] += 1;
                        self.0[throw_to_index].catch_item(item);
                    }
                });
            });
            counts.sort_by(|item_1, item_2| item_2.cmp(item_1));
            counts[0] * counts[1]
        }

        pub fn iter_test_divisible_by_values(&self) -> impl Iterator<Item = Number> + '_ {
            self.0.iter().map(|monkey| monkey.test_divisible_by)
        }
    }

    impl FromStr for VecMonkey {
        type Err = Infallible;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                input
                    .trim()
                    .lines()
                    .group_by(|line| line.trim().is_empty())
                    .into_iter()
                    .filter(|(is_empty, _)| !is_empty)
                    .map(|(_, group)| group.collect())
                    .collect(),
            ))
        }
    }
}
use private::*;

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
    
    Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
    
    Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
    
    Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: Number = 10605;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: Number = 2713310158;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
