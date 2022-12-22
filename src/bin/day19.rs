use rayon::prelude::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day19.txt");

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let blueprints = parse_blueprints(input);
    blueprints
        .par_iter()
        .map(|blueprint| {
            let mut cache = HashMap::new();
            let geodes = dfs(
                GameState {
                    ore_robot_count: 1,
                    clay_robot_count: 0,
                    obsidian_robot_count: 0,
                    geode_robot_count: 0,
                    ore_supply: 0,
                    clay_supply: 0,
                    obsidian_supply: 0,
                    geode_supply: 0,
                    time_remaining: 24,
                    blueprint: *blueprint,
                },
                &mut cache,
            );
            blueprint.id * geodes
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let blueprints = parse_blueprints(input);
    blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut cache = HashMap::new();
            dfs(
                GameState {
                    ore_robot_count: 1,
                    clay_robot_count: 0,
                    obsidian_robot_count: 0,
                    geode_robot_count: 0,
                    ore_supply: 0,
                    clay_supply: 0,
                    obsidian_supply: 0,
                    geode_supply: 0,
                    time_remaining: 32,
                    blueprint: *blueprint,
                },
                &mut cache,
            )
        })
        .product()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct GameState {
    ore_robot_count: u32,
    clay_robot_count: u32,
    obsidian_robot_count: u32,
    geode_robot_count: u32,
    ore_supply: u32,
    clay_supply: u32,
    obsidian_supply: u32,
    geode_supply: u32,
    time_remaining: u32,
    blueprint: Blueprint,
}

fn dfs(mut state: GameState, cache: &mut HashMap<GameState, u32>) -> u32 {
    if state.time_remaining == 0 {
        state.geode_supply
    } else {
        state = prepare_cache_friendly_state(state);
        if let Some(entry) = cache.get(&state) {
            *entry
        } else {
            let mut max = state.geode_supply;

            // Optimisation 1: Always build a geode bot when we can.
            if should_try_geode_bot(&state) {
                max = std::cmp::max(max, get_max_if_building_geode_bot(state, cache));
            } else {
                if should_try_obsidian_bot(&state) {
                    max = std::cmp::max(max, get_max_if_building_obsidian_bot(state, cache));
                }
                if should_try_clay_bot(&state) {
                    max = std::cmp::max(max, get_max_if_building_clay_bot(state, cache));
                }
                if should_try_ore_bot(&state) {
                    max = std::cmp::max(max, get_max_if_building_ore_bot(state, cache));
                }
                max = std::cmp::max(max, get_max_if_building_no_bot(state, cache));
            }
            cache.insert(state, max);
            max
        }
    }
}

fn should_try_geode_bot(state: &GameState) -> bool {
    state.ore_supply >= state.blueprint.geode_robot_ore_cost
        && state.obsidian_supply >= state.blueprint.geode_robot_obsidian_cost
}

fn should_try_obsidian_bot(state: &GameState) -> bool {
    state.ore_supply >= state.blueprint.obsidian_robot_ore_cost
        && state.clay_supply >= state.blueprint.obsidian_robot_clay_cost
        && state.obsidian_robot_count < state.blueprint.geode_robot_obsidian_cost
}

fn should_try_clay_bot(state: &GameState) -> bool {
    state.ore_supply >= state.blueprint.clay_robot_ore_cost
        && state.obsidian_robot_count < state.blueprint.geode_robot_obsidian_cost
        && state.clay_robot_count < state.blueprint.obsidian_robot_clay_cost
}

fn should_try_ore_bot(state: &GameState) -> bool {
    let max_ore_cost = std::cmp::max(
        std::cmp::max(
            std::cmp::max(
                state.blueprint.ore_robot_ore_cost,
                state.blueprint.clay_robot_ore_cost,
            ),
            state.blueprint.obsidian_robot_ore_cost,
        ),
        state.blueprint.geode_robot_ore_cost,
    );
    state.ore_supply >= state.blueprint.ore_robot_ore_cost && state.ore_robot_count < max_ore_cost
}

fn tick(mut state: GameState) -> GameState {
    state.ore_supply = state.ore_supply.saturating_add(state.ore_robot_count);
    state.clay_supply = state.clay_supply.saturating_add(state.clay_robot_count);
    state.obsidian_supply = state
        .obsidian_supply
        .saturating_add(state.obsidian_robot_count);
    state.geode_supply += state.geode_robot_count;
    state.time_remaining -= 1;
    state
}

fn prepare_cache_friendly_state(mut state: GameState) -> GameState {
    state.obsidian_supply = std::cmp::min(
        state.obsidian_supply,
        state.blueprint.geode_robot_obsidian_cost * 2,
    );
    state.clay_supply = std::cmp::min(
        state.clay_supply,
        state.blueprint.obsidian_robot_clay_cost * 2,
    );
    let max_ore_cost = std::cmp::max(
        std::cmp::max(
            std::cmp::max(
                state.blueprint.ore_robot_ore_cost,
                state.blueprint.clay_robot_ore_cost,
            ),
            state.blueprint.obsidian_robot_ore_cost,
        ),
        state.blueprint.geode_robot_ore_cost,
    );
    state.ore_supply = std::cmp::min(state.ore_supply, max_ore_cost * 2);
    state
}

fn get_max_if_building_geode_bot(mut state: GameState, cache: &mut HashMap<GameState, u32>) -> u32 {
    state.ore_supply -= state.blueprint.geode_robot_ore_cost;
    state.obsidian_supply -= state.blueprint.geode_robot_obsidian_cost;
    state = tick(state);
    state.geode_robot_count += 1;
    dfs(state, cache)
}

fn get_max_if_building_obsidian_bot(
    mut state: GameState,
    cache: &mut HashMap<GameState, u32>,
) -> u32 {
    state.ore_supply -= state.blueprint.obsidian_robot_ore_cost;
    state.clay_supply -= state.blueprint.obsidian_robot_clay_cost;
    state = tick(state);
    state.obsidian_robot_count += 1;
    dfs(state, cache)
}

fn get_max_if_building_clay_bot(mut state: GameState, cache: &mut HashMap<GameState, u32>) -> u32 {
    state.ore_supply -= state.blueprint.clay_robot_ore_cost;
    state = tick(state);
    state.clay_robot_count += 1;
    dfs(state, cache)
}

fn get_max_if_building_ore_bot(mut state: GameState, cache: &mut HashMap<GameState, u32>) -> u32 {
    state.ore_supply -= state.blueprint.ore_robot_ore_cost;
    state = tick(state);
    state.ore_robot_count += 1;
    dfs(state, cache)
}

fn get_max_if_building_no_bot(state: GameState, cache: &mut HashMap<GameState, u32>) -> u32 {
    dfs(tick(state), cache)
}

fn parse_blueprints(mut input: &str) -> Box<[Blueprint]> {
    let mut blueprints = Vec::new();
    while let Some(suffix) = input.trim().strip_prefix("Blueprint ") {
        // id
        let (id, suffix) = suffix.split_once(':').unwrap();
        let id = id.trim().parse().unwrap();
        input = suffix.trim();

        // ore robot
        input = input.strip_prefix("Each ore robot costs ").unwrap().trim();
        let (ore_robot_ore_cost, suffix) = input.split_once('.').unwrap();
        input = suffix.trim();
        let ore_robot_ore_cost = ore_robot_ore_cost
            .trim()
            .strip_suffix(" ore")
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        // clay robot
        input = input.strip_prefix("Each clay robot costs ").unwrap().trim();
        let (clay_robot_ore_cost, suffix) = input.split_once('.').unwrap();
        input = suffix.trim();
        let clay_robot_ore_cost = clay_robot_ore_cost
            .trim()
            .strip_suffix(" ore")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        // obsidian robot
        input = input
            .strip_prefix("Each obsidian robot costs ")
            .unwrap()
            .trim();
        let (obsidian_robot, suffix) = input.split_once('.').unwrap();
        input = suffix.trim();
        let (obsidian_robot_ore_cost, obsidian_robot_clay_cost) =
            obsidian_robot.split_once(" and ").unwrap();
        let obsidian_robot_ore_cost = obsidian_robot_ore_cost
            .trim()
            .strip_suffix(" ore")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let obsidian_robot_clay_cost = obsidian_robot_clay_cost
            .trim()
            .strip_suffix(" clay")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        // geode robot
        input = input
            .strip_prefix("Each geode robot costs ")
            .unwrap()
            .trim();
        let (geode_robot, suffix) = input.split_once('.').unwrap();
        input = suffix.trim();
        let (geode_robot_ore_cost, geode_robot_obsidian_cost) =
            geode_robot.split_once(" and ").unwrap();
        let geode_robot_ore_cost = geode_robot_ore_cost
            .trim()
            .strip_suffix(" ore")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let geode_robot_obsidian_cost = geode_robot_obsidian_cost
            .trim()
            .strip_suffix(" obsidian")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        // make blueprint
        blueprints.push(Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        });
    }
    blueprints.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        Blueprint 1:
          Each ore robot costs 4 ore.
          Each clay robot costs 2 ore.
          Each obsidian robot costs 3 ore and 14 clay.
          Each geode robot costs 2 ore and 7 obsidian.
        
        Blueprint 2:
          Each ore robot costs 2 ore.
          Each clay robot costs 3 ore.
          Each obsidian robot costs 3 ore and 8 clay.
          Each geode robot costs 3 ore and 12 obsidian.
        ";
        const EXPECTED: u32 = 33;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        Blueprint 1:
          Each ore robot costs 4 ore.
          Each clay robot costs 2 ore.
          Each obsidian robot costs 3 ore and 14 clay.
          Each geode robot costs 2 ore and 7 obsidian.
          
        Blueprint 2:
          Each ore robot costs 2 ore.
          Each clay robot costs 3 ore.
          Each obsidian robot costs 3 ore and 8 clay.
          Each geode robot costs 3 ore and 12 obsidian.
        ";
        const EXPECTED: u32 = 3472;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
