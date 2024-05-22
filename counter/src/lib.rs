#[cfg(feature = "counter")]
mod engine;

#[cfg(feature = "counter")]
pub use engine::count;
