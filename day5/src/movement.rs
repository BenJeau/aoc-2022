use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Movement {
    pub amount: u32,
    pub from: u32,
    pub to: u32,
}

impl FromStr for Movement {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let captures = RE.captures(s).unwrap();

        Ok(Movement {
            amount: captures[1].parse::<u32>()?,
            from: captures[2].parse::<u32>()?,
            to: captures[3].parse::<u32>()?,
        })
    }
}
