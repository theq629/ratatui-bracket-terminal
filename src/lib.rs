//! [Ratatui][ratatui] backends for [Bracket Terminal][bracket_terminal].
//!
//! This crate does not currently provide any special support for rendering the Ratatui cursor; if
//! you want to show the cursor then the calling code needs to draw it.
//!
//! Note that Bracket (at least with normal fonts) is strictly extended ASCII / CP437, whereas
//! Ratatui defaults to unicode for box drawing etc.. You will likely need to set such characters
//! yourself.

pub mod bterm;
mod colours;
pub mod draw_batch;

pub use colours::{BasicColourConverter, ColourConverter};
