extern crate anyhow;
extern crate tokio;

mod binary_grid;

use anyhow::{Context, Result};
use binary_grid::{BinaryGrid, BinaryGridCullOptions, Bit};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let diagnostic_report = read_diagnostic_report("files/input.txt").await?;

    let epsilon_rate: u32 = diagnostic_report.least_common_bit_in_each_column().into();
    let gamma_rate: u32 = diagnostic_report.most_common_bit_in_each_column().into();

    println!("Epsilon rate:\t{}", epsilon_rate);
    println!("Gamma rate:\t{}", gamma_rate);
    println!("Product:\t{}", epsilon_rate * gamma_rate);

    let co2_scrubber_rating = co2_scrubber_rating_of(&diagnostic_report)
        .context("Failed to read CO2 generator rating")?;
    let oxygen_generator_rating = oxygen_generator_rating_of(&diagnostic_report)
        .context("Failed to read oxygen generator rating")?;

    println!();
    println!("CO2 scrubber rating:\t\t{}", co2_scrubber_rating);
    println!("Oxygen generator rating:\t{}", oxygen_generator_rating);
    println!(
        "Product:\t\t\t{}",
        co2_scrubber_rating * oxygen_generator_rating
    );

    Ok(())
}

/// Returns the CO2 scrubber rating of the specified `diagnostic_report`,
/// returning [Option::None] if no such rating exists.
fn co2_scrubber_rating_of(diagnostic_report: &BinaryGrid) -> Option<u32> {
    let mut column_index = 0;
    let mut culled_diagnostic_report = diagnostic_report.clone();
    while column_index < culled_diagnostic_report.columns() && culled_diagnostic_report.rows() > 1 {
        let least_common_bit = culled_diagnostic_report
            .least_common_bit_in_column(column_index)
            .unwrap_or(Bit::Zero);

        culled_diagnostic_report = culled_diagnostic_report.cull(BinaryGridCullOptions {
            rows_with_bits_matching: least_common_bit,
            at_index: column_index,
        });

        column_index = column_index + 1;
    }

    culled_diagnostic_report
        .row(0)
        .map(|row| -> u32 { row.into() })
}

/// Returns the Oxygen generator rating of the specified `diagnostic_report`,
/// returning [Option::None] if no such rating exists.
fn oxygen_generator_rating_of(diagnostic_report: &BinaryGrid) -> Option<u32> {
    let mut column_index = 0;
    let mut culled_diagnostic_report = diagnostic_report.clone();
    while column_index < culled_diagnostic_report.columns() && culled_diagnostic_report.rows() > 1 {
        let most_common_bit = culled_diagnostic_report
            .most_common_bit_in_column(column_index)
            .unwrap_or(Bit::One);

        culled_diagnostic_report = culled_diagnostic_report.cull(BinaryGridCullOptions {
            rows_with_bits_matching: most_common_bit,
            at_index: column_index,
        });

        column_index = column_index + 1;
    }

    culled_diagnostic_report
        .row(0)
        .map(|row| -> u32 { row.into() })
}

/// Reads the contents of the "diagnostic report" input file as a
/// newline-separated list of binary numbers.
async fn read_diagnostic_report(diagnostic_report_file_path: &str) -> Result<BinaryGrid> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let diagnostic_report_file_path_buf = pwd.join(diagnostic_report_file_path);

    let mut diagnostic_report_file = File::open(&diagnostic_report_file_path_buf)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                diagnostic_report_file_path_buf.display()
            )
        })?;
    let mut raw_diagnostic_report_file_contents = vec![];

    diagnostic_report_file
        .read_to_end(&mut raw_diagnostic_report_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                diagnostic_report_file_path_buf.display()
            )
        })?;

    let diagnostic_report_file_contents =
        String::from_utf8_lossy(&raw_diagnostic_report_file_contents);

    BinaryGrid::deserialize(&diagnostic_report_file_contents)
        .context("Failed to interpret diagnostic report as a serialized binary grid")
}
