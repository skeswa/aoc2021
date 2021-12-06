use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

    /// Plays Bingo, returning a tuple of the **first** winning number and the
    /// [BingoGameBoard] that won.
    pub fn play(&mut self) -> Option<(u8, BingoGameBoard)> {
        for number in self.number_selections.iter() {
            for board in self.boards.iter_mut() {
                board.select(*number);

                if board.has_bingo {
                    return Some((*number, board.clone()));
                }
            }
        }

        None
    }

    /// Plays Bingo, returning a tuple of the **last** winning number and the
    /// [BingoGameBoard] that won.
    pub fn play_exhaustively(&mut self) -> Option<(u8, BingoGameBoard)> {
        let mut boards = self.boards.clone();
        let mut number_index = 0;

        while boards.len() > 0 && number_index < self.number_selections.len() {
            let number = self.number_selections[number_index];

            let mut board_index = 0;
            while board_index < boards.len() {
                let board = &mut boards[board_index];
                board.select(number);

                if board.has_bingo {
                    if boards.len() == 1 {
                        return Some((number, boards[0].clone()));
                    }

                    boards.remove(board_index);
                } else {
                    board_index += 1;
                }
            }

            number_index += 1;
        }

        None
    }
}

/// Represents a single bingo game.
#[derive(Clone, Debug, PartialEq)]
pub struct BingoGameBoard {
    /// `true` if this is a winning [BingoGameBoard].
    has_bingo: bool,
    /// Numbers in this [BingoGameBoard] indexed by their respective indices with in [numbers].
    index_by_number: HashMap<u8, usize>,
    /// Sequence of numbers selected for this bingo game.
    numbers: Vec<u8>,
    /// Indices of all selected numbers in this [BingoGameBoard].
    selected_number_indices: Vec<usize>,
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
            has_bingo: false,
            index_by_number,
            numbers,
            selected_number_indices: Vec::new(),
        })
    }

    /// Returns a [Vec] containing all of the unselected numbers on this
    /// [BingoGameBoard].
    pub fn unselected_numbers(&self) -> Vec<u8> {
        let selected_number_indices =
            HashSet::<usize>::from_iter(self.selected_number_indices.iter().cloned());

        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !selected_number_indices.contains(i))
            .map(|(_, number)| *number)
            .collect::<Vec<u8>>()
    }

    /// Returns `true` if this [BingoGameBoard] has five numbers selected in a
    /// row.
    fn has_horizontal_stretch(&self) -> bool {
        let mut number_of_consecutive_indices = 0;
        let mut previous_index = 0;

        for index in self.selected_number_indices.iter() {
            if number_of_consecutive_indices > 0 &&
            // Reset the concescutive count when we go to the next row.
            index % 5 != 0 && (index - previous_index) == 1
            {
                number_of_consecutive_indices += 1;
            } else {
                number_of_consecutive_indices = 1;
            }

            if number_of_consecutive_indices == 5 {
                return true;
            }

            previous_index = *index;
        }

        return false;
    }

    /// Returns `true` if this [BingoGameBoard] has five numbers selected in a
    /// column.
    fn has_vertical_stretch(&self) -> bool {
        let mut column_totals: [usize; 5] = [0; 5];
        let mut previous_column_indices: [usize; 5] = [0; 5];

        for index in self.selected_number_indices.iter() {
            let column_index = *index % 5;

            // Reset the column total to 0 when column values are not
            // consecutive.
            if column_totals[column_index] != 0 && index - previous_column_indices[column_index] > 5
            {
                column_totals[column_index] = 0;
            }

            column_totals[column_index] += 1;
            previous_column_indices[column_index] = *index;
            if column_totals[column_index] == 5 {
                return true;
            }
        }

        return false;
    }

    /// Selects the specified `number` on this [BingoGameBoard].
    fn select(&mut self, number: u8) {
        let index = self.index_by_number.get(&number);
        if index.is_none() {
            return;
        }

        self.selected_number_indices.push(*index.unwrap());
        self.selected_number_indices.sort();

        if !self.has_bingo && (self.has_horizontal_stretch() || self.has_vertical_stretch()) {
            self.has_bingo = true;
        }
    }
}
