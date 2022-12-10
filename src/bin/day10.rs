const INPUT: &str = include_str!("../input/day10.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
}

fn part_1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .scan((1_i32, 1), |(previous, current), line| {
            let line = line.trim();
            let (delta, cycles) = if line == "noop" {
                (0, 1)
            } else if let Some(delta) = line.strip_prefix("addx ") {
                (delta.parse().unwrap(), 2)
            } else {
                unimplemented!()
            };
            *previous = *current;
            *current += delta;
            Some((*previous, *current, cycles))
        })
        .flat_map(|(previous, current, cycles)| {
            std::iter::repeat(previous)
                .take(cycles - 1)
                .chain(std::iter::once(current))
        })
        .enumerate()
        .skip(18) // skip first 18 because we want the first signal strength AFTER The 19th cycle (during 20th cycle)
        .step_by(40) // step by 40 each time
        .map(|(index, register)| {
            let during_cycle = index as i32 + 2; // index 18 is AFTER cycle 19, which is DURING cycle 20.
            during_cycle * register
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    ";

    #[test]
    fn test_part_1() {
        // Arrange
        const EXPECTED: i32 = 13140;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
