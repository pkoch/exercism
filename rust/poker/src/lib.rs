// I wish I could use creates. :(

use std::{
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
    Jack,
    Queen,
    King,
    Ace,
}

impl From<Rank> for char {
    fn from(r: Rank) -> Self {
        match r {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
            Rank::AceLow => 'A',
        }
    }
}

impl fmt::Display for Rank {
    /// ```
    /// # use poker::*;
    /// assert_eq!(Rank::try_from('A').unwrap().to_string(), "A");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<char> for Rank {
    type Error = Undecodable;

    /// ```
    /// # use poker::*;
    /// assert_eq!(Rank::try_from('2'), Ok(Rank::Two));
    /// assert_eq!(Rank::try_from('?'), Err(Undecodable("'?': not a Rank".to_string())));
    /// ```
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' => Ok(Rank::Ace),
            _ => Err(Undecodable(format!("{:?}: not a Rank", s))),
        }
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

impl Into<char> for Suit {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl fmt::Display for Suit {
    /// ```
    /// # use poker::*;
    /// assert_eq!(Suit::try_from('S').unwrap().to_string(), "S");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self as u8 as char).to_string())
    }
}

impl TryFrom<char> for Suit {
    type Error = Undecodable;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'D' => Ok(Suit::Diamonds),
            'H' => Ok(Suit::Hearts),
            'C' => Ok(Suit::Clubs),
            'S' => Ok(Suit::Spades),
            _ => Err(Undecodable(format!("{:?}: not a Suit", s))),
        }
    }
}

/// ```
/// # use poker::*;
/// fn sorted(s: &str) -> String {
///   let mut h = Hand::try_from(s).unwrap();
///   h.cards.sort();
///   h.to_string()
/// }
///
/// assert_eq!(sorted("AS 2S 3S 4S 5S"), "2S 3S 4S 5S AS");
/// assert_eq!(sorted("AS AC 3S 4D AH"), "3S 4D AC AH AS");
///
/// fn is_sorted(s: &str) {
///     assert_eq!(sorted(s), s)
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

impl fmt::Display for Card {
    /// ```
    /// # use poker::*;
    /// let source = "AS";
    /// assert_eq!(Card::try_from(source).unwrap().to_string(), source);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl TryFrom<&str> for Card {
    type Error = Undecodable;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let chrs = s.chars().collect::<Vec<_>>();
        if chrs.len() != 2 {
            return Err(Undecodable(format!("{:?}: expected to be two chars", s)));
        };
        Ok(Card {
            rank: Rank::try_from(chrs[0])?,
            suit: Suit::try_from(chrs[1])?,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
}

/// ```
/// # use poker::*;
/// let source = "AS 2S 3S 4S 5S";
/// assert_eq!(Hand::try_from(source).unwrap().to_string(), source);
/// ```
impl fmt::Display for Hand {
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
        let cards_v: Vec<Card> = s
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

        let cards = cards_v
            .try_into()
            .map_err(|v| Undecodable(format!("Couldn't turn {:?} into a [Card; 5]", v)))?;

        Ok(Hand { cards })
    }
}

/// ```
/// # use poker::*;
/// fn consecutive_(s: &str) -> bool {
///   consecutive(&Hand::try_from(s).unwrap().cards)
/// }
///
/// assert!(consecutive_("2C 3C 4C 5C 6C"));
/// assert!(consecutive_("6C 7C 8C 9C JC"));
/// assert!(consecutive_("9C JC QC KC AC"));
/// assert!(consecutive_("AC 2C 3C 4C 5C"));
/// ```
pub fn consecutive(cards: &[Card]) -> bool {
    if cards.len() < 2 {
        return true;
    }

    let ranks = cards
        .iter()
        .map(|Card { rank, suit: _ }| *rank)
        .collect::<Vec<_>>();

    if HashSet::<_>::from_iter(ranks.iter()).len() != cards.len() {
        return false;
    }

    let mut possibilities = vec![ranks];
    let ranks_ = possibilities.first().unwrap();
    if ranks_.contains(&Rank::Ace) {
        let mut alt = ranks_.clone();
        alt.retain(|e| *e != Rank::Ace);
        alt.push(Rank::AceLow); // An Ace can also go before a Two.
        possibilities.push(alt);
    }

    for v in possibilities.iter_mut() {
        v.sort()
    }

    possibilities.iter().any(|p| {
        p.windows(2).all(|w| {
            if let [a, b] = w {
                (*a as u8) + 1 == (*b as u8)
            } else {
                false
            }
        })
    })
}

/// ```
/// # use poker::*;
/// fn same_suit_(s: &str) -> bool {
///   same_suit(&Hand::try_from(s).unwrap().cards)
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
/// # use poker::*;
/// fn same_rank_4(s: &str) -> bool {
///   same_rank(&Hand::try_from(s).unwrap().cards[0..3])
/// }
///
/// assert!(same_rank_4("2C 2S 2D 2H AS"));
/// assert!(!same_rank_4("2C 2S 3D 3H AS"));
/// ```
pub fn same_rank(cards: &[Card]) -> bool {
    cards
        .iter()
        .map(|Card { rank, suit: _ }| rank)
        .collect::<HashSet<_>>()
        .len()
        == 1
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandScore {
    HighCard {
        ranks: Vec<Rank>,
    },
    OnePair {
        top_rank: Rank,
        other_ranks: Vec<Rank>,
    },
    TwoPair {
        top_rank: Rank,
        second_rank: Rank,
        other_rank: Rank,
    },
    ThreeOfAKind {
        top_rank: Rank,
        other_ranks: Vec<Rank>,
    },
    Straight {
        top_rank: Rank,
    },
    Flush {
        ranks: Vec<Rank>,
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

/// ```
/// # use poker::{tally_ranks, Undecodable, Card, Hand, Suit::*, Rank::*};
/// assert_eq!(
///     tally_ranks(&Hand::try_from("KC KH QC QH JC").unwrap().cards),
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
    res.sort_by(|a, b| b.cmp(a));
    res
}

impl From<Hand> for HandScore {
    /// ```
    /// # use poker::{Hand, HandScore, HandScore::*, Rank::*};
    /// fn score(s: &str) -> HandScore {
    ///   HandScore::from(Hand::try_from(s).unwrap())
    /// }
    ///
    /// assert_eq!(score("AS 2S 3S 4S 5S"), StraightFlush{top_rank: Ace});
    ///
    /// assert_eq!(score("AS 3S 5S 7S 9S"), Flush{ranks: vec![Three, Five, Seven, Nine, Ace]});
    ///
    /// assert_eq!(score("AS AC AD AH 9H"), FourOfAKind{top_rank: Ace, bottom_rank: Nine});
    /// assert_eq!(score("AH 9C 9D 9H 9S"), FourOfAKind{top_rank: Nine, bottom_rank: Ace});
    /// ```
    fn from(h: Hand) -> Self {
        let mut cards = h.cards;
        cards.sort();
        let same_suit = same_suit(&cards);
        let consecutive = consecutive(&cards);
        let tally = &tally_ranks(&cards)[..];

        if consecutive && same_suit {
            return HandScore::StraightFlush {
                top_rank: cards[4].rank,
            };
        };

        if let [(4, top_rank), (1, bottom_rank)] = tally {
            return HandScore::FourOfAKind {
                top_rank: *top_rank,
                bottom_rank: *bottom_rank,
            };
        }

        if let [(3, top_rank), (2, bottom_rank)] = tally {
            return HandScore::FullHouse {
                top_rank: *top_rank,
                bottom_rank: *bottom_rank,
            };
        }

        if same_suit {
            let ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
            return HandScore::Flush { ranks };
        };

        if consecutive {
            return HandScore::Straight {
                top_rank: cards[4].rank,
            };
        };

        if let [(3, top_rank), (1, other_rank_1), (1, other_rank_2)] = tally {
            return HandScore::ThreeOfAKind {
                top_rank: *top_rank,
                other_ranks: vec![*other_rank_1, *other_rank_2],
            };
        }

        if let [(2, top_rank), (2, second_rank), (1, other_rank)] = tally {
            return HandScore::TwoPair {
                top_rank: *top_rank,
                second_rank: *second_rank,
                other_rank: *other_rank,
            };
        }

        if let [(2, top_rank), (1, other_rank_1), (1, other_rank_2), (1, other_rank_3)] = tally {
            return HandScore::OnePair {
                top_rank: *top_rank,
                other_ranks: vec![*other_rank_1, *other_rank_2, *other_rank_3],
            };
        }

        let ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
        HandScore::HighCard { ranks }
    }
}

impl From<&str> for HandScore {
    fn from(s: &str) -> Self {
        HandScore::from(Hand::try_from(s).unwrap_or_else(|v| panic!("{:?}", v)))
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hs: Vec<(HandScore, &str)> =
        hands.iter().map(|h| (HandScore::from(*h), *h)).collect();
    hs.sort();
    let highest_score = hs.last().unwrap().0.clone();
    hs.retain(|(s, _)| *s == highest_score);
    hs.iter().map(|(_h, s)| *s).collect()
}
