// TIL:
// I found that really difficult, especially part 2.
//  1) Using .nth() on Chars (my_string.chars().nth())
//  2) Proper was to index into a Split is to cast to Vec
//  3) Using find and deaing with Option / Some
//  4) Range struct

use std::ops::Range;

use crate::utils::read_lines;

#[derive(Debug)]
struct Rule {
    input_range: std::ops::Range<i64>,
    modifier: i64,
}

struct Converter {
    rules: Vec<Rule>,
}

impl Converter {
    fn from_list(rules_strings: Vec<&String>) -> Converter {
        // A B C
        // means B->A and for all i in 0..C B+i -> A+i
        let mut rules: Vec<Rule> = vec![];
        for row in rules_strings {
            let row_split: Vec<i64> = row
                .split(' ')
                .map(|split_part| split_part.parse::<i64>().unwrap())
                .collect();
            let a: i64 = row_split[0];
            let b: i64 = row_split[1];
            let c: i64 = row_split[2];
            rules.push(Rule {
                input_range: std::ops::Range {
                    start: b,
                    end: b + c,
                },
                modifier: a - b,
            });
        }
        Converter { rules }
    }

    fn apply_rules(&self, input: i64) -> i64 {
        if let Some(applicable_rule) = self
            .rules
            .iter()
            .find(|rule| rule.input_range.contains(&input))
        {
            return input + applicable_rule.modifier;
        }
        input
    }

    fn apply_rules_to_ranges(&self, mut ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut ranges_to_return: Vec<Range<i64>> = vec![];
        while ranges.len() > 0 {
            let range = ranges.pop().unwrap();
            let mut rules_match: bool = false;

            for rule in self.rules.iter() {
                if (rule.input_range.start <= range.start && rule.input_range.end >= range.end) {
                    ranges_to_return.push(Range {
                        start: range.start + rule.modifier,
                        end: range.end + rule.modifier,
                    });
                    rules_match = true;
                    break
                } else if (rule.input_range.start <= range.start
                    && rule.input_range.end > range.start)
                {
                    ranges_to_return.push(Range {
                        start: range.start + rule.modifier,
                        end: rule.input_range.end + rule.modifier,
                    });
                    ranges.push(Range {
                        start: rule.input_range.end,
                        end: range.end,
                    });
                    rules_match = true;
                } else if (rule.input_range.start < range.end && rule.input_range.end >= range.end)
                {
                    ranges.push(Range {
                        start: range.start,
                        end: rule.input_range.start,
                    });
                    ranges_to_return.push(Range {
                        start: rule.input_range.start + rule.modifier,
                        end: range.end + rule.modifier,
                    });
                    rules_match = true;
                }
            }
            if !rules_match {
                ranges_to_return.push(range);
            }
        }
        ranges_to_return
    }
}

pub fn part1solve() -> u32 {
    let inputs = read_lines("inputs/day05.txt");
    part1solveinner(&inputs)
}

fn part1solveinner(inputs: &Vec<String>) -> u32 {
    let seeds: Vec<i64> = inputs[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut converters: Vec<Converter> = vec![];
    let mut rows_for_converter: Vec<&String> = vec![];
    for (idx, row) in inputs.iter().enumerate().filter(|(idx, _)| idx > &2) {
        if row.len() == 0 {
            converters.push(Converter::from_list(rows_for_converter));
            rows_for_converter = vec![];
            continue;
        }
        if row.chars().nth(0).unwrap().is_ascii_digit() {
            rows_for_converter.push(row)
        }
    }
    converters.push(Converter::from_list(rows_for_converter));
    seeds
        .iter()
        .map(|seed| {
            converters
                .iter()
                .fold(seed.clone(), |acc, converter| converter.apply_rules(acc))
        })
        .min()
        .unwrap() as u32
}

pub fn part2solve() -> u32 {
    let inputs = read_lines("inputs/day05.txt");
    part2solveinner(&inputs)
}

fn part2solveinner(inputs: &Vec<String>) -> u32 {
    let seeds_list: Vec<i64> = inputs[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut seed_ranges: Vec<Range<i64>> = vec![];
    for i in 0..seeds_list.len() / 2 {
        let start = seeds_list[2 * i];
        let length = seeds_list[(2 * i) + 1];
        seed_ranges.push(Range {
            start: start,
            end: start + length,
        });
    }

    let mut converters: Vec<Converter> = vec![];
    let mut rows_for_converter: Vec<&String> = vec![];
    for (idx, row) in inputs.iter().enumerate().filter(|(idx, _)| idx > &2) {
        if row.len() == 0 {
            converters.push(Converter::from_list(rows_for_converter));
            rows_for_converter = vec![];
            continue;
        }
        if row.chars().nth(0).unwrap().is_ascii_digit() {
            rows_for_converter.push(row)
        }
    }
    converters.push(Converter::from_list(rows_for_converter));

    for converter in converters {
        seed_ranges = converter.apply_rules_to_ranges(seed_ranges);
    }
    seed_ranges.iter().map(|r| r.start).min().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input: Vec<String> = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        assert_eq!(part1solveinner(&test_input), 35);
        assert_eq!(part2solveinner(&test_input), 46);
    }
}
