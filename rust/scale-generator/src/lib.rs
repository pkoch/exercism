use std::str::FromStr;

type Error = String;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Note {
    A = 0,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

impl TryFrom<u8> for Note {
    type Error = String;
    fn try_from(n: u8) -> Result<Self, Error> {
        match n {
            0 => Ok(Self::A),
            1 => Ok(Self::ASharp),
            2 => Ok(Self::B),
            3 => Ok(Self::C),
            4 => Ok(Self::CSharp),
            5 => Ok(Self::D),
            6 => Ok(Self::DSharp),
            7 => Ok(Self::E),
            8 => Ok(Self::F),
            9 => Ok(Self::FSharp),
            10 => Ok(Self::G),
            11 => Ok(Self::GSharp),
            _ => Err(format!("{:?}: not a note", n)),
        }
    }
}

impl FromStr for Note {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "A" => Ok(Note::A),
            "A#" => Ok(Note::ASharp),
            "BB" => Ok(Note::ASharp),
            "B" => Ok(Note::B),
            "C" => Ok(Note::C),
            "C#" => Ok(Note::CSharp),
            "DB" => Ok(Note::CSharp),
            "D" => Ok(Note::D),
            "D#" => Ok(Note::DSharp),
            "EB" => Ok(Note::DSharp),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "F#" => Ok(Note::FSharp),
            "GB" => Ok(Note::FSharp),
            "G" => Ok(Note::G),
            "G#" => Ok(Note::GSharp),
            "AB" => Ok(Note::GSharp),
            _ => Err(format!("{:?}: not a note", s)),
        }
    }
}

pub struct Tonic {
    pub note: Note,
    pub notation: TonicNotation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum TonicNotation {
    Sharp,
    Flat,
}

impl TonicNotation {
    pub fn display(&self, note: Note) -> String {
        use Note::*;

        match *self {
            Self::Sharp => match note {
                A => "A",
                ASharp =>  "A#",
                B => "B",
                C => "C",
                CSharp => "C#",
                D => "D",
                DSharp => "D#",
                E => "E",
                F => "F",
                FSharp => "F#",
                G => "G",
                GSharp => "G#",
            },
            Self::Flat => match note {
                A => "A",
                ASharp =>  "Bb",
                B => "B",
                C => "C",
                CSharp => "Db",
                D => "D",
                DSharp => "Eb",
                E => "E",
                F => "F",
                FSharp => "Gb",
                G => "G",
                GSharp => "Ab",
            },
        }.to_string()
    }
}

impl FromStr for Tonic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Tonic {
                note: Note::A,
                notation: TonicNotation::Sharp,
            }),
            "A#" => unimplemented!(),
            "Bb" => Ok(Tonic {
                note: Note::ASharp,
                notation: TonicNotation::Flat,
            }),
            "B" => Ok(Tonic {
                note: Note::B,
                notation: TonicNotation::Sharp,
            }),
            "C" => Ok(Tonic {
                note: Note::C,
                notation: TonicNotation::Sharp,
            }), // Ambivalent
            "C#" => unimplemented!(),
            "Db" => Ok(Tonic {
                note: Note::CSharp,
                notation: TonicNotation::Flat,
            }),
            "D" => Ok(Tonic {
                note: Note::D,
                notation: TonicNotation::Sharp,
            }),
            "D#" => unimplemented!(),
            "Eb" => Ok(Tonic {
                note: Note::DSharp,
                notation: TonicNotation::Flat,
            }),
            "E" => Ok(Tonic {
                note: Note::E,
                notation: TonicNotation::Sharp,
            }),
            "F" => Ok(Tonic {
                note: Note::F,
                notation: TonicNotation::Flat,
            }),
            "F#" => Ok(Tonic {
                note: Note::FSharp,
                notation: TonicNotation::Sharp,
            }),
            "Gb" => Ok(Tonic {
                note: Note::FSharp,
                notation: TonicNotation::Flat,
            }),
            "G" => Ok(Tonic {
                note: Note::G,
                notation: TonicNotation::Sharp,
            }),
            "G#" => unimplemented!(),
            "Ab" => Ok(Tonic {
                note: Note::GSharp,
                notation: TonicNotation::Flat,
            }),
            "a" => Ok(Tonic {
                note: Note::A,
                notation: TonicNotation::Sharp,
            }), // Ambivalent
            "a#" => unimplemented!(),
            "bb" => Ok(Tonic {
                note: Note::ASharp,
                notation: TonicNotation::Flat,
            }),
            "b" => Ok(Tonic {
                note: Note::B,
                notation: TonicNotation::Sharp,
            }),
            "c" => Ok(Tonic {
                note: Note::C,
                notation: TonicNotation::Flat,
            }),
            "c#" => Ok(Tonic {
                note: Note::CSharp,
                notation: TonicNotation::Sharp,
            }),
            "db" => unimplemented!(),
            "d" => Ok(Tonic {
                note: Note::D,
                notation: TonicNotation::Flat,
            }),
            "d#" => Ok(Tonic {
                note: Note::DSharp,
                notation: TonicNotation::Sharp,
            }),
            "eb" => Ok(Tonic {
                note: Note::DSharp,
                notation: TonicNotation::Flat,
            }),
            "e" => Ok(Tonic {
                note: Note::E,
                notation: TonicNotation::Sharp,
            }),
            "f" => Ok(Tonic {
                note: Note::F,
                notation: TonicNotation::Flat,
            }),
            "f#" => Ok(Tonic {
                note: Note::FSharp,
                notation: TonicNotation::Sharp,
            }),
            "gb" => unimplemented!(),
            "g" => Ok(Tonic {
                note: Note::G,
                notation: TonicNotation::Flat,
            }),
            "g#" => Ok(Tonic {
                note: Note::GSharp,
                notation: TonicNotation::Sharp,
            }),
            "ab" => unimplemented!(),
            _ => Err(format!("{:?}: not a note", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Interval {
    Minor = 1,
    Major = 2,
    Augmented = 3,
}

impl Interval {
    pub fn advance(&self, n: Note) -> Note {
        let highest_note = Note::GSharp;
        ((n as u8 + *self as u8) % (highest_note as u8 + 1)).try_into().expect("highest_note must be set to the highest enum value")
    }
}

impl TryFrom<char> for Interval {
    type Error = String;

    fn try_from(c: char) -> Result<Interval, Self::Error> {
        match c {
            'm' => Ok(Self::Minor),
            'M' => Ok(Self::Major),
            'A' => Ok(Self::Augmented),
            _ => Err(format!("{:?}: not an interval", c)),
        }
    }
}

pub struct Scale {
    tonic: Tonic,
    intervals: Vec<Interval>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: tonic.parse()?,
            intervals: intervals
                .chars()
                .map(Interval::try_from)
                .collect::<Result<Vec<Interval>, Error>>()?,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let note = self.tonic.note;
        let notation = self.tonic.notation;
        self.intervals
            .iter()
            .fold(vec![note], |mut notes, next| {
                notes.push(next.advance(*notes.last().expect("notes can't be empty")));
                notes
            })
            .iter()
            .map(|n| notation.display(*n))
            .collect()
    }
}
