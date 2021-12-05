use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    /// Regular expression designed to match empty lines.
    static ref EMPTY_LINE_PATTERN: Regex =
    Regex::new( r"\r?\n\r?\n").unwrap();

    /// Regular expression designed to match numbers.
    static ref NUMBER_PATTERN: Regex =
        Regex::new( r"\d+").unwrap();
}

/// Represents a single bingo game.
#[derive(Clone, Debug, PartialEq)]
pub struct BingoGame {
    /// Game boards in this bingo game.
    boards: Vec<BingoGameBoard>,
    /// Sequence of numbers selected for this bingo game.
    number_selections: Vec<u8>,
}

impl BingoGame {
    /// Interprets an empty line-delimited [str] of bingo game data as a
    /// [BingoGame].
    pub fn deserialize(serialized_bingo_game: &str) -> Result<Self> {
        let line_groups = EMPTY_LINE_PATTERN
            .split(serialized_bingo_game)
            .collect::<Vec<&str>>();

        if line_groups.len() < 2 {
            return Err(anyhow!("Serialized bingo game had no boards"));
        }

        let serialized_number_selections = line_groups[0];
        let number_selections = NUMBER_PATTERN
            .find_iter(serialized_number_selections)
            .map(|raw_number| {
                raw_number
                    .as_str()
                    .parse::<u8>()
                    .with_context(|| format!("\"{}\" is not a valid number", raw_number.as_str()))
            })
            .collect::<Result<Vec<u8>>>()
            .context("Failed deserialize number selections")?;

        let boards = line_groups
            .iter()
            .skip(1)
            .map(|line_group| BingoGameBoard::deserialize(line_group))
            .collect::<Result<Vec<BingoGameBoard>>>()
            .context("Failed deserialize game boards")?;

        Ok(BingoGame {
            boards,
            number_selections,
        })
    }
}

/// Represents a single bingo game.
#[derive(Clone, Debug, PartialEq)]
pub struct BingoGameBoard {
    /// Numbers in this [BingoGameBoard] indexed by their respective indices with in [numbers].
    index_by_number: HashMap<u8, usize>,
    /// Sequence of numbers selected for this bingo game.
    numbers: Vec<u8>,
}

impl BingoGameBoard {
    /// Interprets a 5x5 grid of numbers as a [BingoGameBoard].
    fn deserialize(serialized_bingo_game_board: &str) -> Result<Self> {
        let numbers = NUMBER_PATTERN
            .find_iter(serialized_bingo_game_board)
            .map(|raw_number| {
                raw_number
                    .as_str()
                    .parse::<u8>()
                    .with_context(|| format!("\"{}\" is not a valid number", raw_number.as_str()))
            })
            .collect::<Result<Vec<u8>>>()
            .context("Failed to read numbers")?;

        if numbers.len() != 25 {
            return Err(anyhow!(
                "Serialized game board had {} numbers (not 25)",
                numbers.len()
            ));
        }

        let index_by_number = numbers
            .iter()
            .enumerate()
            .map(|(i, number)| (*number, i))
            .collect::<HashMap<u8, usize>>();

        Ok(BingoGameBoard {
            index_by_number,
            numbers,
        })
    }
}
