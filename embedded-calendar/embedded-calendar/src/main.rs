use std::convert::TryFrom;

type Timestamp = i64;

#[derive(Debug, Clone, Copy)]
struct Time {
    hour: i64,
    minute: i64,
    millisecond: i64,
}

const MS_IN_DAY: i64 = 24 * MS_IN_HOUR;
const MS_IN_HOUR: i64 = 60 * MS_IN_MINUTE;
const MS_IN_MINUTE: i64 = 60 * MS_IN_SECOND;
const MS_IN_SECOND: i64 = 1000;

impl Time {
    fn from_milliseconds(millisecond: i64) -> Self {
        let hour = millisecond / MS_IN_HOUR;
        let millisecond = millisecond % MS_IN_HOUR;
        let minute = millisecond / MS_IN_MINUTE;
        let millisecond = millisecond % MS_IN_MINUTE;

        Self {
            hour,
            minute,
            millisecond,
        }
    }

    fn to_milliseconds(&self) -> i64 {
        self.hour * MS_IN_HOUR
            + self.minute * MS_IN_MINUTE
            + self.millisecond
    }
}

#[derive(Debug, Clone, Copy)]
struct DateTime {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    millisecond: i64,
}

impl DateTime {
    fn parse_from_str(s: &str) -> Option<Self> {
        let second: f64 = (&s[17..]).parse().ok()?;
        Some(Self {
            year: (&s[0..4]).parse().ok()?,
            month: (&s[5..7]).parse().ok()?,
            day: (&s[8..10]).parse().ok()?,
            hour: (&s[11..13]).parse().ok()?,
            minute: (&s[14..16]).parse().ok()?,
            millisecond: (second * 1000.0) as i64,
        })
    }

    fn add_milliseconds(&self, ms: i64) -> Self {
        let mut day_offset = ms / MS_IN_DAY;
        let time_offset = ms % MS_IN_DAY;

        let &DateTime {
            mut year,
            mut month,
            mut day,
            ..
        } = self;

        let mut new_time_of_day =
            self.time_of_day_milliseconds() + time_offset;

        if new_time_of_day >= MS_IN_DAY {
            new_time_of_day -= MS_IN_DAY;
            day_offset += 1;
        }

        if day_offset > 0 {
            let (y, m, d) = carry_over_day(
                year,
                month,
                day + day_offset,
            );
            year = y;
            month = m;
            day = d;
        }

        let Time {
            hour,
            minute,
            millisecond,
        } = Time::from_milliseconds(new_time_of_day);

        Self {
            year,
            month,
            day,
            hour,
            minute,
            millisecond,
        }
    }

    fn time_of_day_milliseconds(&self) -> i64 {
        let tod = Time {
            hour: self.hour,
            minute: self.minute,
            millisecond: self.millisecond,
        };
        tod.to_milliseconds()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let second: f64 =
            (self.millisecond as f64) / 1000.0;
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:06.3}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            second
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

impl TryFrom<&str> for Entry {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
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

fn days_in_month(y: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28 + (((y & 3) == 0) as i64),
        _ => 30,
    }
}

fn carry_over_day(
    mut y: i64,
    mut m: i64,
    mut d: i64,
) -> (i64, i64, i64) {
    loop {
        let d_max = days_in_month(y, m);
        if d > d_max {
            d -= d_max;
            m += 1;
        } else {
            break;
        }

        if m > 12 {
            m -= 12;
            y += 1;
        }
    }

    (y, m, d)
}

fn run(entries: Vec<Entry>) {
    let mut reference: Option<Reference> = None;

    for entry in entries {
        match entry {
            Entry::Reference(r) => {
                reference = Some(r);
            }
            Entry::Get(t) => {
                if let Some(r) = &reference {
                    let diff = t - r.t;
                    let dt = r.dt.add_milliseconds(diff);
                    println!("{}", dt);

                    reference = Some(Reference { dt, t });
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
        .filter_map(|line| {
            Entry::try_from(line.as_ref()).ok()
        })
        .collect();

    run(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_time_add_day() {
        let dt = DateTime::parse_from_str(
            "2020-05-14 12:00:00.000",
        )
        .unwrap();
        assert_eq!(dt.day, 14);
        assert_eq!(dt.hour, 12);
        let dt1 = dt.add_milliseconds(MS_IN_DAY);
        assert_eq!(dt1.day, 15);
        assert_eq!(dt1.hour, 12);
    }

    #[test]
    fn date_time_add_month() {
        let dt = DateTime::parse_from_str(
            "2020-05-14 12:00:00.000",
        )
        .unwrap();
        assert_eq!(dt.month, 5);
        assert_eq!(dt.day, 14);
        assert_eq!(dt.hour, 12);
        let dt1 = dt.add_milliseconds(31 * MS_IN_DAY);
        assert_eq!(dt1.month, 6);
        assert_eq!(dt1.day, 14);
        assert_eq!(dt1.hour, 12);
    }

    #[test]
    fn date_time_add_hour() {
        let dt = DateTime::parse_from_str(
            "2020-05-14 12:00:00.000",
        )
        .unwrap();
        assert_eq!(dt.month, 5);
        assert_eq!(dt.day, 14);
        assert_eq!(dt.hour, 12);
        let dt1 = dt.add_milliseconds(1 * MS_IN_HOUR);
        assert_eq!(dt1.month, 5);
        assert_eq!(dt1.day, 14);
        assert_eq!(dt1.hour, 13);
    }

    #[test]
    fn date_time_new_year() {
        let reference = Reference {
            dt: DateTime {
                year: 2019,
                month: 10,
                day: 29,
                hour: 16,
                minute: 17,
                millisecond: 13777,
            },
            t: 1572365832000,
        };

        let t = 1580458588000;

        let t =
            reference.dt.add_milliseconds(t - reference.t);

        assert_eq!(t.year, 2020);
        assert_eq!(t.month, 1);
        assert_eq!(t.day, 31);
    }
}
