extern crate anyhow;
extern crate tokio;

use anyhow::{Context, Error, Result};
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let sonar_sweep_depths = read_sonar_sweep_depths("files/input.txt").await?;

    println!("Hello, world!\n{:?}", sonar_sweep_depths);

    Ok(())
}

/// Reads the contents of the "sonar sweep" input file as a newline-separated
/// list of integer depths.
async fn read_sonar_sweep_depths(sonar_sweep_file_path: &str) -> Result<Vec<i32>, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let sonar_sweep_file_path_buf = pwd.join(sonar_sweep_file_path);

    let mut sonar_sweep_file = File::open(&sonar_sweep_file_path_buf)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                sonar_sweep_file_path_buf.display()
            )
        })?;
    let mut raw_sonar_sweep_file_contents = vec![];

    sonar_sweep_file
        .read_to_end(&mut raw_sonar_sweep_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                sonar_sweep_file_path_buf.display()
            )
        })?;

    let sonar_sweep_file_contents = String::from_utf8_lossy(&raw_sonar_sweep_file_contents);

    let sonar_sweep_depths = sonar_sweep_file_contents
        .split("\n")
        .map(|raw_depth| {
            raw_depth
                .parse::<i32>()
                .with_context(|| format!("\"{}\" is not a valid integer", raw_depth))
        })
        .collect::<Result<Vec<i32>>>()
        .context("Failed to parse sonar sweep depths")?;

    Ok(sonar_sweep_depths)
}
