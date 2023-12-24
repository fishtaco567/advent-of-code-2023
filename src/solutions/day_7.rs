use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub fn solve(input: &str) {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        hands.push(line.into());
    }

    hands.sort();

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i as u32 + 1);
    }

    println!("{sum}");

    for hand in hands.iter_mut() {
        hand.swap_to_jokers();
    }

    hands.sort();

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i as u32 + 1);
    }

    println!("{sum}");
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: u32,
}

impl Hand {
    fn swap_to_jokers(&mut self) {
        for card in self.cards.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_cmp = self.hand_type().cmp(&other.hand_type());
        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }

        for i in 0..self.cards.len() {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            } else if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut map = HashMap::new();

        for card in &self.cards {
            map.entry(card)
                .and_modify(|x: &mut i32| *x = *x + 1)
                .or_insert(1);
        }

        let j = map.remove(&Card::Joker).unwrap_or(0);

        if map.len() == 1 || map.values().max().unwrap_or(&0) + j == 5 {
            return HandType::FiveOf;
        } else if (map.len() == 2 && map.values().contains(&4))
            || map.values().max().unwrap_or(&0) + j == 4
        {
            return HandType::FourOf;
        } else if map.len() == 2 {
            return HandType::FullHouse;
        } else if (map.len() == 3 && map.values().contains(&3))
            || map.values().max().unwrap() + j == 3
        {
            return HandType::ThreeOf;
        } else if (map.len() == 3 && map.values().filter(|x| **x == 2).count() == 2)
            || map.len() == 3
        {
            return HandType::TwoPair;
        } else if map.values().max().unwrap() + j >= 2 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bet) = value.split_once(' ').unwrap();
        Self {
            cards: cards.chars().map(|c| c.into()).collect(),
            bet: bet.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    T,
    Num(u32),
    Joker,
}

impl Card {
    fn val(&self) -> u32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::T => 10,
            Card::Num(x) => *x,
            Card::Joker => 1,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val().cmp(&other.val())
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::T,
            c @ '2'..='9' => Card::Num(c.to_digit(10).unwrap()),
            _ => panic!(),
        }
    }
}
