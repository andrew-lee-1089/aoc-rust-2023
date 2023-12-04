use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

struct Game {
    card_id: u8,
    winning_nos: HashSet<u8>,
    scratchcard_nos: HashSet<u8>,
}

impl Game {
    fn scratchcard_value(&self) -> u32 {
        let count = self.winning_nos.intersection(&self.scratchcard_nos).count() as u32;
        if count == 0 {
            return 0;
        }
        u32::pow(2, count - 1)
    }

    fn part_two_count(&self) -> u32 {
        self.winning_nos.intersection(&self.scratchcard_nos).count() as u32
    }

    fn from_str(line: &str) -> Game {
        let line_regex = Regex::new(
            r"Card +(?P<card_id>\d+): (?P<winning_nos>[\d ]+)+ \|(?P<scratchcard_nos>[\d ]+)+$",
        )
        .unwrap();
        let Some(caps) = line_regex.captures(line) else {
            panic!("On the Streets Of London")
        };
        let card_id = caps["card_id"].parse::<u8>().unwrap();
        let winning_nos: HashSet<u8> = HashSet::from_iter(
            caps["winning_nos"]
                .split_whitespace()
                .map(|a| a.parse::<u8>().unwrap()),
        );
        let scratchcard_nos: HashSet<u8> = HashSet::from_iter(
            caps["scratchcard_nos"]
                .split_whitespace()
                .map(|a| a.parse::<u8>().unwrap()),
        );
        Game {
            card_id,
            winning_nos,
            scratchcard_nos,
        }
    }
}

fn parse() -> Vec<Game> {
    let inputs = read_lines("inputs/day04.txt");
    inputs.iter().map(|line| Game::from_str(line)).collect()
}

pub fn part1solve() -> u32 {
    let games = parse();
    games
        .iter()
        .map(|machine_part| machine_part.scratchcard_value())
        .sum()
}

pub fn part2solve() -> u32 {
    let games = parse();
    let mut copies_of_card_id: Vec<u32> = games.iter().map(|g| 1).collect();
    for i in 0..games.len() {
        for j in 1..=games[i].part_two_count() {
            copies_of_card_id[&i + j as usize] += copies_of_card_id[i];
        }
    }
    copies_of_card_id.iter().sum()
}
