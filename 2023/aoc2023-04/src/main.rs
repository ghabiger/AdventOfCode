use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Clone)]
struct Card {
    n: usize,
    winners: Vec<usize>,
    draw: Vec<usize>,
    matches: Vec<usize>,
    value: usize,
}

fn line_to_card(line: &str, winners_length: usize) -> Card {
    let re = Regex::new(r"(\d+)").unwrap();
    let numbers: Vec<usize> = re
        .captures_iter(&line)
        .map(|caps| {
            let (_, [num]) = caps.extract();
            num.parse().unwrap()
        })
        .collect();

    let winners = numbers[1..winners_length + 1].to_vec();
    let draw = numbers[winners_length + 1..].to_vec();
    let matches = find_matches(&winners, &draw);
    let value = match matches.len() {
        0 => 0,
        _ => 2_usize.pow((matches.len() - 1).try_into().unwrap()),
    };

    Card {
        n: numbers[0],
        winners,
        draw,
        matches,
        value,
    }
}

fn find_matches(winners: &Vec<usize>, draw: &Vec<usize>) -> Vec<usize> {
    let mut matches: Vec<usize> = Vec::new();

    for n in draw {
        if winners.contains(n) {
            matches.push(*n);
        }
    }

    matches
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File should be readable");

    let mut card_value_sum = 0;
    let mut original_cards: Vec<Card> = Vec::new();
    let mut card_copy_counts: Vec<usize> = Vec::new();
    for line in input.lines() {
        let card = line_to_card(line, 10);
        original_cards.push(card.clone());
        card_copy_counts.push(1);
        card_value_sum += card.value;
    }

    // iterate through all original cards
    for cur_original_index in 0..original_cards.len() - 1 {
        let num_of_won_cards = original_cards[cur_original_index].matches.len();
        if num_of_won_cards > 0 {
            // We've won cards. Let's add their count to the tallying hashmap
            // add exactly as many cards as we have copies times the number of matches
            let num_of_copies = card_copy_counts[cur_original_index];
            for index_relative in 1..num_of_won_cards + 1 {
                if cur_original_index + index_relative < original_cards.len() {
                    card_copy_counts[cur_original_index + index_relative] += num_of_copies;
                }
            }
        }
    }

    println!("Sum of values of all cards: {}", card_value_sum);
    println!(
        "Sum of all cards: {}",
        card_copy_counts.into_iter().sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n");

        let mut num_matches: Vec<usize> = Vec::new();
        let mut values: Vec<usize> = Vec::new();
        for line in input.lines() {
            println!("{line}");
            let card = line_to_card(line, 5);
            println!(
                "Card {} with winners {:?} and draws {:?} has value {} with {} matches: {:?}",
                card.n,
                card.winners,
                card.draw,
                card.value,
                card.matches.len(),
                card.matches
            );
            num_matches.push(card.matches.len());
            values.push(card.value);
        }

        assert_eq!(num_matches, vec![4, 2, 2, 1, 0, 0]);
        assert_eq!(values, vec![8, 2, 2, 1, 0, 0]);
    }
}
