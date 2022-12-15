const INPUT: &str = include_str!("../input/day15.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT, 2_000_000));
}

fn part_1(input: &str, y: i32) -> u32 {
    let (sensors, min_x, max_x) = get_sensor_data(input);
    count_invalid_spaces(&sensors, y, min_x, max_x)
}

fn count_invalid_spaces(sensors: &[SensorData], y: i32, min_x: i32, max_x: i32) -> u32 {
    (min_x..=max_x).fold(0, |count, x| {
        let coord = (x, y);
        if coord_is_invalid_for_all_sensors(sensors, coord) {
            count + 1
        } else {
            count
        }
    })
}

fn coord_is_invalid_for_all_sensors(sensors: &[SensorData], coord: (i32, i32)) -> bool {
    // if it's any beacon coordinate then it can't be invalid.
    // otherwise it needs to be within range of one sensor to be invalid.
    !sensors.iter().any(|sensor| sensor.beacon_pos == coord)
        && sensors
            .iter()
            .any(|sensor| get_manhattan_distance(sensor.sensor_pos, coord) <= sensor.distance)
}

#[derive(Debug)]
struct SensorData {
    sensor_pos: (i32, i32),
    beacon_pos: (i32, i32),
    distance: u32,
}

fn get_sensor_data(input: &str) -> (Vec<SensorData>, i32, i32) {
    let data = input.trim().lines().map(process_line).collect::<Vec<_>>();
    let (min_x, max_x) = data
        .iter()
        .fold((i32::MAX, i32::MIN), |(min_x, max_x), data| {
            let candidate_min_x = data.sensor_pos.0 - data.distance as i32;
            let candidate_max_x = data.sensor_pos.0 + data.distance as i32;
            (
                std::cmp::min(min_x, candidate_min_x),
                std::cmp::max(max_x, candidate_max_x),
            )
        });
    (data, min_x, max_x)
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

fn get_sensor_pos(input: &str) -> (i32, i32) {
    let mut splits = input
        .trim()
        .strip_prefix("Sensor at ")
        .unwrap()
        .split(", ")
        .map(|split| split.split('=').nth(1).unwrap().parse().unwrap());
    (splits.next().unwrap(), splits.next().unwrap())
}

fn get_beacon_pos(input: &str) -> (i32, i32) {
    let mut splits = input
        .trim()
        .strip_prefix("closest beacon ")
        .unwrap()
        .split(", ")
        .map(|split| split.split('=').nth(1).unwrap().parse().unwrap());
    (splits.next().unwrap(), splits.next().unwrap())
}

fn get_manhattan_distance(first: (i32, i32), second: (i32, i32)) -> u32 {
    ((first.0 - second.0).abs() + (first.1 - second.1).abs()) as u32
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
        const Y_TO_COUNT: i32 = 10;
        const EXPECTED: u32 = 26;

        // Act
        let output = part_1(INPUT, Y_TO_COUNT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
