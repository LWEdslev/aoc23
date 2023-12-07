use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Joker
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Joker),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            _ => None
        }
    }

    fn from_string(s: &str) -> Vec<Card> {
        assert!(s.len() == 5);
        let mut chars = s.chars();
        (0..5).map(|_| Card::from_char(chars.next().unwrap()).unwrap()).collect()
    }

    fn value(&self) -> u64 {
        match self {
            Card::Joker => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }

    fn vec_to_value(cards: &Vec<Card>) -> u64 {
        let mut res = 0;
        for card in cards {
            res *= 100;
            res += card.value();
        }
        res
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Score {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Score {
    fn calculate(cards: &Vec<Card>) -> Self {
        let mut map = HashMap::new();
        for card in cards {
            if !map.contains_key(card) { map.insert(card.clone(), 0); }
            *map.get_mut(card).unwrap() += 1;
        }

        let mut jokers = 0;
        let mut pairs = 0;
        let mut triple = false;
        let mut four = false;
        let mut five = false;
        for (card, count) in map.iter() {
            if card == &Card::Joker {
                jokers = *count;
                break;
            }
        }

        for (card, count) in map.iter() {
            if card == &Card::Joker { continue }
            if *count == 2 { pairs += 1 }
            if *count == 3 { triple = true }
            if *count == 4 { four = true }
            if *count == 5 { five = true }
        }       

        if five || (four && jokers == 1) || (triple && jokers == 2) || (pairs == 1 && jokers == 3) || jokers > 3 { return Self::FiveOfAKind }
        if four || (triple && jokers == 1) || (pairs == 1 && jokers == 2) || jokers == 3  { return Self::FourOfAKind }
        if (triple && pairs == 1) || (pairs == 2 && jokers == 1) { return Self::FullHouse }
        if triple || (pairs == 1 && jokers == 1) || (jokers == 2)  { return Self::ThreeOfAKind }
        if pairs == 2 || (jokers == 1 && pairs == 1) { return Self::TwoPair }
        if pairs == 1 || (jokers == 1) { return Self::OnePair }
        return Self::HighCard
    }
    fn value(&self) -> u64 {
        use Score::*;
        match self {
            FiveOfAKind => 7,
            FourOfAKind => 6,
            FullHouse => 5,
            ThreeOfAKind => 4,
            TwoPair => 3,
            OnePair => 2,
            HighCard => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    score: Score,
}

fn main() {
    let file = std::fs::read_to_string("aoc06.txt").unwrap();
    let mut hands = file.lines().map(|line| {
        let mut split = line.trim().split_ascii_whitespace();
        let cards = Card::from_string(split.next().unwrap());
        let bid = split.next().unwrap().parse().unwrap();
        let score = Score::calculate(&cards);
        Hand {
            cards,
            bid,
            score
        }
    }).collect::<Vec<_>>();

    hands.sort_by(|a, b| Card::vec_to_value(&a.cards).cmp(&Card::vec_to_value(&b.cards)));
    hands.sort_by(|a, b| a.score.value().cmp(&b.score.value()));

    let mut res = 0;
    for i in 1..hands.len()+1 {
        let hand = hands.get(i-1).unwrap();
        println!("Val: {} * {} = {}", i, hand.bid, (i as u64) * hand.bid);
        res += (i as u64) * hand.bid;
    }

    println!("{res}")
}