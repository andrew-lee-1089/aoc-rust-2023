use regex::Regex;
use std::fs::read_to_string;

struct CubeSet {
    blue: i32,
    red: i32,
    green: i32,
}

impl CubeSet {
    fn is_part_one_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn parse_set(line: &str) -> CubeSet {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        let words: Vec<&str> = line.split(" ").collect();
        for (idx, word) in words.iter().enumerate() {
            if word.contains("red") {
                red = words[idx - 1].parse::<i32>().unwrap()
            }
            if word.contains("blue") {
                blue = words[idx - 1].parse::<i32>().unwrap()
            }
            if word.contains("green") {
                green = words[idx - 1].parse::<i32>().unwrap()
            }
        }
        CubeSet { blue, red, green }
    }
}

struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn is_part_one_possible(&self) -> bool {
        self.sets.iter().all(|s| s.is_part_one_possible())
    }

    fn minimum_possible_cubes(&self) -> i32 {
        self.sets.iter().map(|s| s.red).max().unwrap()
            * self.sets.iter().map(|s| s.blue).max().unwrap()
            * self.sets.iter().map(|s| s.green).max().unwrap()
    }

    fn parse_line(line: &str) -> Game {
        let re: Regex = Regex::new(r"^Game (?<gameId>[0-9]+):(?<setData>.*)").unwrap();
        let Some(caps) = re.captures(line) else {
            panic!()
        };
        let id = caps["gameId"].parse::<i32>().unwrap();
        let sets: Vec<CubeSet> = caps["setData"]
            .split(";")
            .into_iter()
            .map(|line| CubeSet::parse_set(line))
            .collect();

        Game { id, sets }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn part1solve() -> i32 {
    let inputs = read_lines("inputs/day02.txt");
    inputs
        .into_iter()
        .map(|i| Game::parse_line(&i))
        .filter(|game| game.is_part_one_possible())
        .map(|game| game.id)
        .sum()
}

pub fn part2solve() -> i32 {
    let inputs = read_lines("inputs/day02.txt");
    inputs
        .into_iter()
        .map(|i| Game::parse_line(&i).minimum_possible_cubes())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            Game::parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .is_part_one_possible(),
            true
        );
        assert_eq!(
            Game::parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .is_part_one_possible(),
            true
        );
        assert_eq!(
            Game::parse_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
            .is_part_one_possible(),
            false
        );
        assert_eq!(
            Game::parse_line(
                "Game 100: 2 blue, 12 green; 6 green, 1 red, 12 blue; 1 green, 5 blue, 1 red; 1 red, 12 green, 6 blue; 16 blue, 3 green"
            )
            .is_part_one_possible(),
            false
        );
        assert_eq!(
            Game::parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
                .is_part_one_possible(),
            true
        );
    }
}
