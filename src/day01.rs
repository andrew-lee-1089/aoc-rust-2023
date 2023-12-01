use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn calibration_value_part_one(input: String) -> i32 {
    let char_v: Vec<char> = input.chars().collect();
    let numerical_values: Vec<char> = char_v.into_iter().filter(|c| c.is_digit(10)).collect();
    let firstdigit: char = numerical_values[0];
    let lastdigit: char = numerical_values[numerical_values.len() - 1];
    let calibration_value_str = firstdigit.to_string() + &lastdigit.to_string();
    calibration_value_str.parse::<i32>().unwrap()
}

fn calibration_value_part_two(mut input: String) -> i32 {
    // We replace all instances of 'one' with 'one1one' etc.
    // So 'eightwothree' becomes 'eight8eightwo2twothree3three'
    // then use the same logic as used for part 1.
    let digit_names: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in 1..10 {
        input = input.replace(
            digit_names[i - 1],
            format!(
                "{}{}{}",
                digit_names[i - 1],
                i.to_string().as_str(),
                digit_names[i - 1]
            )
            .as_str(),
        )
    }
    calibration_value_part_one(input)
}

pub fn part1solve() -> i32 {
    let inputs = read_lines("inputs/day01.txt");
    inputs
        .into_iter()
        .map(|i| calibration_value_part_one(i))
        .sum()
}

pub fn part2solve() -> i32 {
    let inputs = read_lines("inputs/day01.txt");
    inputs
        .into_iter()
        .map(|i| calibration_value_part_two(i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(calibration_value_part_one("1abc2".to_string()), 12);
        assert_eq!(calibration_value_part_two("pqr3stu8vwx".to_string()), 38);
        assert_eq!(calibration_value_part_two("a1b2c3d4e5f".to_string()), 15);
        assert_eq!(calibration_value_part_two("treb7uchet".to_string()), 77);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(calibration_value_part_two("two1nine".to_string()), 29);
        assert_eq!(calibration_value_part_two("eightwothree".to_string()), 83);
        assert_eq!(
            calibration_value_part_two("abcone2threexyz".to_string()),
            13
        );
        assert_eq!(calibration_value_part_two("xtwone3four".to_string()), 24);
        assert_eq!(calibration_value_part_two("zoneight234".to_string()), 14);
    }
}
