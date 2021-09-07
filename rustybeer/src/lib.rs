//! Utilities for working with `rustybeer`:
//! * List of beer styles that can be indexed and requested
//! * List of hops and their AA%
//! * List of yeasts and their properties
//! * Units conversions from strings

pub mod abv_calories;
pub mod beer_styles;
pub mod calculators;
pub mod conversions;
pub mod hops;
pub mod yeasts;

pub use measurements;

mod macros;
mod strings;
