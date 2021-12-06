use anyhow::{Context, Result};
use hydrothermal_vent_lines::HydrothermalVentLines;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::hydrothermal_vent_lines::{Coordinate, Traceable};

extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod hydrothermal_vent_lines;

#[tokio::main]
async fn main() -> Result<()> {
    let hydrothermal_vent_lines = read_hydrothermal_vent_lines("files/input.txt").await?;

    let coordinates_with_multiple_overlapping_vent_lines = hydrothermal_vent_lines
        .without_untraceable_ven_lines()
        .trace()?
        .aggregate()
        .iter()
        .filter(|(_, coordinate_count)| **coordinate_count > 1)
        .map(|(coordinate, _)| *coordinate)
        .collect::<Vec<Coordinate>>();

    println!(
        "Coordinates with multiple overlapping vent lines: {}",
        coordinates_with_multiple_overlapping_vent_lines.len(),
    );

    Ok(())
}

/// Reads the contents of the "diagnostic report" input file as a
/// newline-separated list of binary numbers.
async fn read_hydrothermal_vent_lines(
    hydrothermal_vent_lines_file_path: &str,
) -> Result<HydrothermalVentLines> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let hydrothermal_vent_lines_file_path_buf = pwd.join(hydrothermal_vent_lines_file_path);

    let mut hydrothermal_vent_lines_file = File::open(&hydrothermal_vent_lines_file_path_buf)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                hydrothermal_vent_lines_file_path_buf.display()
            )
        })?;
    let mut raw_hydrothermal_vent_lines_file_contents = vec![];

    hydrothermal_vent_lines_file
        .read_to_end(&mut raw_hydrothermal_vent_lines_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                hydrothermal_vent_lines_file_path_buf.display()
            )
        })?;

    let hydrothermal_vent_lines_file_contents =
        String::from_utf8_lossy(&raw_hydrothermal_vent_lines_file_contents);

    HydrothermalVentLines::deserialize(&hydrothermal_vent_lines_file_contents)
        .context("Failed to read hydrothermal vent lines file")
}
