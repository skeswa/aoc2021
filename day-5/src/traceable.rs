use crate::coordinate::Coordinates;

use anyhow::Result;

/// Anything that can be traced in space.
pub trait Traceable {
    /// Returns `true` if this [HydrothermalVentLine] can be traced.
    ///
    /// Parameters:
    /// *   `are_diagonals_allowed`\
    ///     Is `true` if diagonal lines are considered to be traceable.
    fn can_trace(&self, are_diagonals_allowed: bool) -> bool;

    // Returns a [Vec] of all the coordinates covered by this
    /// [Traceable], returning [Err] if such coordinates cannot be enumerated.
    ///
    /// Parameters:
    /// *   `are_diagonals_allowed`\
    ///     Is `true` if diagonal lines are considered to be traceable.
    fn trace(&self, are_diagonals_allowed: bool) -> Result<Coordinates>;
}
