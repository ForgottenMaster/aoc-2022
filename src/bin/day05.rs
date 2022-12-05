use itertools::*;

const INPUT: &str = include_str!("../input/day05.txt");

type Stack = Vec<char>;
type Command = (usize, usize, usize); // (amount, from_index, to_index)

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> String {
    let (mut stacks, commands) = extract_stacks_and_commands(input);
    apply_commands_to_stacks(commands.into_iter(), &mut stacks);
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn apply_commands_to_stacks(commands: impl Iterator<Item = Command>, stacks: &mut [Stack]) {
    commands.for_each(|(amount, from_index, to_index)| {
        (0..amount).for_each(|_| {
            let elem = stacks[from_index].pop().unwrap();
            stacks[to_index].push(elem);
        });
    });
}

fn extract_stacks_and_commands(input: &str) -> (Vec<Stack>, Vec<Command>) {
    let groups = input.lines().group_by(|line| line.trim().is_empty());
    let mut sections = groups
        .into_iter()
        .skip_while(|(key, _)| *key)
        .filter_map(|(key, group)| if key { None } else { Some(group) });
    (
        extract_stacks(sections.next().unwrap()),
        extract_commands(sections.next().unwrap()),
    )
}

fn extract_stacks<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Stack> {
    // bit annoying but we can't reverse a regular iterator so have to collect into a Vec
    // first to get a double-ended one.
    let mut input = input.collect::<Vec<_>>().into_iter().rev();

    // number of stacks we can find by just grabbing the highest value in the list of numbers on the
    // last line of the section.
    let number_of_stacks = extract_number_of_stacks(input.next().unwrap());

    // create empty stacks.
    let mut stacks = allocate_stacks(number_of_stacks);

    // for the stack elements themselves, we're essentially taking
    // groups of 4 characters to process. We can do this with the chunks
    // method on slices.
    input.for_each(|line| {
        line.chars()
            .chunks(4)
            .into_iter()
            .map(extract_stack_element)
            .enumerate()
            .for_each(|(index, elem)| {
                if elem != ' ' {
                    stacks[index].push(elem);
                }
            });
    });

    // return initialised stacks.
    stacks
}

fn extract_stack_element(mut chunk: impl Iterator<Item = char>) -> char {
    chunk.nth(1).unwrap()
}

fn allocate_stacks(number_of_stacks: usize) -> Vec<Stack> {
    let mut stacks = Vec::with_capacity(number_of_stacks);
    (0..number_of_stacks).for_each(|_| {
        stacks.push(Vec::new());
    });
    stacks
}

fn extract_number_of_stacks(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .rev()
        .next()
        .unwrap()
}

fn extract_commands<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Command> {
    input.map(extract_command).collect()
}

fn extract_command(input: &str) -> Command {
    let mut values = input
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|elem| elem.parse::<usize>().unwrap());
    (
        values.next().unwrap(),
        values.next().unwrap() - 1,
        values.next().unwrap() - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: &str = "CMZ";

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(&output, EXPECTED);
    }
}
