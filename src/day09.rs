// TIL:
// Well that was completely easy with a basic recursive function.

use crate::utils::read_lines;

fn parse() -> Vec<Vec<i32>> {
    let inputs = read_lines("inputs/day09.txt");
    inputs
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|a| a.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn calculate_differences(row: &Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = vec![];
    for i in 1..row.len() {
        differences.push(row[i] - row[i - 1]);
    }
    differences
}

fn get_next_value(row: &Vec<i32>, previous: bool) -> i32 {
    if row.iter().all(|value| value == &0) {
        return 0;
    }
    let differences = calculate_differences(row);
    let next_difference = get_next_value(&differences, previous);
    if previous {
        return row.first().unwrap() - next_difference;
    }
    row.last().unwrap() + next_difference
}

pub fn part1solve() -> i32 {
    let input_rows = parse();
    input_rows
        .iter()
        .map(|row: &Vec<i32>| get_next_value(row, false))
        .sum()
}

pub fn part2solve() -> i32 {
    let input_rows = parse();
    input_rows
        .iter()
        .map(|row: &Vec<i32>| get_next_value(row, true))
        .sum()
}
