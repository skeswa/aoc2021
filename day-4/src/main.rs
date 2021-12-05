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
    let bingo_game = read_bingo_game("files/sample.txt").await?;

    println!("Hello, world! {:?}", bingo_game);

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
