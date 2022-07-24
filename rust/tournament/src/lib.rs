use std::{collections::HashMap, str::FromStr};

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
struct Match {
    home: String,
    away: String,
    outcome: Outcome,
}

impl Match {
    fn outcome_for(&self, team: &str) -> Outcome {
        if self.home == team {
            return self.outcome;
        }
        if self.away == team {
            return self.outcome.flip();
        }
        panic!("{:?} is not a team involved in {:?}", team, self)
    }
}

impl FromStr for Match {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atoms: Vec<&str> = s.splitn(3, ';').collect();
        if atoms.len() != 3 {
            return Err(format!("{:?} must have 3 fields separated by ';'.", s));
        };
        Ok(Match {
            home: atoms[0].to_string(),
            away: atoms[1].to_string(),
            outcome: atoms[2].parse()?,
        })
    }
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let matches: Vec<Match> = match_results
        .split("\n")
        .filter(|s| *s != "")
        .map(|l| Match::from_str(l).unwrap())
        .collect();
    let team_involvements: HashMap<&str, Vec<&Match>> = matches
        .iter()
        .flat_map(|m| [(m.home.as_str(), m), (m.away.as_str(), m)])
        .fold(HashMap::<&str, Vec<&Match>>::new(), |mut ti, (t, m)| {
            ti.entry(t).or_default().push(m);
            ti
        });
    let mut leaderboard_entries = team_involvements
    .iter()
    .map(|(team, involvements)| {
        let outcomes: Vec<Outcome> = involvements
            .iter()
            .map(|i| i.outcome_for(team))
            .collect();
        (
            *team,
            outcomes.len() as i32,
            outcomes.iter().filter(|o| **o == Win).count() as i32,
            outcomes.iter().filter(|o| **o == Loss).count() as i32,
            outcomes.iter().filter(|o| **o == Draw).count() as i32,
            outcomes.iter().map::<i32, _>(|o| o.into()).sum::<i32>(),
        )
    })
    .collect::<Vec<(&str, i32, i32, i32, i32, i32)>>();
    leaderboard_entries.sort_by_key(|t| (t.5 * -1, t.0));

    let mut res = vec![HEADER.to_string()];
    res.extend(leaderboard_entries
        .iter()
        .map(|(team, mp, w, d, l, p)| {
            format!(
                "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                team,
                mp,
                w,
                l,
                d,
                p,
            )
        })
        .collect::<Vec<String>>()
    );
    res.join("\n")
}
