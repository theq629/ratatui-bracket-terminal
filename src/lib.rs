//! [Ratatui][ratatui] backends for [Bracket Terminal][bracket_terminal].
//!
//! This crate does not currently provide any special support for rendering the Ratatui cursor; if
//! you want to show the cursor then the calling code needs to draw it.

pub mod bterm;
mod colours;
pub mod draw_batch;

pub use colours::{BasicColourConverter, ColourConverter};
