use crate::utils;
use std::{cmp::Ordering, collections::HashMap, vec};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut this_hand_strength = find_hand_strength(&self);
        let other_hand_strength = find_hand_strength(&other);
        if this_hand_strength == other_hand_strength {
            this_hand_strength += hand_diff(&self, &other);
        }
        return this_hand_strength.cmp(&other_hand_strength);
    }
    // ...
}

fn find_hand_strength(hand: &Hand) -> i32 {
    let mut found_map: HashMap<char, i32> = HashMap::new();
    for card in &hand.cards {
        let x = found_map.get_mut(&card);
        match x {
            Some(y) => {
                *y += 1;
            }
            None => {
                found_map.insert(*card, 1);
            }
        }
    }

    let mut found_vec: Vec<i32> = Vec::new();
    for (_key, value) in found_map {
        found_vec.push(value);
    }
    found_vec.sort();
    found_vec.reverse();

    match found_vec.as_slice() {
        [5] => return 7,
        [4, 1] => return 6,
        [3, 2] => return 5,
        [3, 1, 1] => return 4,
        [2, 2, 1] => return 3,
        [2, 1, 1, 1] => return 2,
        [1, 1, 1, 1, 1] => return 1,
        _ => return 0,
    }
}

fn hand_diff(hand1: &Hand, hand2: &Hand) -> i32 {
    let mut _diff: i32 = 0;
    let mut _diff_vec: Vec<char> = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    for i in 0..hand1.cards.len() {
        if hand1.cards[i] != hand2.cards[i] {
            return _diff_vec.iter().position(|&r| r == hand2.cards[i]).unwrap() as i32
                - _diff_vec.iter().position(|&r| r == hand1.cards[i]).unwrap() as i32;
        }
    }
    return _diff;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut hands: Vec<Hand> = Vec::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                let game = row.trim().split(' ').collect::<Vec<&str>>();
                let current_hand = Hand {
                    cards: game[0].chars().collect::<Vec<char>>(),
                    bid: game[1].parse::<i32>().unwrap(),
                };
                hands.push(current_hand);
            }
        }
    }
    hands.sort();
    let mut _sum: i32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        _sum += hand.bid * (i as i32 + 1);
    }
    println!("Sum: {}", _sum);
}
