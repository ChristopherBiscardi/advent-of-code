use std::ops::Deref;

use itertools::Itertools;
use miette::NamedSource;

use crate::custom_error::{AocError, CardFromCharError};

#[derive(
    Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq,
)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy,
)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}
impl TryFrom<char> for Card {
    type Error = CardFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Card::*;
        match value {
            'A' => Ok(A),
            'K' => Ok(K),
            'Q' => Ok(Q),
            'J' => Ok(J),
            'T' => Ok(T),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            invalid_char => {
                Err(CardFromCharError::InvalidCharacter(
                    invalid_char,
                ))
            }
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct HandScore {
    hand: HandType,
    cards: [Card; 5],
}
fn score_hand(
    hand: &str,
) -> Result<HandScore, CardFromCharError> {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!(
            "should never happen. Encountered `{}`",
            value
        ),
    };
    let cards: Vec<Card> = hand
        .chars()
        .map(TryFrom::try_from)
        .try_collect()?;
    let mut card_arr: [Card; 5] = [Card::A; 5];
    card_arr.iter_mut().set_from(cards);

    Ok(HandScore {
        hand: hand_type,
        cards: card_arr,
    })
}

#[derive(Debug)]
struct Hand<'a> {
    #[allow(dead_code)]
    hand: &'a str,
    bid: u32,
    score: HandScore,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let hands = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            let (hand, bid) = line
                .split_once(' ')
                .ok_or_else(|| AocError::SplitError {
                    src: NamedSource::new(
                        "input1.txt",
                        input.to_string(),
                    ),
                    bad_bit: (line_number, 0usize).into(),
                })?;
            Ok(Hand {
                hand,
                bid: bid.parse::<u32>()?,
                score: score_hand(hand)?,
            })
        })
        .collect::<Result<Vec<Hand>, AocError>>()?;

    let hands = hands
        .into_iter()
        .sorted_by_key(|x| (x.score.hand, x.score.cards))
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum::<u32>();
    Ok(hands.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
