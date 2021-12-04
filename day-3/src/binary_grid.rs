use anyhow::{Context, Result};
use bit::BitAggregator;
pub use bit::{Bit, BitSequence};

/// 2D grid of ones and zeroes.
#[derive(Debug, PartialEq)]
pub struct BinaryGrid {
    /// 2D [Vec] of [Bit] instances.
    bits: Vec<Vec<Bit>>,
    /// Number of columns in this grid.
    width: usize,
}

impl BinaryGrid {
    /// Interprets a newline-delimited [str] of binary numbers as a
    /// [BinaryGrid].
    pub fn deserialize(serialized_binary_grid: &str) -> Result<BinaryGrid> {
        let bits = serialized_binary_grid
            .lines()
            .map(|line| {
                line.chars()
                    .map(|bit_char| {
                        Bit::from(bit_char)
                            .with_context(|| format!("\"{}\" is not a valid bit char", bit_char))
                    })
                    .collect::<Result<Vec<Bit>>>()
            })
            .collect::<Result<Vec<Vec<Bit>>>>()
            .with_context(|| {
                format!(
                    "\"{}\" is not a valid serialized binary grid",
                    serialized_binary_grid
                )
            })?;

        let width = bits.iter().map(|row| row.len()).max().unwrap_or(0);

        Ok(BinaryGrid { bits, width })
    }

    /// Returns a [BitSequence] of the least common bit in each column.
    pub fn least_common_bit_in_each_column(&self) -> BitSequence {
        (0..self.width)
            .map(|column_index| self.least_common_bit_in_column(column_index).unwrap())
            .collect::<BitSequence>()
    }

    /// Returns a [BitSequence] of the most common bit in each column.
    pub fn most_common_bit_in_each_column(&self) -> BitSequence {
        (0..self.width)
            .map(|column_index| self.most_common_bit_in_column(column_index).unwrap())
            .collect::<BitSequence>()
    }

    /// Summarizes an entire column of [Bit] in a [BitAggregator], returning
    /// the [BitAggregator] thereafter.
    fn aggregate_bits_in_column(&self, column_index: usize) -> BitAggregator {
        if column_index >= self.width {
            return BitAggregator::zero();
        }

        self.bits
            .iter()
            .map(|bits| bits[column_index])
            .map(BitAggregator::from)
            .reduce(BitAggregator::combine)
            .unwrap_or_else(BitAggregator::zero)
    }

    /// Returns the most common [Bit] in the column indicated by `column_index`,
    /// returning [Option::None] if no such [Bit] exists.
    fn most_common_bit_in_column(&self, column_index: usize) -> Option<Bit> {
        self.aggregate_bits_in_column(column_index).most_common()
    }

    /// Returns the least common [Bit] in the column indicated by
    /// `column_index`, returning [Option::None] if no such [Bit] exists.
    fn least_common_bit_in_column(&self, column_index: usize) -> Option<Bit> {
        self.aggregate_bits_in_column(column_index).least_common()
    }
}

/// Module encupsulating bitwise logic used by the [super::BinaryGrid].
mod bit {
    /// Character representing bitwise one.
    const ONE: char = '1';

    /// Character representing bitwise zero.
    const ZERO: char = '0';

    /// Enumerates both possible vlaues for a bit.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Bit {
        /// Enum representation of a bitwise one.
        One,
        /// Enum representation of a bitwise zero.
        Zero,
    }

    impl Bit {
        /// Attempts to interpret the given [char] as a [Bit], returning
        /// [Option::None] if interpretation fails.
        pub fn from(bit_char: char) -> Option<Bit> {
            match bit_char {
                ONE => Some(Bit::One),
                ZERO => Some(Bit::Zero),
                _ => None,
            }
        }
    }

    /// Utility type used to summarize [Bit] collections.
    ///
    /// The `0` field refers to the number of [Bit::Zero] instances in a
    /// collection.
    ///
    /// The `1` field refers to the number of [Bit::One] instances in a
    /// collection.
    #[derive(Debug, PartialEq)]
    pub struct BitAggregator(usize, usize);

    impl BitAggregator {
        /// Combines two instances of [BitAggregator] by summing their
        /// respective fields within a new [BitAggregator] instance.
        pub fn combine(a: BitAggregator, b: BitAggregator) -> BitAggregator {
            BitAggregator(a.0 + b.0, a.1 + b.1)
        }

        /// Creates a new [BitAggregator] both its fields set to `0`.
        pub fn zero() -> BitAggregator {
            BitAggregator(0, 0)
        }

        /// Returns the [Bit] indicated to be the least common according to this
        /// [BitAggregator], returning [Option::None] if no such [Bit] is
        /// indicated.
        pub fn least_common(&self) -> Option<Bit> {
            if self.0 < self.1 {
                Some(Bit::Zero)
            } else if self.1 < self.0 {
                Some(Bit::One)
            } else {
                None
            }
        }

        /// Returns the [Bit] indicated to be the most common according to this
        /// [BitAggregator], returning [Option::None] if no such [Bit] is
        /// indicated.
        pub fn most_common(&self) -> Option<Bit> {
            if self.0 > self.1 {
                Some(Bit::Zero)
            } else if self.1 > self.0 {
                Some(Bit::One)
            } else {
                None
            }
        }
    }

    impl From<Bit> for BitAggregator {
        fn from(bit: Bit) -> Self {
            match bit {
                Bit::One => BitAggregator(0, 1),
                Bit::Zero => BitAggregator(1, 0),
            }
        }
    }

    /// Represents an ordered collection of [Bit] instances.
    #[derive(Debug, PartialEq)]
    pub struct BitSequence(Vec<Bit>);

    impl Into<u32> for BitSequence {
        fn into(self) -> u32 {
            self.0
                .iter()
                .rev()
                .enumerate()
                .map(|(i, bit)| match bit {
                    Bit::One => u32::pow(2, i as u32),
                    Bit::Zero => 0,
                })
                .sum()
        }
    }

    impl FromIterator<Bit> for BitSequence {
        fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
            BitSequence(iter.into_iter().collect::<Vec<Bit>>())
        }
    }
}
