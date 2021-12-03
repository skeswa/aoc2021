extern crate anyhow;
extern crate tokio;

mod binary_grid;

use anyhow::{Context, Result};
use binary_grid::BinaryGrid;
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

    Ok(())
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
