use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day18.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let points = input
        .trim()
        .lines()
        .map(parse_point)
        .collect::<HashSet<_>>();
    points
        .iter()
        .flat_map(|point| get_neighboring_points(*point).filter(|point| !points.contains(point)))
        .count()
}

fn part_2(input: &str) -> usize {
    let points = input
        .trim()
        .lines()
        .map(parse_point)
        .collect::<HashSet<_>>();
    let bounds = get_bounds(&points);
    let exterior_points = flood_exterior(bounds, &points);
    points
        .iter()
        .flat_map(|point| {
            get_neighboring_points(*point).filter(|point| exterior_points.contains(point))
        })
        .count()
}

fn flood_exterior(
    bounds: ((i32, i32, i32), (i32, i32, i32)),
    points: &HashSet<(i32, i32, i32)>,
) -> HashSet<(i32, i32, i32)> {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = bounds;
    let capacity = (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
    let mut exterior_points = HashSet::with_capacity(capacity as usize);
    exterior_points.insert((0, 0, 0));
    let mut frontier = vec![(0, 0, 0)];
    while let Some(to_expand) = frontier.pop() {
        for point in get_neighboring_points(to_expand) {
            if inside_bounds(point, bounds)
                && !points.contains(&point)
                && exterior_points.insert(point)
            {
                frontier.push(point);
            }
        }
    }
    exterior_points
}

fn inside_bounds(point: (i32, i32, i32), bounds: ((i32, i32, i32), (i32, i32, i32))) -> bool {
    let (x, y, z) = point;
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = bounds;
    x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z
}

fn get_bounds(points: &HashSet<(i32, i32, i32)>) -> ((i32, i32, i32), (i32, i32, i32)) {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = points.iter().copied().fold(
        (
            (i32::MAX, i32::MAX, i32::MAX),
            (i32::MIN, i32::MIN, i32::MIN),
        ),
        |((min_x, min_y, min_z), (max_x, max_y, max_z)), (x, y, z)| {
            (
                (
                    std::cmp::min(x, min_x),
                    std::cmp::min(y, min_y),
                    std::cmp::min(z, min_z),
                ),
                (
                    std::cmp::max(x, max_x),
                    std::cmp::max(y, max_y),
                    std::cmp::max(z, max_z),
                ),
            )
        },
    );
    (
        (min_x - 1, min_y - 1, min_z - 1),
        (max_x + 1, max_y + 1, max_z + 1),
    )
}

fn get_neighboring_points(point: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let (x, y, z) = point;
    std::iter::once((x - 1, y, z))
        .chain(std::iter::once((x + 1, y, z)))
        .chain(std::iter::once((x, y - 1, z)))
        .chain(std::iter::once((x, y + 1, z)))
        .chain(std::iter::once((x, y, z - 1)))
        .chain(std::iter::once((x, y, z + 1)))
}

fn parse_point(input: &str) -> (i32, i32, i32) {
    let mut iter = input
        .trim()
        .split(',')
        .map(|elem| elem.parse::<i32>().unwrap());
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
        ";
        const EXPECTED: usize = 64;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
        ";
        const EXPECTED: usize = 58;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
