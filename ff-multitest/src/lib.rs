#[cfg(feature = "count")]
mod engine;

#[cfg(feature = "count")]
pub use engine::count_all;
