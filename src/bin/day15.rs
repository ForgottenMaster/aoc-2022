use itertools::*;

const INPUT: &str = include_str!("../input/day15.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT, 2_000_000));
    println!("Part 2 => {}", part_2(INPUT, 4_000_000));
}

fn part_1(input: &str, y: i64) -> u64 {
    let sensors = get_sensor_data(input);
    let (min_x, max_x) = sensors
        .iter()
        .fold((i64::MAX, i64::MIN), |(min_x, max_x), sensor| {
            let candidate_min_x = sensor.sensor_pos.0 - sensor.distance as i64;
            let candidate_max_x = sensor.sensor_pos.0 + sensor.distance as i64;
            (
                std::cmp::min(min_x, candidate_min_x),
                std::cmp::max(max_x, candidate_max_x),
            )
        });
    (min_x..=max_x)
        .filter_map(|x| {
            let coord = (x, y);
            if !sensors
                .iter()
                .any(|sensor| sensor.sensor_pos == coord || sensor.beacon_pos == coord)
                && sensors.iter().any(|sensor| {
                    get_manhattan_distance(sensor.sensor_pos, coord) <= sensor.distance
                })
            {
                Some(())
            } else {
                None
            }
        })
        .count() as u64
}

fn part_2(input: &str, max: i64) -> u64 {
    let sensors = get_sensor_data(input);
    let coord = sensors
        .iter()
        .flat_map(|sensor| perimeter_coordinate_iter(sensor.sensor_pos, sensor.distance as i64 + 1))
        .find(|coord| {
            coord.0 >= 0
                && coord.1 >= 0
                && coord.0 <= max
                && coord.1 <= max
                && !sensors.iter().any(|sensor| {
                    get_manhattan_distance(sensor.sensor_pos, *coord) <= sensor.distance
                })
        })
        .unwrap();
    (coord.0 * 4_000_000 + coord.1) as u64
}

fn perimeter_coordinate_iter(
    center_point: (i64, i64),
    radius: i64,
) -> impl Iterator<Item = (i64, i64)> {
    (0..=radius).flat_map(move |vertical_deviation| {
        let min_x = center_point.0 - radius + vertical_deviation;
        let max_x = center_point.0 + radius - vertical_deviation;
        if vertical_deviation == 0 {
            Either::Left(
                std::iter::once((min_x, center_point.1))
                    .chain(std::iter::once((max_x, center_point.1))),
            )
        } else {
            Either::Right(
                std::iter::once((min_x, center_point.1 - vertical_deviation))
                    .chain(std::iter::once((
                        max_x,
                        center_point.1 - vertical_deviation,
                    )))
                    .chain(std::iter::once((
                        min_x,
                        center_point.1 + vertical_deviation,
                    )))
                    .chain(std::iter::once((
                        max_x,
                        center_point.1 + vertical_deviation,
                    ))),
            )
        }
    })
}

#[derive(Debug)]
struct SensorData {
    sensor_pos: (i64, i64),
    beacon_pos: (i64, i64),
    distance: u64,
}

fn get_sensor_data(input: &str) -> Box<[SensorData]> {
    input
        .trim()
        .lines()
        .map(process_line)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn process_line(input: &str) -> SensorData {
    let mut splits = input.trim().split(": ");
    let sensor_pos = get_sensor_pos(splits.next().unwrap());
    let beacon_pos = get_beacon_pos(splits.next().unwrap());
    let distance = get_manhattan_distance(sensor_pos, beacon_pos);
    SensorData {
        sensor_pos,
        beacon_pos,
        distance,
    }
}

fn get_sensor_pos(input: &str) -> (i64, i64) {
    let mut splits = input
        .trim()
        .strip_prefix("Sensor at ")
        .unwrap()
        .split(", ")
        .map(|split| split.split('=').nth(1).unwrap().parse().unwrap());
    (splits.next().unwrap(), splits.next().unwrap())
}

fn get_beacon_pos(input: &str) -> (i64, i64) {
    let mut splits = input
        .trim()
        .strip_prefix("closest beacon ")
        .unwrap()
        .split(", ")
        .map(|split| split.split('=').nth(1).unwrap().parse().unwrap());
    (splits.next().unwrap(), splits.next().unwrap())
}

fn get_manhattan_distance(first: (i64, i64), second: (i64, i64)) -> u64 {
    ((first.0 - second.0).abs() + (first.1 - second.1).abs()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        ";
        const Y_TO_COUNT: i64 = 10;
        const EXPECTED: u64 = 26;

        // Act
        let output = part_1(INPUT, Y_TO_COUNT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        ";
        const EXPECTED: u64 = 56000011;

        // Act
        let output = part_2(INPUT, 20);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
