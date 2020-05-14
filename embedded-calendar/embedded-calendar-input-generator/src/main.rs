use chrono::NaiveDateTime;
use std::convert::TryFrom;

type Timestamp = u64;

#[derive(Debug)]
struct Reference {
    dt: NaiveDateTime,
    t: Timestamp,
}

impl From<&str> for Reference {
    fn from(s: &str) -> Self {
        let dt = NaiveDateTime::parse_from_str(
            &s[0..23],
            "%Y-%m-%d %H:%M:%S%.3f",
        )
        .unwrap();

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
            Entry::Get(timestamp) => {
                if let Some(reference) = &reference {
                    let diff = (timestamp as i64) - (reference.t as i64);
                    let diff = chrono::Duration::milliseconds(diff);
                    let dt = reference.dt.checked_add_signed(diff).unwrap();
                    println!("{}", dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string());
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
