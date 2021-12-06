use crate::coordinate::Coordinates;

use anyhow::Result;

/// Anything that can be traced in space.
pub trait Traceable {
    // Returns a [Vec] of all the coordinates covered by this
    /// [Traceable], returning [Err] if such coordinates cannot be enumerated.
    fn trace(&self) -> Result<Coordinates>;
}
