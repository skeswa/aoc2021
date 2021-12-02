extern crate anyhow;
extern crate itertools;
extern crate tokio;

use anyhow::{Context, Error, Result};
use itertools::izip;
use std::env::current_dir;
use std::iter;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let sonar_sweep_depths = read_sonar_sweep_depths("files/input.txt").await?;

    let number_of_depth_increases = number_of_increases_in(&sonar_sweep_depths);
    println!("Number of depth increases: {}", number_of_depth_increases);

    let three_measurement_sums = triplewise(sonar_sweep_depths)
        .filter(|(maybe_first, maybe_second, _)| !maybe_first.is_none() && !maybe_second.is_none())
        .map(|(maybe_first, maybe_second, third)| {
            maybe_first.unwrap_or(0) + maybe_second.unwrap_or(0) + third
        })
        .collect::<Vec<i32>>();

    let three_measurement_sum_increases = number_of_increases_in(&three_measurement_sums);
    println!(
        "Number of three-measurement sum increases: {}",
        three_measurement_sum_increases
    );

    Ok(())
}

/// Returns the number of increases in the given `sequence` of integers.
fn number_of_increases_in<'a, I>(sequence: I) -> usize
where
    I: IntoIterator<Item = &'a i32> + Clone,
{
    pairwise(sequence)
        .filter(|(maybe_prev, next)| match maybe_prev {
            Some(prev) => next > prev,
            _ => false,
        })
        .map(|(_, next)| next)
        .count()
}

/// Returns a new [Iterator] that places each element of the given iterator on
/// the right side of a tuple, placing the element before to its left
/// (e.g. `(prev, next)`).
fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = iter::once(None).chain(right.clone().into_iter().map(Some));
    left.zip(right)
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
        .lines()
        .map(|raw_depth| {
            raw_depth
                .parse::<i32>()
                .with_context(|| format!("\"{}\" is not a valid integer", raw_depth))
        })
        .collect::<Result<Vec<i32>>>()
        .context("Failed to parse sonar sweep depths")?;

    Ok(sonar_sweep_depths)
}

/// Returns a new [Iterator] that places each element of the given iterator on
/// the right side of a tuple, placing the two elements before to its left
/// (e.g. `(2 before, 1 before, element)`).
fn triplewise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, Option<I::Item>, I::Item)>
where
    I: IntoIterator + Clone,
{
    let middle = iter::once(None).chain(right.clone().into_iter().map(Some));
    let left = iter::once(None).chain(iter::once(None).chain(right.clone().into_iter().map(Some)));

    izip!(left, middle, right)
}
