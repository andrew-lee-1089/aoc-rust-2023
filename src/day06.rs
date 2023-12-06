// TIL:
// Well that was exceptionally easy comapred to yesterdat, did it in 15 mins, compared to 2 hours
// 1) Used zip, and fold

use crate::utils::read_lines;

struct Game {
    time: u64,
    distance_to_beat: u64,
}

impl Game {
    fn winning_count(&self) -> u64 {
        (0..self.time)
            .into_iter()
            .map(|t| t * (self.time - t))
            .filter(|distance| distance > &self.distance_to_beat)
            .count() as u64
    }
}

fn parse() -> Vec<Game> {
    let inputs = read_lines("inputs/day06.txt");

    let times = inputs[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances = inputs[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Game {
            time: *time,
            distance_to_beat: *distance,
        })
        .collect::<Vec<Game>>()
}

fn parse_part2() -> Game {
    let inputs = read_lines("inputs/day06.txt");

    let time = inputs[0]
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = inputs[1]
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    Game {
        time,
        distance_to_beat: distance,
    }
}

pub fn part1solve() -> u64 {
    let games = parse();
    games.iter().fold(1, |acc, game| acc * game.winning_count())
}

pub fn part2solve() -> u64 {
    let game = parse_part2();
    game.winning_count()
}
