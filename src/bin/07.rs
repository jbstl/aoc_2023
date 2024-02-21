use std::{char, cmp::Ordering};

advent_of_code::solution!(7);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardLabel {
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
use anyhow::{anyhow, Ok, Result};
use CardLabel::*;

impl TryFrom<char> for CardLabel {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            'T' => Ok(Ten),
            'J' => Ok(Jack),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            'A' => Ok(Ace),
            _ => Err(anyhow!("Unable to parse card label from char: {}", value)),
        }
    }
}

#[derive(Eq)]
struct CardHand {
    cards: [CardLabel; 5],
    bid: u32,
    joker_wild: bool,
}

impl CardHand {
    fn get_cards_repr(&self) -> [u8; 13] {
        let mut cards = [0; 13];
        for card in &self.cards {
            match card {
                Two => cards[0] += 1,
                Three => cards[1] += 1,
                Four => cards[2] += 1,
                Five => cards[3] += 1,
                Six => cards[4] += 1,
                Seven => cards[5] += 1,
                Eight => cards[6] += 1,
                Nine => cards[7] += 1,
                Ten => cards[8] += 1,
                Jack => cards[9] += 1,
                Queen => cards[10] += 1,
                King => cards[11] += 1,
                Ace => cards[12] += 1,
            }
        }
        if self.joker_wild {
            // add the joker count to the highest card number in order to improve the hand
            let joker_count = cards[9];
            if joker_count > 0 {
                let mut max_index = 0;
                let mut max = 0;
                for (index, card_count) in cards.iter().enumerate() {
                    if index == 9 {
                        continue;
                    }
                    if card_count > &max {
                        max_index = index;
                        max = *card_count;
                    }
                }
                cards[max_index] += joker_count;
                cards[9] = 0;
            }
        }
        cards
    }

    fn get_card_count_repr(&self) -> [u8; 5] {
        let mut repr = [0; 5];
        let card_repr = self.get_cards_repr();
        for card_count in card_repr {
            if card_count > 0 {
                repr[card_count as usize - 1] += 1;
            }
        }
        repr
    }

    fn get_hand_type(&self) -> HandType {
        let card_count_repr = self.get_card_count_repr();
        if card_count_repr[4] == 1 {
            HandType::FiveOfAKind
        } else if card_count_repr[3] == 1 {
            HandType::FourOfAKind
        } else if card_count_repr[2] == 1 && card_count_repr[1] == 1 {
            HandType::FullHouse
        } else if card_count_repr[2] == 1 {
            HandType::ThreeOfAKind
        } else if card_count_repr[1] == 2 {
            HandType::TwoPair
        } else if card_count_repr[1] == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_cmp = self.get_hand_type().cmp(&other.get_hand_type());
        if let Ordering::Equal = hand_cmp {
            // compare the rank of each label
            let mut order = None;
            for i in 0..5 {
                let self_card = self.cards[i];
                let other_card = other.cards[i];
                if self_card != other_card {
                    if self.joker_wild {
                        if let CardLabel::Jack = self_card {
                            order = Some(Ordering::Less);
                        } else if let CardLabel::Jack = other_card {
                            order = Some(Ordering::Greater)
                        } else {
                            order = Some(self_card.cmp(&other_card));
                        }
                    } else {
                        order = Some(self_card.cmp(&other_card));
                    }
                    break;
                }
            }
            order.unwrap()
        } else {
            hand_cmp
        }
    }
}

impl TryFrom<&str> for CardHand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut sections = value.split_whitespace();

        let hand = sections.next().unwrap();
        let hand_len = hand.len();
        if hand_len == 5 {
            let mut cards: [CardLabel; 5] = [CardLabel::Two; 5];
            for (index, c) in hand.chars().enumerate() {
                cards[index] = c.try_into()?;
            }
            let bid = sections.next().unwrap().parse().unwrap();
            let card_hand = CardHand {
                cards,
                bid,
                joker_wild: false,
            };

            Ok(card_hand)
        } else {
            Err(anyhow!(
                "CardHand must contain 5 values. Current hand has {}",
                hand_len
            ))
        }
    }
}

fn parse_input(input: &str) -> Vec<CardHand> {
    let results = input.lines().map(|line| line.try_into().unwrap()).collect();
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse_input(input);
    hands.sort();
    let (_, total_winnings) = hands.iter().fold((1, 0), |(mut acc, mut total), hand| {
        total += acc * hand.bid;
        acc += 1;
        (acc, total)
    });
    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse_input(input);
    for hand in hands.iter_mut() {
        hand.joker_wild = true;
    }
    hands.sort();
    let (_, total_winnings) = hands.iter().fold((1, 0), |(mut acc, mut total), hand| {
        total += acc * hand.bid;
        acc += 1;
        (acc, total)
    });
    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
