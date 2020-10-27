/// Wrapper for `approx::assert_relative_eq` with `epsilon = 1e-4` as default
/// since default epsilon used by `approx` is too accurate
#[macro_export]
macro_rules! assert_approx {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        approx::assert_relative_eq!($given, $expected, epsilon = 1e-4 $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        approx::assert_relative_eq!($given, $expected, epsilon = 1e-4 $(, $opt = $val)*)
    };
}
