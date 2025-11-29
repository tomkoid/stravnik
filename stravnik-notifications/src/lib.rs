pub mod errors;
pub mod formatting;
pub mod ntfy;

#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "matrix")]
pub mod matrix;
