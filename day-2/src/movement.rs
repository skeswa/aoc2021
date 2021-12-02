use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

/// Enumerates every possible direction of movement for the submarine.
#[derive(Debug, PartialEq)]
pub enum Movement {
    /// Describes a downward movement with a specified [i32] magnitude.
    Down(i32),
    /// Describes a forward movement with a specified [i32] magnitude.
    Forward(i32),
    /// Describes an upward movement with a specified [i32] magnitude.
    Up(i32),
}

lazy_static! {
    /// Regular expression designed to match strings that look like
    /// " forward 2" and "up 6  ".
    ///
    /// Capture groups:
    /// *   [`1`] direction
    /// *   [`2`] magnitude
    static ref MOVEMENT_PATTERN: Regex =
        Regex::new(
            format!(
                r"\s*(?P<{}>[a-z]+)\s(?P<{}>\d)\s*",
                capture_group_name::MOVEMENT_DIRECTION,
                capture_group_name::MOVEMENT_MAGNITUDE,
            ).as_str()).unwrap();
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        let captures = MOVEMENT_PATTERN
            .captures(input)
            .with_context(|| format!("\"{}\" is not a valid movement", input))?;
        let raw_direction = captures
            .name(capture_group_name::MOVEMENT_DIRECTION)
            .with_context(|| format!("\"{}\" is not a valid movement (invalid direction)", input))?
            .as_str();
        let raw_magnitude = captures
            .name(capture_group_name::MOVEMENT_MAGNITUDE)
            .with_context(|| {
                format!("\"{}\" is not a valid movement (invalid magnitude)", input)
            })?;

        let magnitude = raw_magnitude.as_str().parse::<i32>().with_context(|| {
            format!("\"{}\" is not a valid movement (invalid magnitude)", input)
        })?;

        match raw_direction {
            direction_label::DOWN => Ok(Movement::Down(magnitude)),
            direction_label::FORWARD => Ok(Movement::Forward(magnitude)),
            direction_label::UP => Ok(Movement::Up(magnitude)),
            _ => Err(anyhow!(
                "\"{}\" is not a valid movement (invalid direction)",
                raw_direction
            )),
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Movement::Down(magnitude) => {
                write!(f, "{} {}", direction_label::DOWN, magnitude)
            }
            Movement::Forward(magnitude) => write!(f, "{} {}", direction_label::FORWARD, magnitude),
            Movement::Up(magnitude) => {
                write!(f, "{} {}", direction_label::UP, magnitude)
            }
        }
    }
}

/// Module used to namespace movement regular expression capture group names.
mod capture_group_name {
    /// Name of the capture group used to select the direction of movement string.
    pub const MOVEMENT_DIRECTION: &str = "direction";

    /// Name of the capture group used to select the magnitude of movement string.
    pub const MOVEMENT_MAGNITUDE: &str = "magnitude";
}

/// Module used to namespace text labels for movement directions.
mod direction_label {
    /// Text snippet associated with [Movement::Down].
    pub const DOWN: &str = "down";

    /// Text snippet associated with [Movement::Forward].
    pub const FORWARD: &str = "forward";

    /// Text snippet associated with [Movement::Up].
    pub const UP: &str = "up";
}
