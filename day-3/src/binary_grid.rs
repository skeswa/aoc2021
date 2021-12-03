use anyhow::{Context, Result};
use bit::BitAggregator;
pub use bit::{Bit, BitSequence};

#[derive(Debug, PartialEq)]
pub struct BinaryGrid {
    bits: Vec<Vec<Bit>>,
    width: usize,
}

impl BinaryGrid {
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

        Ok(BinaryGrid {
            bits: bits,
            width: width,
        })
    }

    pub fn least_common_bit_in_each_column(&self) -> BitSequence {
        (0..self.width)
            .map(|column_index| self.least_common_bit_in_column(column_index).unwrap())
            .collect::<BitSequence>()
    }

    pub fn most_common_bit_in_each_column(&self) -> BitSequence {
        (0..self.width)
            .map(|column_index| self.most_common_bit_in_column(column_index).unwrap())
            .collect::<BitSequence>()
    }

    fn aggregate_bits_in_column(&self, column_index: usize) -> BitAggregator {
        if column_index >= self.width {
            return BitAggregator::empty();
        }

        self.bits
            .iter()
            .map(|bits| bits[column_index])
            .map(BitAggregator::from)
            .reduce(BitAggregator::aggregate)
            .unwrap_or_else(BitAggregator::empty)
    }

    fn most_common_bit_in_column(&self, column_index: usize) -> Option<Bit> {
        self.aggregate_bits_in_column(column_index).most_common()
    }

    fn least_common_bit_in_column(&self, column_index: usize) -> Option<Bit> {
        self.aggregate_bits_in_column(column_index).least_common()
    }
}

mod bit {
    const ONE: char = '1';
    const ZERO: char = '0';

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Bit {
        One,
        Zero,
    }

    impl Bit {
        pub fn from(bit_char: char) -> Option<Bit> {
            match bit_char {
                ONE => Some(Bit::One),
                ZERO => Some(Bit::Zero),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct BitAggregator(usize, usize);

    impl BitAggregator {
        pub fn aggregate(a: BitAggregator, b: BitAggregator) -> BitAggregator {
            BitAggregator(a.0 + b.0, a.1 + b.1)
        }

        pub fn empty() -> BitAggregator {
            BitAggregator(0, 0)
        }

        pub fn from(bit: Bit) -> BitAggregator {
            match bit {
                Bit::One => BitAggregator(0, 1),
                Bit::Zero => BitAggregator(1, 0),
            }
        }

        pub fn least_common(&self) -> Option<Bit> {
            if self.0 < self.1 {
                Some(Bit::Zero)
            } else if self.1 < self.0 {
                Some(Bit::One)
            } else {
                None
            }
        }

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
