extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod bingo_game;

use anyhow::{Context, Result};
use bingo_game::BingoGame;
use std::env::current_dir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let bingo_game = read_bingo_game("files/input.txt").await?;

    let (winning_number, winning_board) =
        bingo_game.clone().play().context("There was no winner!")?;
    let winning_board_sum: u32 = winning_board
        .unselected_numbers()
        .iter()
        .map(|number| *number as u32)
        .sum();

    println!("Winning number:\t\t{}", winning_number);
    println!("Winning board sum:\t{:?}", winning_board_sum);
    println!(
        "Product:\t\t{}",
        (winning_number as u32) * winning_board_sum
    );

    let (last_winning_number, last_winning_board) = bingo_game
        .clone()
        .play_exhaustively()
        .context("There wasn't a last winner!")?;
    let last_winning_board_sum: u32 = last_winning_board
        .unselected_numbers()
        .iter()
        .map(|number| *number as u32)
        .sum();

    println!();
    println!("Last winning number:\t{}", last_winning_number);
    println!("Last winning board sum:\t{:?}", last_winning_board_sum);
    println!(
        "Product:\t\t{}",
        (last_winning_number as u32) * last_winning_board_sum
    );

    Ok(())
}

/// Reads the contents of the "diagnostic report" input file as a
/// newline-separated list of binary numbers.
async fn read_bingo_game(bingo_game_file_path: &str) -> Result<BingoGame> {
    let pwd = current_dir().context("Failed to read current working directory")?;
    let bingo_game_file_path_buf = pwd.join(bingo_game_file_path);

    let mut bingo_game_file = File::open(&bingo_game_file_path_buf)
        .await
        .with_context(|| {
            format!(
                "Failed to open file at path \"{}\"",
                bingo_game_file_path_buf.display()
            )
        })?;
    let mut raw_bingo_game_file_contents = vec![];

    bingo_game_file
        .read_to_end(&mut raw_bingo_game_file_contents)
        .await
        .with_context(|| {
            format!(
                "Failed to read file at path \"{}\"",
                bingo_game_file_path_buf.display()
            )
        })?;

    let bingo_game_file_contents = String::from_utf8_lossy(&raw_bingo_game_file_contents);

    BingoGame::deserialize(&bingo_game_file_contents).context("Failed to read bingo game file")
}
