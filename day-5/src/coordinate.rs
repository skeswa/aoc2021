use std::{collections::HashMap, fmt};

/// Represents a point in space.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinate {
    /// X-component of this [Coordinate].
    pub x: i32,
    /// Y-component of this [Coordinate].
    pub y: i32,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Represents multiple points in space.
#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates(Vec<Coordinate>);

impl Coordinates {
    /// Returns a [HashMap] relating each [Coordinate] to a count of how many
    /// copies of that [Coordinate] exist in this [Coordinates].
    pub fn aggregate(&self) -> HashMap<Coordinate, usize> {
        let mut coordinate_counts = HashMap::<Coordinate, usize>::new();

        for coordinate in self.0.iter() {
            coordinate_counts.insert(
                *coordinate,
                *coordinate_counts.get(coordinate).unwrap_or(&0) + 1,
            );
        }

        coordinate_counts
    }
}

impl From<Vec<Coordinate>> for Coordinates {
    fn from(coordinates: Vec<Coordinate>) -> Self {
        Coordinates(coordinates)
    }
}

impl FromIterator<Coordinate> for Coordinates {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Coordinates(iter.into_iter().collect())
    }
}

impl FromIterator<Vec<Coordinate>> for Coordinates {
    fn from_iter<T: IntoIterator<Item = Vec<Coordinate>>>(iter: T) -> Self {
        Coordinates(iter.into_iter().flatten().collect())
    }
}

impl FromIterator<Coordinates> for Coordinates {
    fn from_iter<T: IntoIterator<Item = Coordinates>>(iter: T) -> Self {
        Coordinates(
            iter.into_iter()
                .map(|coordinates| coordinates.0)
                .flatten()
                .collect(),
        )
    }
}
