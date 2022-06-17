use std::ops::RangeInclusive;

use chrono::{Datelike, NaiveDate};

pub enum Period {
    Month(u32),
    Months(u32, u32),
    Quarter(u32),
    Halfyear(u32),
    Year,
}

fn clamp(v: &u32, min: u32, max: u32) -> u32 {
    u32::max(min, u32::min(max, *v))
}

impl Period {
    pub fn as_range(&self) -> RangeInclusive<u32> {
        match self {
            Period::Month(m) => {
                let m = clamp(m, 1, 12);
                m..=m
            }
            Period::Months(from, until) => {
                let from = clamp(from, 1, 12);
                let until = clamp(until, 1, 12);
                from..=until
            }
            Period::Quarter(q) => {
                let q = clamp(q, 1, 4);
                match q {
                    1 => 1..=3,
                    2 => 4..=6,
                    3 => 7..=9,
                    _ => 10..=12,
                }
            }
            Period::Halfyear(hy) => {
                let hy = clamp(hy, 1, 2);
                if hy == 1 {
                    1..=6
                } else {
                    7..=12
                }
            }
            Period::Year => 1..=12,
        }
    }

    pub fn contains(&self, d: NaiveDate) -> bool {
        let month = d.month();
        let rng = self.as_range();
        rng.contains(&month)
    }
}
