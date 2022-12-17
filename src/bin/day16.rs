/// I'm too stupid to be able to solve this puzzle on my own so after a day of banging my head against it
/// I set out to look at hints. However this solution is basically the one described in this YouTube video
/// so credit where credit's due: https://www.youtube.com/watch?v=bLMj50cpOug
use {
    petgraph::{algo::astar, graph::NodeIndex, Direction, Graph},
    std::collections::BTreeMap,
};

const INPUT: &str = include_str!("../input/day16.txt");

type GraphType<'a> = Graph<(&'a str, u32), u32>;

#[cfg(not(tarpaulin))]
fn main() {
    println!("Part 1 => {}", part_1(INPUT));
    println!("Part 2 => {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let graph = extract_node_graph(input);
    let starting_node = get_node_index("AA", &graph).unwrap();
    let opened = 0;
    let mut cache = BTreeMap::new();
    calculate_maximum_total_flow(&graph, opened, &mut cache, 30, starting_node)
}

fn part_2(input: &str) -> u32 {
    let graph = extract_node_graph(input);
    let starting_node = get_node_index("AA", &graph).unwrap();
    let mut cache = BTreeMap::new();
    (0..=u16::MAX)
        .map(|opened| {
            calculate_maximum_total_flow(&graph, opened, &mut cache, 26, starting_node)
                + calculate_maximum_total_flow(
                    &graph,
                    u16::MAX ^ opened,
                    &mut cache,
                    26,
                    starting_node,
                )
        })
        .max()
        .unwrap()
}

fn calculate_maximum_total_flow(
    graph: &GraphType,
    opened: u16,
    cache: &mut BTreeMap<(u16, NodeIndex, u32), u32>,
    time_remaining: u32,
    current_node: NodeIndex,
) -> u32 {
    let cache_key = (opened, current_node, time_remaining);
    if let Some(value) = cache.get(&cache_key) {
        *value
    } else {
        let max = graph
            .neighbors_directed(current_node, Direction::Outgoing)
            .filter_map(|node| {
                let node_mask = 1 << node.index();
                if opened & node_mask != 0 {
                    None
                } else {
                    let distance = graph[graph.find_edge(current_node, node).unwrap()];
                    if distance < time_remaining {
                        Some((node, distance + 1))
                    } else {
                        None
                    }
                }
            })
            .fold(0, |max, (node, time_to_open)| {
                let opened = opened | 1 << node.index();
                let time_remaining = time_remaining - time_to_open;
                std::cmp::max(
                    max,
                    calculate_maximum_total_flow(graph, opened, cache, time_remaining, node)
                        + time_remaining * graph[node].1,
                )
            });
        cache.insert(cache_key, max);
        max
    }
}

fn extract_node_graph(input: &str) -> GraphType<'_> {
    let mut graph = GraphType::new();
    iter_parsed_lines(input).for_each(|(name, flow_rate, neighbor_names_iter)| {
        let node_index = get_or_add_node_index(name, &mut graph);
        graph[node_index] = (name, flow_rate);
        neighbor_names_iter.for_each(|neighbor_name| {
            let neighbor_node_index = get_or_add_node_index(neighbor_name, &mut graph);
            graph.add_edge(node_index, neighbor_node_index, 1);
        });
    });
    compress_node_graph(graph)
}

fn compress_node_graph(graph: GraphType) -> GraphType {
    let mut compressed_graph = GraphType::with_capacity(graph.node_count(), graph.edge_count());
    let graph = &graph; // we need to use move closure for the inner filter and we can't move from graph as it's being used. We can move a reference though.
    let start_node_index = get_node_index("AA", graph).unwrap();
    graph
        .node_indices()
        .filter(|index| *index == start_node_index || graph[*index].1 > 0)
        .flat_map(|from_index| {
            graph
                .node_indices()
                .filter(move |index| {
                    graph[*index].1 > 0 && *index != from_index && *index != start_node_index
                })
                .map(move |to_index| (from_index, to_index))
        })
        .for_each(|(from_index, to_index)| {
            if let Some((distance, _)) =
                astar(graph, from_index, |index| index == to_index, |_| 1, |_| 0)
            {
                let from_data = graph[from_index];
                let to_data = graph[to_index];
                let from_index = get_or_add_node_index(from_data.0, &mut compressed_graph);
                let to_index = get_or_add_node_index(to_data.0, &mut compressed_graph);
                compressed_graph[from_index] = from_data;
                compressed_graph[to_index] = to_data;
                compressed_graph.add_edge(from_index, to_index, distance);
            }
        });
    compressed_graph
}

fn get_node_index(name: &str, graph: &GraphType) -> Option<NodeIndex> {
    graph.node_indices().find(|index| graph[*index].0 == name)
}

fn get_or_add_node_index<'a>(name: &'a str, graph: &mut GraphType<'a>) -> NodeIndex {
    if let Some(node_index) = get_node_index(name, graph) {
        node_index
    } else {
        graph.add_node((name, 0))
    }
}

fn iter_parsed_lines(input: &str) -> impl Iterator<Item = (&str, u32, impl Iterator<Item = &str>)> {
    input.trim().lines().map(parse_line)
}

fn parse_line(input: &str) -> (&str, u32, impl Iterator<Item = &str>) {
    let mut splits = input.trim().split("; ");
    let (name, flow_rate) = extract_node_data(splits.next().unwrap());
    let neighbor_names_iter = extract_neighbor_node_names(splits.next().unwrap());
    (name, flow_rate, neighbor_names_iter)
}

fn extract_neighbor_node_names(input: &str) -> impl Iterator<Item = &str> {
    input
        .strip_prefix("tunnels lead to valves ")
        .or_else(|| input.strip_prefix("tunnel leads to valve "))
        .unwrap()
        .split(", ")
}

fn extract_node_data(input: &str) -> (&str, u32) {
    let mut splits = input.split('=');
    let name = splits
        .next()
        .unwrap()
        .strip_prefix("Valve ")
        .unwrap()
        .strip_suffix(" has flow rate")
        .unwrap();
    let rate = splits.next().unwrap().parse().unwrap();
    (name, rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        const INPUT: &str = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
        ";
        const EXPECTED: u32 = 1651;

        // Act
        let output = part_1(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }

    #[test]
    fn test_part_2() {
        // Arrange
        const INPUT: &str = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
        ";
        const EXPECTED: u32 = 1707;

        // Act
        let output = part_2(INPUT);

        // Assert
        assert_eq!(output, EXPECTED);
    }
}
