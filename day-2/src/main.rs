extern crate anyhow;
extern crate lazy_static;
extern crate regex;

mod movement;

use anyhow::{Context, Error, Result};
use movement::Movement;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let submarine_movements = read_submarine_movements("files/input.txt").await?;

    println!("# of movements: {}\n", (&submarine_movements).len());

    let aimless_position = submarine_movements
        .iter()
        .map(|movement| match movement {
            Movement::Down(magnitude) => (0, *magnitude),
            Movement::Up(magnitude) => (0, -1 * *magnitude),
            Movement::Forward(magnitude) => (*magnitude, 0),
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap_or((0, 0));

    println!("Aimless horizontal position:\t{}", aimless_position.0);
    println!("Aimless depth:\t\t\t{}", aimless_position.1);
    println!(
        "Product:\t\t\t{}\n",
        aimless_position.0 * aimless_position.1
    );

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;

    for submarine_movement in submarine_movements {
        match submarine_movement {
            Movement::Down(magnitude) => {
                aim += magnitude;
            }
            Movement::Up(magnitude) => {
                aim -= magnitude;
            }
            Movement::Forward(magnitude) => {
                horizontal_position += magnitude;
                depth += aim * magnitude;
            }
        }
    }

    println!("Horizontal position:\t\t{}", horizontal_position);
    println!("Depth:\t\t\t\t{}", depth);
    println!("Product:\t\t\t{}\n", horizontal_position * depth);

    Ok(())
}

/// Reads the contents of the "submarine movements" input file as a
/// newline-separated list of serialized movement commands.
async fn read_submarine_movements(
    submarine_movement_file_path: &str,
) -> Result<Vec<Movement>, Error> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let submarine_movement_file_path_buf = pwd.join(submarine_movement_file_path);

    let mut submarine_movement_file = File::open(&submarine_movement_file_path_buf)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                submarine_movement_file_path_buf.display()
            )
        })?;
    let mut raw_submarine_movement_file_contents = vec![];

    submarine_movement_file
        .read_to_end(&mut raw_submarine_movement_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                submarine_movement_file_path_buf.display()
            )
        })?;

    let submarine_movement_file_contents =
        String::from_utf8_lossy(&raw_submarine_movement_file_contents);

    let submarine_movements = submarine_movement_file_contents
        .lines()
        .map(|raw_submarine_movement| {
            raw_submarine_movement
                .parse::<Movement>()
                .with_context(|| format!("\"{}\" is not a valid movement", raw_submarine_movement))
        })
        .collect::<Result<Vec<Movement>>>()
        .context("Failed to parse submarine movements")?;

    Ok(submarine_movements)
}
