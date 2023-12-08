#![feature(iter_advance_by)]

use std::fs;

use itertools::Itertools;

use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

/// A `None` means the card is a Joker
type WeakCardVersion = Option<Card>;

impl Card {
    #[inline]
    fn card_to_weak(self) -> WeakCardVersion {
        (self != Self::J).then_some(self)
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::N9),
            '8' => Ok(Self::N8),
            '7' => Ok(Self::N7),
            '6' => Ok(Self::N6),
            '5' => Ok(Self::N5),
            '4' => Ok(Self::N4),
            '3' => Ok(Self::N3),
            '2' => Ok(Self::N2),
            _ => bail!("invalid card given"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HightCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait GetHand {
    fn hand(self) -> Hand;
}

impl GetHand for [Card; 5] {
    fn hand(mut self) -> Hand {
        self.sort();

        // count the grouped cards
        let mut counts = self
            .iter()
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect_vec();

        // now we use the counts to identify the hand
        counts.sort_unstable();
        counts.reverse();

        match counts[..] {
            [5] => Hand::FiveOfAKind,
            [4, 1] => Hand::FourOfAKind,
            [3, 2] => Hand::FullHouse,
            [3, 1, 1] => Hand::ThreeOfAKind,
            [2, 2, 1] => Hand::TwoPairs,
            [2, 1, 1, 1] => Hand::OnePair,
            [1, 1, 1, 1, 1] => Hand::HightCard,
            _ => unreachable!(),
        }
    }
}

impl GetHand for [WeakCardVersion; 5] {
    fn hand(mut self) -> Hand {
        self.sort();

        // now we remove the jokers, that will allow us to compare only the remaining cards
        let mut counts = self
            .iter()
            .filter(|card| card.is_some())
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect_vec();

        counts.sort_unstable();
        counts.reverse();

        match counts[..] {
            [] | [_] => Hand::FiveOfAKind,
            [_, 1] => Hand::FourOfAKind,
            [_, 2] => Hand::FullHouse,
            [_, 1, 1] => Hand::ThreeOfAKind,
            [_, 2, 1] => Hand::TwoPairs,
            [_, 1, 1, 1] => Hand::OnePair,
            [1, 1, 1, 1, 1] => Hand::HightCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Entry {
    cards: [Card; 5],
    bid: u32,
}

impl Entry {
    pub fn new(cards: [Card; 5], bid: u32) -> Self {
        Self { cards, bid }
    }
}

fn parse(raw_data: &String) -> Result<Vec<Entry>> {
    raw_data
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').context("invalid input given")?;
            let bid: u32 = bid.parse().expect("invalid big value");
            let cards = cards.chars().map(Card::try_from).ok_collect_array()?;
            Ok(Entry::new(cards, bid))
        })
        .ok_collect_vec()
}

fn compute_part1(raw_data: &String) -> u32 {
    let data = parse(raw_data).unwrap();

    data.iter()
        .map(|entry| (entry.cards.hand(), entry.cards, entry.bid))
        .sorted()
        .enumerate()
        .map(|(idx, (_, _, bid))| bid * (idx as u32 + 1))
        .sum()
}

fn compute_part2(raw_data: &String) -> u32 {
    let data = parse(raw_data).unwrap();

    data.iter()
        .map(|entry| {
            let weak_cards = entry.cards.map(Card::card_to_weak);
            (weak_cards.hand(), weak_cards, entry.bid)
        })
        .sorted()
        .enumerate()
        .map(|(idx, (_, _, bid))| bid * (idx as u32 + 1))
        .sum()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day07/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<()> {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(compute_part1(&INPUT.to_string()), 6440);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(compute_part2(&INPUT.to_string()), 5905);
    Ok(())
}
