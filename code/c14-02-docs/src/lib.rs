//! # My Crate
//!
//! `my_crate` is a collection of utility functions to make certain calculations more convenient.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let five = 5;
/// assert_eq!(mycrate::add_one(five), 6);
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
}

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
