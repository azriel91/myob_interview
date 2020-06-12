use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Health {
    Ok,
    Degraded,
    Down,
    Unknown,
}

impl FromStr for Health {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "ok" => Ok(Health::Ok),
            "degraded" => Ok(Health::Degraded),
            "down" => Ok(Health::Down),
            _ => Err(s.to_string()),
        }
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Health::Ok => write!(f, "Ok"),
            Health::Degraded => write!(f, "Degraded"),
            Health::Down => write!(f, "Down"),
            Health::Unknown => write!(f, "Unknown"),
        }
    }
}
