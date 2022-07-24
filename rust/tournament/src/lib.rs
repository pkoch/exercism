use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}
use Outcome::*;

impl Outcome {
    fn flip(&self) -> Self {
        match self {
            Loss => Win,
            Draw => Draw,
            Win => Loss,
        }
    }
}

impl From<&Outcome> for i32 {
    fn from(o: &Outcome) -> Self {
        match o {
            Loss => 0,
            Draw => 1,
            Win => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "loss" => Ok(Loss),
            "draw" => Ok(Draw),
            "win" => Ok(Win),
            _ => Err(format!("{:?}: not a valid Outcome", s)),
        }
    }
}

#[derive(Debug)]
struct Match<'a> {
    home: &'a str,
    away: &'a str,
    outcome: Outcome,
}

impl<'a> Match<'a> {
    fn from_str<'b: 'a>(s: &'b str) -> Result<Self, String> {
        let atoms: Vec<&'b str> = s.splitn(3, ';').collect();
        if atoms.len() != 3 {
            return Err(format!("{:?} must have 3 fields separated by ';'.", s));
        };
        Ok(Match::<'a> {
            home: atoms[0],
            away: atoms[1],
            outcome: atoms[2].parse()?,
        })
    }
}

struct LeaderboardEntry<'a> {
    team: &'a str,
    mp: i32,
    w: i32,
    d: i32,
    l: i32,
    p: i32,
}

impl<'a> Display for LeaderboardEntry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.team, self.mp, self.w, self.l, self.d, self.p,
        )
    }
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let matches: Vec<Match> = match_results
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| Match::from_str(l).unwrap())
        .collect();

    let team_outcomes: HashMap<&str, Vec<Outcome>> = matches
        .iter()
        .flat_map(|m| [(m.home, m.outcome), (m.away, m.outcome.flip())])
        .fold(HashMap::<&str, Vec<Outcome>>::new(), |mut ti, (t, m)| {
            ti.entry(t).or_default().push(m);
            ti
        });

    let mut leaderboard_entries = team_outcomes
        .iter()
        .map(|(team, outcomes)| LeaderboardEntry {
            team: *team,
            mp: outcomes.len() as i32,
            w: outcomes.iter().filter(|o| **o == Win).count() as i32,
            d: outcomes.iter().filter(|o| **o == Loss).count() as i32,
            l: outcomes.iter().filter(|o| **o == Draw).count() as i32,
            p: outcomes.iter().map::<i32, _>(|o| o.into()).sum(),
        })
        .collect::<Vec<LeaderboardEntry>>();
    leaderboard_entries.sort_by_key(|le| (-le.p, le.team));

    vec![HEADER.to_string()]
        .into_iter()
        .chain(leaderboard_entries.iter().map(LeaderboardEntry::to_string))
        .collect::<Vec<String>>()
        .join("\n")
}
