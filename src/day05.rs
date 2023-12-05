// TIL:
//  1) Using .nth() on Chars (my_string.chars().nth())
//  2) Proper was to index into a Split is to cast to Vec
//  3) Using find and deaing with Option / Some
//  4) Range struct

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
        println!("{}", rules_strings.len());

        for row in rules_strings {
            let row_split: Vec<i64> = row
                .split(' ')
                .map(|split_part| split_part.parse::<i64>().unwrap())
                .collect();
            let a: i64 = row_split[0];
            let b: i64 = row_split[1];
            let c: i64 = row_split[2];
            println!("{} {} {}", a, b, c);
            rules.push(Rule {
                input_range: std::ops::Range { start: b, end: b+c },
                modifier: a - b,
            });
        }
        Converter { rules }
    }

    fn apply_rules(&self, input: i64) -> i64 {
        println!("Applying rules for input {}, rules: {:?}", input, self.rules);
        if let Some(applicable_rule) = self
            .rules
            .iter()
            .find(|rule| rule.input_range.contains(&input))
        {
            return input + applicable_rule.modifier;
        }
        input
    }
}

fn parse(input: Vec<String>) -> (Vec<i64>, Vec<Converter>) {
    let seeds: Vec<i64> = input[0]
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut converters: Vec<Converter> = vec![];
    let mut rows_for_converter: Vec<&String> = vec![];
    for (idx, row) in input.iter().enumerate().filter(|(idx, _)| idx > &2) {
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
    (seeds, converters)
}

pub fn part1solve() -> u32 {
    let inputs = read_lines("inputs/day05.txt");
    part1solveinner(inputs)
}

fn part1solveinner(inputs: Vec<String>) -> u32 {
    let (seeds, converters): (Vec<i64>, Vec<Converter>) = parse(inputs);
    println!("bhjkjhgf {}", converters.len());
    seeds
        .iter()
        .map(|seed| {
            converters
                .iter()
                .fold(seed.clone(), |acc, converter| { println!("seed {} {}", seed, acc); converter.apply_rules(acc) })
        })
        .min()
        .unwrap() as u32
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

        assert_eq!(part1solveinner(test_input), 35);
    }
}
