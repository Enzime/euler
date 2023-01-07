use core::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use euler_utils::Counter;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Rank {
    Number(u8),
    Letter(char),
}

impl Rank {
    fn from_value(value: usize) -> Self {
        use Rank::*;

        if value > 10 {
            return Letter("JQKA"[value - 11..value - 10].chars().next().unwrap());
        }

        Number(value as u8)
    }

    fn value(&self) -> usize {
        let letter_ordering = "JQKA".chars().enumerate().map(|(i, c)| (c, i)).collect::<HashMap<_, _>>();

        use Rank::*;

        match self {
            Number(x) => *x as usize,
            Letter(x) => 11 + letter_ordering[x],
        }
    }
}

impl Default for Rank {
    fn default() -> Self { Self::Number(0) }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum Category {
    NoCategory,
    HighCard(Rank),
    OnePair(Rank),
    TwoPairs(Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush,
    FullHouse(Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

impl Category {
    fn determine(&self, hand: &Hand) -> Option<Self> {
        use Category::*;
        use Rank::*;

        if hand.cards.len() == 0 && self != &NoCategory {
            return None;
        }

        let rank_counts = hand.cards.iter()
                                    .map(|(rank, _)| rank.value())
                                    .collect::<Counter<_>>()
                                    .sorted();

        let mut ranks_for_count = HashMap::new();

        for (rank, count) in &rank_counts {
            ranks_for_count.entry(*count).or_insert(vec![]).push(rank);
        }

        for i in 1..=5 {
            ranks_for_count.entry(i).or_insert(vec![]);
        }

        match self {
            NoCategory => {
                if hand.cards.len() == 0 {
                    return Some(NoCategory);
                }
            },
            HighCard(_) => {
                if let Some(value) = ranks_for_count[&1].iter().max() {
                    return Some(HighCard(Rank::from_value(**value)));
                }
            },
            OnePair(_) => {
                if ranks_for_count[&2].len() == 1 {
                    return Some(OnePair(Rank::from_value(*ranks_for_count[&2][0])));
                }
            },
            TwoPairs(_) => {
                if rank_counts[0].1 == 2 && rank_counts[1].1 == 2 {
                    // the larger pair will be sorted second
                    return Some(TwoPairs(Rank::from_value(rank_counts[1].0)));
                }
            },
            ThreeOfAKind(_) => {
                if rank_counts[0].1 == 3 {
                    return Some(ThreeOfAKind(Rank::from_value(rank_counts[0].0)));
                }
            },
            Straight(_) => {
                let consecutive_ranks = hand.cards.iter()
                                                  .map(|(rank, _)| rank.value())
                                                  .enumerate()
                                                  .map(|(i, value)| value - i)
                                                  .fold(true, |acc, x| acc && x == hand.cards[0].0.value());

                if consecutive_ranks {
                    return Some(Straight(hand.cards[0].0));
                }
            },
            Flush => {
                let suits = hand.cards.iter().map(|(_, suit)| suit).collect::<Counter<_>>();

                if suits.len() == 1 {
                    return Some(Flush);
                }
            },
            FullHouse(_) => {
                if OnePair(Default::default()).determine(hand).is_some() {
                    if let Some(ThreeOfAKind(rank)) = ThreeOfAKind(Default::default()).determine(hand) {
                        return Some(FullHouse(rank));
                    }
                }
            },
            FourOfAKind(_) => {
                if rank_counts[0].1 == 4 {
                    return Some(FourOfAKind(Rank::from_value(rank_counts[0].0)));
                }
            },
            StraightFlush(_) => {
                if Flush.determine(hand).is_some() {
                    if let Some(Straight(rank)) = Straight(Default::default()).determine(hand) {
                        return Some(StraightFlush(rank));
                    }
                }
            },
            RoyalFlush => {
                if let Some(StraightFlush(Number(10))) = StraightFlush(Default::default()).determine(hand) {
                    return Some(RoyalFlush);
                }
            }
        };

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<(Rank, char)>,
    categories: Vec<Category>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: vec![], categories: vec![Category::NoCategory] }
    }

    fn from(string: &str) -> Self {
        let mut hand = Self::new();
        let cards = string.split(' ').collect::<Vec<_>>();

        use Rank::*;

        for card in cards {
            let card = card.chars().collect::<Vec<_>>();

            let rank = match card[0].to_digit(10) {
                Some(x) => Number(x as u8),
                None => if card[0] == 'T' { Number(10) } else { Letter(card[0]) },
            };

            hand.cards.push((rank, card[1]));
        }

        hand.cards.sort();

        for i in 0..hand.cards.len() - 1 {
            if hand.cards[i] == hand.cards[i + 1] {
                panic!("Duplicate card found!");
            }
        }

        hand.determine_categories();

        hand
    }

    fn determine_categories(&mut self) {
        self.categories.clear();

        for category in Category::iter().rev() {
            match category.determine(self) {
                Some(x) => self.categories.push(x),
                None => (),
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.categories.cmp(&other.categories)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // don't mind me, just making sure the types I wrote all work :)
    assert_eq!(Hand::from("5H 5C 6S 7S KD") > Hand::from("2C 3S 8S 8D TD"), false);
    assert_eq!(Hand::from("5D 8C 9S JS AC") > Hand::from("2C 5C 7D 8S QH"), true);
    assert_eq!(Hand::from("2D 9C AS AH AC") > Hand::from("3D 6D 7D TD QD"), false);
    assert_eq!(Hand::from("4D 6S 9H QH QC") > Hand::from("3D 6D 7H QD QS"), true);
    assert_eq!(Hand::from("2H 2D 4C 4D 4S") > Hand::from("3C 3D 3S 9S 9D"), true);
    assert_eq!(Hand::from("5H 5C 6S 7S KD") == Hand::from("5H 5C 6S 7S KD"), true);
    assert_eq!(Rank::Number(5) > Rank::Number(4), true);
    assert_eq!(Rank::Number(5) > Rank::Number(5), false);
    assert_eq!(Rank::Number(5) > Rank::Number(6), false);
    assert_eq!(Rank::Letter('Q') > Rank::Number(6), true);
    assert_eq!(Rank::Letter('K') < Rank::Number(6), false);
    assert_eq!(Rank::Letter('K') > Rank::Letter('Q'), true);
    assert_eq!(Rank::Letter('A') > Rank::Letter('K'), true);
    assert_eq!(Rank::Letter('A') > Rank::Letter('Q'), true);
    assert_eq!(Rank::Letter('A') < Rank::Letter('Q'), false);
    assert_eq!(Rank::Number(10) < Rank::Letter('J'), true);
    assert_eq!(Rank::Number(10) > Rank::Letter('J'), false);
    assert_eq!(Rank::Number(10) == Rank::Letter('J'), false);
    assert_eq!(Rank::Number(10) == Rank::Number(10), true);
    assert_eq!(Rank::Letter('J') == Rank::Letter('J'), true);
    assert_eq!(Rank::Letter('J') == Rank::Letter('K'), false);

    let hands = fs::read_to_string("poker.txt").unwrap().trim().to_string();

    let mut player_one_wins = 0;

    for line in hands.split('\n') {
        let player_one = Hand::from(&line[..14]);
        let player_two = Hand::from(&line[15..]);

        if player_one > player_two {
            player_one_wins += 1;
        }
    }

    println!("{}", player_one_wins);
}
