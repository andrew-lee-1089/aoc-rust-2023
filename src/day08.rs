// TIL:
// Sigh, my code was looking elegant at the end of part 1, then added a part 2 solution I was pleased with and it was sloow
// Making my code work part 2 was horrid, and I'm still not fully convinced by my maths.

use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;

fn parse(lines: Vec<String>) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines_iter = lines.iter();
    let move_order = lines_iter.next().unwrap().chars().collect::<Vec<char>>();
    lines_iter.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let re: Regex =
        Regex::new(r"^(?<node>[\w]{3}) = \((?<leftHandMove>[\w]{3}), (?<rightHandMove>[\w]{3})\)")
            .unwrap();
    for row in lines_iter {
        //SDS = (RCP, RCP)

        let Some(caps) = re.captures(row) else {
            panic!("On the Streets Of London")
        };
        network.insert(
            caps["node"].to_string(),
            (
                caps["leftHandMove"].to_string(),
                caps["rightHandMove"].to_string(),
            ),
        );
    }
    (move_order, network)
}

pub fn part1solve() -> u32 {
    let inputs = read_lines("inputs/day08.txt");
    part1inner(inputs)
}

pub fn part2solve() -> u64 {
    let inputs = read_lines("inputs/day08.txt");
    part2inner(inputs)
}

fn get_next_node(
    network: &HashMap<String, (String, String)>,
    current_node: &String,
    next_move: char,
) -> String {
    let next_node: &str;
    if next_move == 'L' {
        next_node = &network.get(current_node).unwrap().0;
    } else {
        next_node = &network.get(current_node).unwrap().1;
    }
    next_node.to_string()
}

pub fn part1inner(inputs: Vec<String>) -> u32 {
    let (move_order, network) = parse(inputs);
    let mut current_node = "AAA".to_string();
    let mut current_count = 0;
    while current_node != "ZZZ" {
        let next_move = move_order[&current_count % move_order.len()];
        current_node = get_next_node(&network, &current_node, next_move);
        current_count += 1;
    }

    current_count as u32
}

fn make_simplified_network(
    network: &HashMap<String, (String, String)>,
    move_order: &Vec<char>,
) -> HashMap<String, String> {
    let starting_nodes = network
        .keys()
        .map(|node| node.to_owned())
        .collect::<Vec<String>>();
    let mut current_nodes = starting_nodes.clone();

    for i in 0..move_order.len() {
        let next_move = move_order[i];

        current_nodes = current_nodes
            .iter()
            .map(|node| get_next_node(&network, node, next_move))
            .collect::<Vec<String>>();
    }

    let mut simplified_network: HashMap<String, String> = HashMap::new();
    for i in 0..starting_nodes.len() {
        simplified_network.insert(starting_nodes[i].clone(), current_nodes[i].clone());
    }

    simplified_network
}

pub fn part2inner(inputs: Vec<String>) -> u64 {
    let (move_order, network) = parse(inputs);
    // After |move_order|turns node A goes to simplified_network[A]
    let simplified_network = make_simplified_network(&network, &move_order);

    let a_nodes = network
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .map(|node| node.to_owned())
        .collect::<Vec<String>>();

    let mut current_nodes = a_nodes.clone();
    let mut current_count = 0;

    let mut a_nodes_len_to_z: HashMap<String, u64> = HashMap::new();

    while a_nodes_len_to_z.keys().len() < a_nodes.len() {
        current_nodes = current_nodes
            .iter()
            .map(|node| simplified_network.get(node).unwrap().to_owned())
            .collect::<Vec<String>>();
        current_count += 1;

        for i in 0..a_nodes.len() {
            if current_nodes[i].ends_with("Z") && !a_nodes_len_to_z.contains_key(&a_nodes[i]) {
                a_nodes_len_to_z.insert(a_nodes[i].to_string(), current_count);
            }
        }
    }

    // Use Chinese remiander theorem to work out how long it will take. 
    a_nodes_len_to_z
        .values()
        .into_iter()
        .fold(1, |acc, no| num::integer::lcm(acc, *no))
        * move_order.len() as u64
}

mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        println!("Here I am");
        let test_input: Vec<String> = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part1inner(test_input), 2);
    }

    #[test]
    fn test_part_one_example_two() {
        let test_input: Vec<String> = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part1inner(test_input), 6);
    }

    #[test]
    fn test_part_two() {
        let test_input: Vec<String> = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part2inner(test_input), 6);
    }
}
