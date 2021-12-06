use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

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

/// Represents a point in space.
#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate {
    /// X-component of this [Coordinate].
    x: i32,
    /// Y-component of this [Coordinate].
    y: i32,
}

/// Represents a single hydrothermal vent line.
#[derive(Clone, Debug, PartialEq)]
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
