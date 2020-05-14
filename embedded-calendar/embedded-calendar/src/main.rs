use std::convert::TryFrom;

type Timestamp = i64;

#[derive(Debug, Clone, Copy)]
struct DateTime {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: f64,
}

impl DateTime {
    fn parse_from_str(s: &str) -> Option<Self> {
        Some(Self {
            year: (&s[0..4]).parse().ok()?,
            month: (&s[5..7]).parse().ok()?,
            day: (&s[8..10]).parse().ok()?,
            hour: (&s[11..13]).parse().ok()?,
            minute: (&s[14..16]).parse().ok()?,
            second: (&s[17..]).parse().ok()?,
        })
    }

    fn add_milliseconds(&self, ms: i64) -> Self {
        self.clone()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02.3}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

#[derive(Debug)]
struct Reference {
    dt: DateTime,
    t: Timestamp,
}

impl From<&str> for Reference {
    fn from(s: &str) -> Self {
        let dt =
            DateTime::parse_from_str(&s[0..23]).unwrap();

        let t: Timestamp = (&s[24..]).parse().unwrap();

        Self { dt, t }
    }
}

#[derive(Debug)]
enum Entry {
    Reference(Reference),
    Get(Timestamp),
}

impl TryFrom<String> for Entry {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.chars().next() {
            Some('R') => Ok(Entry::Reference(
                Reference::from(&s[2..]),
            )),
            Some('G') => {
                Ok(Entry::Get((&s[2..]).parse().unwrap()))
            }
            _ => Err(()),
        }
    }
}

fn run(entries: Vec<Entry>) {
    let mut reference: Option<Reference> = None;

    for entry in entries {
        match entry {
            Entry::Reference(r) => {
                reference = Some(r);
            }
            Entry::Get(t) => {
                if let Some(reference) = &reference {
                    let diff = t - reference.t;
                    let dt =
                        reference.dt.add_milliseconds(diff);
                    println!("{}", dt);
                } else {
                    println!("-");
                }
            }
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(Result::unwrap);

    let entries: Vec<_> = lines
        .filter_map(|line| Entry::try_from(line).ok())
        .collect();

    run(entries);
}
