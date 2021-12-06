use crate::coordinate::{Coordinate, Coordinates};
use crate::traceable::Traceable;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

/// Represents a single hydrothermal vent line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HydrothermalVentLine {
    /// Where this [HydrothermalVentLine] starts.
    beginning: Coordinate,
    /// Where this [HydrothermalVentLine] terminates.
    end: Coordinate,
}

impl HydrothermalVentLine {
    /// Interprets the given [str] as a [HydrothermalVentLine].
    pub fn deserialize(serialized_hydrothermal_vent_line: &str) -> Result<HydrothermalVentLine> {
        let captures = VENT_LINE_PATTERN
            .captures(serialized_hydrothermal_vent_line)
            .with_context(|| {
                format!(
                    "\"{}\" is not a valid serialized hydrothermal vent line",
                    serialized_hydrothermal_vent_line
                )
            })?;

        let x1 = captures
            .name(capture_group_name::X1)
            .and_then(|capture| capture.as_str().parse::<i32>().ok())
            .with_context(|| {
                format!("\"{}\" lacks a valid x1", serialized_hydrothermal_vent_line)
            })?;
        let y1 = captures
            .name(capture_group_name::Y1)
            .and_then(|capture| capture.as_str().parse::<i32>().ok())
            .with_context(|| {
                format!("\"{}\" lacks a valid y1", serialized_hydrothermal_vent_line)
            })?;
        let x2 = captures
            .name(capture_group_name::X2)
            .and_then(|capture| capture.as_str().parse::<i32>().ok())
            .with_context(|| {
                format!("\"{}\" lacks a valid x2", serialized_hydrothermal_vent_line)
            })?;
        let y2 = captures
            .name(capture_group_name::Y2)
            .and_then(|capture| capture.as_str().parse::<i32>().ok())
            .with_context(|| {
                format!("\"{}\" lacks a valid y2", serialized_hydrothermal_vent_line)
            })?;

        Ok(HydrothermalVentLine {
            beginning: Coordinate { x: x1, y: y1 },
            end: Coordinate { x: x2, y: y2 },
        })
    }

    /// Returns `true` if this [HydrothermalVentLine] is a diagonal line.
    fn is_diagonal(&self) -> bool {
        (self.end.x - self.beginning.x).abs() == (self.end.y - self.beginning.y).abs()
    }

    /// Returns `true` if this [HydrothermalVentLine] is a horizontal line.
    fn is_horizontal(&self) -> bool {
        self.beginning.y == self.end.y
    }

    /// Returns `true` if this [HydrothermalVentLine] is a vertical line.
    fn is_vertical(&self) -> bool {
        self.beginning.x == self.end.x
    }
}

impl Traceable for HydrothermalVentLine {
    fn can_trace(&self, are_diagonals_allowed: bool) -> bool {
        self.is_horizontal() || self.is_vertical() || (are_diagonals_allowed && self.is_diagonal())
    }

    fn trace(&self, are_diagonals_allowed: bool) -> Result<Coordinates> {
        if !self.can_trace(are_diagonals_allowed) {
            return Err(anyhow!("{:?} is untraceable", self));
        }

        let mut coordinate = self.beginning;
        let Coordinate {
            x: destination_x,
            y: destination_y,
        } = self.end;
        let mut coordinates = vec![coordinate];

        while coordinate != self.end {
            coordinate = Coordinate {
                x: if destination_x > coordinate.x {
                    coordinate.x + 1
                } else if destination_x < coordinate.x {
                    coordinate.x - 1
                } else {
                    coordinate.x
                },
                y: if destination_y > coordinate.y {
                    coordinate.y + 1
                } else if destination_y < coordinate.y {
                    coordinate.y - 1
                } else {
                    coordinate.y
                },
            };

            coordinates.push(coordinate)
        }

        Ok(Coordinates::from(coordinates))
    }
}

/// Represents a collection of hydrothermal vent lines.
#[derive(Clone, Debug, PartialEq)]
pub struct HydrothermalVentLines(Vec<HydrothermalVentLine>);

impl HydrothermalVentLines {
    /// Interprets a newline-delimited [str] of serialized hydrothermal vent
    /// lines as [HydrothermalVentLines].
    pub fn deserialize(serialized_hydrothermal_vent_lines: &str) -> Result<HydrothermalVentLines> {
        let hydrothermal_vent_lines = serialized_hydrothermal_vent_lines
            .lines()
            .map(HydrothermalVentLine::deserialize)
            .collect::<Result<Vec<HydrothermalVentLine>>>()
            .with_context(|| {
                format!(
                    "\"{}\" is not a valid collection of serialized hydrothermal vent lines",
                    serialized_hydrothermal_vent_lines
                )
            })?;

        Ok(HydrothermalVentLines(hydrothermal_vent_lines))
    }

    /// Returns a clone of this [HydrothermalVentLines] sans any untraceable
    /// hydrothermal vent lines.
    ///
    /// Parameters:
    /// *   `are_diagonals_allowed`\
    ///     Is `true` if diagonal lines are considered to be traceable.
    pub fn without_untraceable_vent_lines(
        &self,
        are_diagonals_allowed: bool,
    ) -> HydrothermalVentLines {
        HydrothermalVentLines(
            self.0
                .iter()
                .filter(|vent_line| vent_line.can_trace(are_diagonals_allowed))
                .map(|vent_line| vent_line.to_owned())
                .collect(),
        )
    }
}

impl Traceable for HydrothermalVentLines {
    fn can_trace(&self, _: bool) -> bool {
        true
    }

    fn trace(&self, are_diagonals_allowed: bool) -> Result<Coordinates> {
        let coordinates = self
            .0
            .iter()
            .map(|vent_line| vent_line.trace(are_diagonals_allowed))
            .collect::<Result<Coordinates>>()
            .context("Cannot trace every hydrothermal vent line")?;

        Ok(coordinates)
    }
}

/// Module used to namespace regular expression capture group names.
mod capture_group_name {
    /// Name of the capture group used to select first X-coordinate.
    pub const X1: &str = "x1";

    /// Name of the capture group used to select second X-coordinate.
    pub const X2: &str = "x2";

    /// Name of the capture group used to select first Y-coordinate.
    pub const Y1: &str = "y1";

    /// Name of the capture group used to select second Y-coordinate.
    pub const Y2: &str = "y2";
}

lazy_static! {
    /// Regular expression designed to match hydrothermal vent lines.
    ///
    /// Capture groups:
    /// *   [`1`] x1
    /// *   [`2`] y1
    /// *   [`3`] x2
    /// *   [`4`] y2
    static ref VENT_LINE_PATTERN: Regex =
        Regex::new(format!(
            r"\s*(?P<{}>\d+),\s*(?P<{}>\d+)\s*->\s*(?P<{}>\d+),\s*(?P<{}>\d+)\s*",
            capture_group_name::X1,
            capture_group_name::Y1,
            capture_group_name::X2,
            capture_group_name::Y2,
        ).as_str()).unwrap();
}
