use core::cmp::Ordering;
use itertools::*;

const INPUT: &str = include_str!("../input/day13.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

#[derive(Debug, PartialEq)]
enum Token {
    Open(usize),
    Close(usize),
    Comma,
    Number(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    Integer(u32),
    List(Box<[Element]>),
}

impl Ord for Element {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Self::Integer(self_integer), Self::Integer(rhs_integer)) => {
                self_integer.cmp(rhs_integer)
            }
            (Self::Integer(_), rhs) => {
                Self::List(vec![(*self).clone()].into_boxed_slice()).cmp(rhs)
            }
            (_, rhs @ Self::Integer(_)) => {
                self.cmp(&Self::List(vec![(*rhs).clone()].into_boxed_slice()))
            }
            (Self::List(self_list), Self::List(rhs_list)) => {
                let (self_len, rhs_len) = (self_list.len(), rhs_list.len());
                let max_len = std::cmp::max(self_len, rhs_len);
                for i in 0..max_len {
                    if let (Some(self_element), Some(rhs_element)) =
                        (self_list.get(i), rhs_list.get(i))
                    {
                        let ordering = self_element.cmp(rhs_element);
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    } else if self_len < rhs_len {
                        return Ordering::Less;
                    } else if self_len > rhs_len {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, mut group)| {
            if is_empty {
                None
            } else {
                Some(parse_input(group.next().unwrap()) < parse_input(group.next().unwrap()))
            }
        })
        .enumerate()
        .filter_map(|(index, is_correct)| if is_correct { Some(index + 1) } else { None })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut elements = input
        .trim()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(parse_input(line))
            }
        })
        .collect::<Vec<_>>();
    let markers = vec![parse_input("[[2]]"), parse_input("[[6]]")];
    elements.extend(markers.iter().cloned());
    elements.sort();
    elements
        .into_iter()
        .enumerate()
        .filter_map(|(index, element)| {
            if markers.contains(&element) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product()
}

fn parse_input(input: &str) -> Element {
    tokenise(input)
        .fold(
            (vec![], vec![]),
            |(mut roots, mut stack), token| -> (Vec<Element>, Vec<Vec<Element>>) {
                match token {
                    Token::Open(count) => (0..count).for_each(|_| stack.push(vec![])),
                    Token::Number(number) => {
                        stack.last_mut().unwrap().push(Element::Integer(number))
                    }
                    Token::Close(count) => (0..count).for_each(|_| {
                        let top = stack.pop().unwrap();
                        if let Some(under) = stack.last_mut() {
                            under.push(Element::List(top.into_boxed_slice()));
                        } else {
                            roots.push(Element::List(top.into_boxed_slice()));
                        }
                    }),
                    Token::Comma => {} // ignore the commas
                };
                (roots, stack)
            },
        )
        .0
        .into_iter()
        .next()
        .unwrap()
}

fn tokenise(input: &str) -> impl Iterator<Item = Token> + '_ {
    input
        .trim()
        .chars()
        .peekable()
        .batching(|iter| match iter.peek() {
            None => None,
            Some(c @ '[' | c @ ']') => {
                let c = *c;
                let count = iter.peeking_take_while(|elem| *elem == c).count();
                Some(if c == '[' {
                    Token::Open(count)
                } else {
                    Token::Close(count)
                })
            }
            Some(',') => {
                iter.next(); // consume the comma from the stream.
                Some(Token::Comma)
            }
            Some(_) => Some(Token::Number(
                iter.peeking_take_while(|elem| elem.is_ascii_digit())
                    .fold(0, |total, c| total * 10 + c.to_digit(10).unwrap()),
            )),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    [1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";

    #[test]
    fn test_tokenise_flat() {
        // Arrange
        const INPUT: &str = "[1,1,10,1,1]";
        const EXPECTED: &[Token] = &[
            Token::Open(1),
            Token::Number(1),
            Token::Comma,
            Token::Number(1),
            Token::Comma,
            Token::Number(10),
            Token::Comma,
            Token::Number(1),
            Token::Comma,
            Token::Number(1),
            Token::Close(1),
        ];

        // Act
        let output = tokenise(INPUT).collect::<Vec<_>>();

        // Assert
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_tokenise_deeply_nested() {
        // Arrange
        const INPUT: &str = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        const EXPECTED: &[Token] = &[
            Token::Open(1),
            Token::Number(1),
            Token::Comma,
            Token::Open(1),
            Token::Number(2),
            Token::Comma,
            Token::Open(1),
            Token::Number(3),
            Token::Comma,
            Token::Open(1),
            Token::Number(4),
            Token::Comma,
            Token::Open(1),
            Token::Number(5),
            Token::Comma,
            Token::Number(6),
            Token::Comma,
            Token::Number(7),
            Token::Close(4),
            Token::Comma,
            Token::Number(8),
            Token::Comma,
            Token::Number(9),
            Token::Close(1),
        ];

        // Act
        let output = tokenise(INPUT).collect::<Vec<_>>();

        // Assert
        assert_eq!(&output, EXPECTED);
    }

    #[test]
    fn test_parse_input_flat() {
        // Arrange
        const INPUT: &str = "[1,1,13,1,1]";
        let expected = Element::List(
            vec![
                Element::Integer(1),
                Element::Integer(1),
                Element::Integer(13),
                Element::Integer(1),
                Element::Integer(1),
            ]
            .into_boxed_slice(),
        );

        // Act
        let output = parse_input(INPUT);

        // Assert
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_input_deeply_nested() {
        // Arrange
        const INPUT: &str = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let expected = Element::List(
            vec![
                Element::Integer(1),
                Element::List(
                    vec![
                        Element::Integer(2),
                        Element::List(
                            vec![
                                Element::Integer(3),
                                Element::List(
                                    vec![
                                        Element::Integer(4),
                                        Element::List(
                                            vec![
                                                Element::Integer(5),
                                                Element::Integer(6),
                                                Element::Integer(7),
                                            ]
                                            .into_boxed_slice(),
                                        ),
                                    ]
                                    .into_boxed_slice(),
                                ),
                            ]
                            .into_boxed_slice(),
                        ),
                    ]
                    .into_boxed_slice(),
                ),
                Element::Integer(8),
                Element::Integer(9),
            ]
            .into_boxed_slice(),
        );

        // Act
        let output = parse_input(INPUT);

        // Assert
        assert_eq!(output, expected);
    }

    #[test]
    fn test_correct_order_1() {
        // Arrange
        let first = parse_input("[1,1,3,1,1]");
        let second = parse_input("[1,1,5,1,1]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(in_right_order);
    }

    #[test]
    fn test_correct_order_2() {
        // Arrange
        let first = parse_input("[[1],[2,3,4]]");
        let second = parse_input("[[1],4]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(in_right_order);
    }

    #[test]
    fn test_correct_order_3() {
        // Arrange
        let first = parse_input("[[4,4],4,4]");
        let second = parse_input("[[4,4],4,4,4]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(in_right_order);
    }

    #[test]
    fn test_correct_order_4() {
        // Arrange
        let first = parse_input("[]");
        let second = parse_input("[3]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(in_right_order);
    }

    #[test]
    fn test_incorrect_order_1() {
        // Arrange
        let first = parse_input("[9]");
        let second = parse_input("[[8,7,6]]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(!in_right_order);
    }

    #[test]
    fn test_incorrect_order_2() {
        // Arrange
        let first = parse_input("[7,7,7,7]");
        let second = parse_input("[7,7,7]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(!in_right_order);
    }

    #[test]
    fn test_incorrect_order_3() {
        // Arrange
        let first = parse_input("[[[]]]");
        let second = parse_input("[[]]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(!in_right_order);
    }

    #[test]
    fn test_incorrect_order_4() {
        // Arrange
        let first = parse_input("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let second = parse_input("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        // Act
        let in_right_order = first < second;

        // Assert
        assert!(!in_right_order);
    }

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: usize = 13;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const EXPECTED: usize = 140;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
