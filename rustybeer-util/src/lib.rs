//! Utilities for working with `rustybeer`:
//! * List of beer styles that can be indexed and requested
//! * List of hops and their AA%
//! * Units conversions from strings

pub mod beer_styles;
pub mod conversions;
pub mod hops;

#[macro_export]
macro_rules! relative_eq_eps {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        approx::Relative::default().epsilon(1e-4)$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        approx::Relative::default().epsilon(1e-4)$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! assert_relative_eq {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        approx::__assert_approx!(relative_eq_eps, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        approx::__assert_approx!(relative_eq_eps, $given, $expected $(, $opt = $val)*)
    };
}
