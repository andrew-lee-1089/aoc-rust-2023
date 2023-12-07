// TIL:
// On Part 1: More invovled that Day 6, easier than day 5
// 1) Used enums, derive / partialOrd / hash  impl Ord etc
// 2) ma\tch syntax, zip
// 3) HashMap
// On Part 2: I feel dirty and ashamed, making an Ord struct was kinda needless and there was fiddly logic there. 
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::utils::read_lines;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn part2_card_compare(a: &CardType, b: &CardType) -> Ordering {
    if a == &CardType::Jack && b == &CardType::Jack {
        return Ordering::Equal;
    } else if a == &CardType::Jack {
        println!("LHS is less");
        return Ordering::Less;
    } else if b == &CardType::Jack {
        println!("RHS is greater");
        return Ordering::Greater;
    } else {
        println!(
            "Based on compar
        
        ing {:?}, {:?}",
            a, b
        );
        return a.cmp(&b);
    }
}

fn part_2_hand_cmp(a: &Hand, b: &Hand) -> Ordering {
    println!(
        "Comapring {:?} {:?} to {:?} {:?}",
        a.cards, a.hand_type_pt2, b.cards, b.hand_type_pt2
    );
    if a.hand_type_pt2 < b.hand_type_pt2 {
        println!("LHS is less");
        return Ordering::Less;
    } else if a.hand_type_pt2 > b.hand_type_pt2 {
        println!("LHS is greater");
        return Ordering::Greater;
    }
    println!(
        "Same type of hand - looknig at cards: {:?} {:?} to {:?} {:?}",
        a.cards, a.hand_type_pt2, b.cards, b.hand_type_pt2
    );
    // Cards are of same type - look at the relative value of each card in turn
    let mut pairs = a
        .cards
        .iter()
        .zip(&b.cards)
        .collect::<Vec<(&CardType, &CardType)>>();
    while pairs.len() > 0 {
        let pair: (&CardType, &CardType) = pairs.remove(0);
        if part2_card_compare(pair.0, pair.1) != Ordering::Equal {
            return part2_card_compare(pair.0, pair.1);
        }
    }
    Ordering::Equal
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<CardType>,
    bet: u32,
    hand_type: HandType,
    hand_type_pt2: HandType,
}

impl Hand {
    fn from_str(line: &str) -> Hand {
        // line is of the form '5A8QA 674'
        println!("{}", line);
        let (cards_str, bet_str) = line.split_once(' ').unwrap();
        println!("{}", bet_str);
        let bet = bet_str.parse::<u32>().unwrap();
        let mut cards: Vec<CardType> = vec![];
        for char in cards_str.chars() {
            let card_type = match char {
                '2' => CardType::Two,
                '3' => CardType::Three,
                '4' => CardType::Four,
                '5' => CardType::Five,
                '6' => CardType::Six,
                '7' => CardType::Seven,
                '8' => CardType::Eight,
                '9' => CardType::Nine,
                'T' => CardType::Ten,
                'J' => CardType::Jack,
                'Q' => CardType::Queen,
                'K' => CardType::King,
                'A' => CardType::Ace,
                _ => panic!("At the disco"),
            };
            cards.push(card_type)
        }

        let mut myHashMap: HashMap<&CardType, i32> = HashMap::new();
        for card in cards.iter() {
            *myHashMap.entry(&card).or_insert(0) += 1;
        }
        let mut hand_type = HandType::HighCard;
        if myHashMap.values().any(|v| v == &5) {
            hand_type = HandType::FiveOfAKind;
        } else if myHashMap.values().any(|v| v == &4) {
            hand_type = HandType::FourOfAKind;
        } else if myHashMap.values().any(|v| v == &3) && myHashMap.values().any(|v| v == &2) {
            hand_type = HandType::FullHouse;
        } else if myHashMap.values().any(|v| v == &3) {
            hand_type = HandType::ThreeOfAKind;
        } else if myHashMap.values().filter(|v| v == &&2).count() == 2 {
            hand_type = HandType::TwoPair;
        } else if myHashMap.values().filter(|v| v == &&2).count() == 1 {
            hand_type = HandType::OnePair;
        }

        let mut myHashMap_2: HashMap<&CardType, i32> = HashMap::new();
        for card in cards.iter() {
            if card != &CardType::Jack {
                *myHashMap_2.entry(&card).or_insert(0) += 1;
            }
        }
        let no_jacks = cards.iter().filter(|c| c == &&CardType::Jack).count() as i32;

        let mut hand_type_pt2 = HandType::HighCard;
        if no_jacks == 5 || myHashMap_2.values().any(|v| v + no_jacks == 5) {
            hand_type_pt2 = HandType::FiveOfAKind;
        } else if myHashMap_2.values().any(|v| v + no_jacks == 4) {
            hand_type_pt2 = HandType::FourOfAKind;
        } else if no_jacks == 1 && myHashMap.values().filter(|v| v == &&2).count() == 2
            || myHashMap.values().any(|v| v == &3) && myHashMap.values().any(|v| v == &2)
        {
            hand_type_pt2 = HandType::FullHouse;
        } else if myHashMap_2.values().any(|v| v + no_jacks == 3) {
            hand_type_pt2 = HandType::ThreeOfAKind;
        } else if myHashMap.values().filter(|v| v == &&2).count() == 2 || ((myHashMap.values().filter(|v| v == &&2).count() == 1) && no_jacks == 1) {
            hand_type_pt2 = HandType::TwoPair;
        } else if no_jacks >= 1 || myHashMap.values().filter(|v| v == &&2).count() == 1 {
            hand_type_pt2 = HandType::OnePair;
        }

        Hand {
            cards,
            bet,
            hand_type,
            hand_type_pt2,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }
        // Cards are of same type - look at the relative value of each card in turn
        let mut pairs = self
            .cards
            .iter()
            .zip(&other.cards)
            .collect::<Vec<(&CardType, &CardType)>>();
        while pairs.len() > 0 {
            let pair: (&CardType, &CardType) = pairs.remove(0);
            if pair.0 < pair.1 {
                return Ordering::Less;
            } else if pair.0 > pair.1 {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn part1inner(inputs: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = inputs.iter().map(|line| Hand::from_str(line)).collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| ((idx + 1) as u32) * hand.bet)
        .sum()
}

pub fn part1solve() -> u32 {
    let inputs = read_lines("inputs/day07.txt");
    part1inner(&inputs)
}

fn part2inner(inputs: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = inputs.iter().map(|line| Hand::from_str(line)).collect();
    hands.sort_by(|a, b| part_2_hand_cmp(a, b));
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| ((idx + 1) as u32) * hand.bet)
        .sum()
}

pub fn part2solve() -> u32 {
    let inputs = read_lines("inputs/day07.txt");
    part2inner(&inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input: Vec<String> = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part1inner(&test_input), 6440);
    }

    #[test]
    fn test_part_two() {
        let test_input: Vec<String> = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(part2inner(&test_input), 5905);
    }
}
