/// Center marker plugin
pub mod center_marker;

/// Commonly used stuff
pub mod prelude {
    #[doc(hidden)]
    pub use crate::center_marker::{CenterEntity, CenterMarkerPlugin};
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctest;
