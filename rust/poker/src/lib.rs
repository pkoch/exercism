// I wish I could use crates. :(

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Undecodable(pub String);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    AceLow,
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

impl TryFrom<&str> for Rank {
    type Error = Undecodable;

    /// ```
    /// # use poker::*;
    /// assert_eq!(Rank::try_from("2"), Ok(Rank::Two));
    /// assert_eq!(Rank::try_from("10"), Ok(Rank::Ten));
    /// assert_eq!(Rank::try_from("K"), Ok(Rank::King));
    /// assert_eq!(Rank::try_from("?"), Err(Undecodable("\"?\": not a Rank".to_string())));
    /// ```
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(Undecodable(format!("{:?}: not a Rank", s))),
        }
    }
}

impl fmt::Display for Rank {
    /// ```
    /// # use poker::*;
    /// assert_eq!(Rank::try_from("A").unwrap().to_string(), "A");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
                Rank::AceLow => "A",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    Clubs = 'C' as u8,
    Diamonds = 'D' as u8,
    Hearts = 'H' as u8,
    Spades = 'S' as u8,
}

impl TryFrom<&char> for Suit {
    type Error = Undecodable;

    /// ```
    /// # use poker::*;
    /// assert_eq!(Suit::try_from(&'D'), Ok(Suit::Diamonds));
    /// assert_eq!(Suit::try_from(&'?'), Err(Undecodable("'?': not a Suit".to_string())));
    /// ```
    fn try_from(s: &char) -> Result<Self, Self::Error> {
        match s {
            'D' => Ok(Suit::Diamonds),
            'H' => Ok(Suit::Hearts),
            'C' => Ok(Suit::Clubs),
            'S' => Ok(Suit::Spades),
            _ => Err(Undecodable(format!("{:?}: not a Suit", s))),
        }
    }
}

impl From<&Suit> for char {
    fn from(s: &Suit) -> Self {
        match s {
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Clubs => 'C',
            Suit::Spades => 'S',
        }
    }
}

impl fmt::Display for Suit {
    /// ```
    /// # use poker::*;
    /// assert_eq!(Suit::try_from(&'S').unwrap().to_string(), "S");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self).to_string())
    }
}

/// ```
/// # use poker::*;
/// fn is_sorted(s: &str) {
///     assert_eq!(Hand::of(s).to_string(), s)
/// }
/// is_sorted("2C 3C 4C 5C 6C");
/// is_sorted("6C 7C 8C 9C JC");
/// is_sorted("9C JC QC KC AC");
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = Undecodable;

    /// ```
    /// # use poker::{*, Undecodable, Card, Rank::*, Suit::*};
    /// assert_eq!(Card::try_from("AS"), Ok(Card{rank: Ace, suit: Spades}));
    /// assert_eq!(Card::try_from("A"), Err(Undecodable("\"A\": expected to be at least two chars".to_string())));
    /// ```
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut chrs = s.chars().collect::<Vec<_>>();

        if chrs.len() < 2 {
            return Err(Undecodable(format!(
                "{:?}: expected to be at least two chars",
                s
            )));
        }
        let st = chrs.pop().unwrap();

        Ok(Card {
            rank: Rank::try_from(chrs.iter().collect::<String>().as_str())?,
            suit: Suit::try_from(&st)?,
        })
    }
}

impl fmt::Display for Card {
    /// ```
    /// # use poker::*;
    /// assert_eq!(Card::try_from("AS").unwrap().to_string(), "AS");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(PartialOrd, Ord, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn of(s: &str) -> Self {
        Hand::try_from(s).unwrap_or_else(|v| panic!("{:?}", v))
    }
}

impl TryFrom<&str> for Hand {
    type Error = Undecodable;

    /// ```
    /// # use poker::{Undecodable, Card, Hand, Suit::*, Rank::*};
    /// assert_eq!(Hand::try_from("2C 3C 4C 5C 6C"), Ok(Hand{cards:[
    ///     Card { rank: Two, suit: Clubs },
    ///     Card { rank: Three, suit: Clubs },
    ///     Card { rank: Four, suit: Clubs },
    ///     Card { rank: Five, suit: Clubs },
    ///     Card { rank: Six, suit: Clubs },
    /// ]}));
    /// assert_eq!(Hand::try_from("AS"), Err(Undecodable("\"AS\": expected to be 5 cards".to_string())));
    /// assert_eq!(Hand::try_from("AS AS 3S 4S 5S"), Err(Undecodable("\"AS AS 3S 4S 5S\": expected all cards to be different".to_string())));
    /// ```
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut cards_v: Vec<Card> = s
            .split_ascii_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>, _>>()?;

        if cards_v.len() != 5 {
            return Err(Undecodable(format!("{:?}: expected to be 5 cards", s)));
        };

        if !cards_v
            .windows(2)
            .all(|cs| if let [a, b] = cs { a != b } else { false })
        {
            return Err(Undecodable(format!(
                "{:?}: expected all cards to be different",
                s
            )));
        };

        cards_v.sort();

        let cards = cards_v
            .try_into()
            .map_err(|v| Undecodable(format!("Couldn't turn {:?} into a [Card; 5]", v)))?;

        Ok(Hand { cards })
    }
}

impl fmt::Display for Hand {
    /// ```
    /// # use poker::*;
    /// // Putting cards in Hand sorts them.
    /// assert_eq!(Hand::of("AS 2S 3S 4S 5S").to_string(), "2S 3S 4S 5S AS");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

/// ```
/// # use poker::{*, Rank, Rank::*};
/// fn consecutive_(s: &str) -> Option<Rank> {
///   consecutive(&Hand::of(s).cards)
/// }
///
/// assert_eq!(consecutive_("2C 3C 4C 5C 6C"), Some(Six));
/// assert_eq!(consecutive_("6C 7C 8C 9C 10C"), Some(Ten));
/// assert_eq!(consecutive_("10C JC QC KC AC"), Some(Ace));
/// assert_eq!(consecutive_("AC 2C 3C 4C 5C"), Some(Five));
/// assert_eq!(consecutive_("AC 3C 5C 7C 9C"), None);
/// ```
pub fn consecutive(cards: &[Card]) -> Option<Rank> {
    let ranks: Vec<&Rank> = cards
        .iter()
        .map(|Card { rank, suit: _ }| rank)
        .collect::<Vec<_>>();

    if HashSet::<_>::from_iter(ranks.iter()).len() != cards.len() {
        return None;
    }

    let mut possibilities: Vec<Vec<&Rank>> = vec![ranks];
    let ranks_ = &possibilities[0];
    if ranks_.contains(&&Rank::Ace) {
        let mut alt = ranks_.clone();
        alt.retain(|e| **e != Rank::Ace);
        alt.push(&Rank::AceLow);
        possibilities.push(alt);
    }

    for v in possibilities.iter_mut() {
        v.sort();
        if v.windows(2).all(|w| (*w[0] as u8) + 1 == (*w[1] as u8)) {
            return Some(*v[4]);
        }
    }

    None
}

/// ```
/// # use poker::*;
/// fn same_suit_(s: &str) -> bool {
///   same_suit(&Hand::of(s).cards)
/// }
///
/// assert!(same_suit_("2C 3C 4C 5C 6C"));
/// assert!(!same_suit_("6C 7C 8C 9C JH"));
/// ```
pub fn same_suit(cards: &[Card]) -> bool {
    cards
        .iter()
        .map(|Card { rank: _, suit }| suit)
        .collect::<HashSet<_>>()
        .len()
        == 1
}

/// ```
/// # use poker::{tally_ranks, Undecodable, Card, Hand, Suit::*, Rank::*};
/// assert_eq!(
///     tally_ranks(&Hand::of("KC KH QC QH JC").cards),
///     vec![
///         (2, King),
///         (2, Queen),
///         (1, Jack),
///     ],
/// );
/// ```
pub fn tally_ranks(cards: &[Card]) -> Vec<(usize, Rank)> {
    let mut res: Vec<(usize, Rank)> = cards
        .iter()
        .map(|c| c.rank)
        .fold(HashMap::<Rank, usize>::new(), |mut h, r| {
            *h.entry(r).or_default() += 1;
            h
        })
        .into_iter()
        .map(|(r, s)| (s, r))
        .collect();
    res.sort_by_key(|k| Reverse(*k));
    res
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandScore {
    HighCard {
        ranks: [Rank; 5],
    },
    OnePair {
        top_rank: Rank,
        other_ranks: [Rank; 3],
    },
    TwoPair {
        top_rank: Rank,
        second_rank: Rank,
        other_rank: Rank,
    },
    ThreeOfAKind {
        top_rank: Rank,
        other_ranks: [Rank; 2],
    },
    Straight {
        top_rank: Rank,
    },
    Flush {
        ranks: [Rank; 5],
    },
    FullHouse {
        top_rank: Rank,
        bottom_rank: Rank,
    },
    FourOfAKind {
        top_rank: Rank,
        bottom_rank: Rank,
    },
    StraightFlush {
        top_rank: Rank,
    },
}

impl From<Hand> for HandScore {
    /// ```
    /// # use poker::{Hand, HandScore, HandScore::*, Rank::*};
    /// fn score(s: &str) -> HandScore {
    ///   HandScore::from(Hand::of(s))
    /// }
    ///
    /// assert_eq!(score("AS 2S 3S 4S 5S"), StraightFlush{top_rank: Five});
    ///
    /// assert_eq!(score("AS 3S 5S 7S 9S"), Flush{ranks: [Three, Five, Seven, Nine, Ace]});
    ///
    /// assert_eq!(score("AS AC AD AH 9H"), FourOfAKind{top_rank: Ace, bottom_rank: Nine});
    /// assert_eq!(score("AH 9C 9D 9H 9S"), FourOfAKind{top_rank: Nine, bottom_rank: Ace});
    /// ```
    fn from(h: Hand) -> Self {
        let ranks: [Rank; 5] = {
            let mut r = [Rank::AceLow; 5];
            for (i, c) in h.cards.iter().enumerate() {
                r[i] = c.rank;
            }
            r
        };

        match (
            consecutive(&h.cards),
            same_suit(&h.cards),
            &tally_ranks(&h.cards)[..],
        ) {
            (Some(top_rank), true, _) => HandScore::StraightFlush { top_rank },
            (_, _, [(4, top_rank), (1, bottom_rank)]) => HandScore::FourOfAKind {
                top_rank: *top_rank,
                bottom_rank: *bottom_rank,
            },
            (_, _, [(3, top_rank), (2, bottom_rank)]) => HandScore::FullHouse {
                top_rank: *top_rank,
                bottom_rank: *bottom_rank,
            },
            (None, true, _) => HandScore::Flush { ranks },
            (Some(top_rank), false, _) => HandScore::Straight { top_rank },
            (_, _, [(3, top_rank), (1, other_rank_1), (1, other_rank_2)]) => {
                HandScore::ThreeOfAKind {
                    top_rank: *top_rank,
                    other_ranks: [*other_rank_1, *other_rank_2],
                }
            }
            (_, _, [(2, top_rank), (2, second_rank), (1, other_rank)]) => HandScore::TwoPair {
                top_rank: *top_rank,
                second_rank: *second_rank,
                other_rank: *other_rank,
            },
            (_, _, [(2, top_rank), (1, other_rank_1), (1, other_rank_2), (1, other_rank_3)]) => {
                HandScore::OnePair {
                    top_rank: *top_rank,
                    other_ranks: [*other_rank_1, *other_rank_2, *other_rank_3],
                }
            }
            (_, _, _) => HandScore::HighCard { ranks },
        }
    }
}

impl From<&str> for HandScore {
    fn from(s: &str) -> Self {
        HandScore::from(Hand::of(s))
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 0 {
        return vec![];
    }

    let mut hs: Vec<(HandScore, &str)> = hands.iter().map(|s| {
        (HandScore::from(Hand::of(*s)), *s)
    }).collect();
    hs.sort();
    let highest_score = hs.last().unwrap().0.clone();
    hs.retain(|(hs, _s)| *hs == highest_score);
    hs.iter().map(|(_hs, s)| *s).collect()
}
