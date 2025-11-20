//! Backends which draw directly to a Bracket [BTerm].
//!
//! This may be convenient if drawing to Bracket Terminal only through Ratatui. Otherwise, the
//! [draw batch interface][crate::draw_batch] is probably more convenient.
//!
//! ```rust,ignore
//! fn tick(&mut self, ctx: &mut BTerm) {
//!     Terminal::new(self.backend_man.get(ctx))
//!         .expect("failed to make ratatui terminal")
//!         .draw(|f| ...)
//!         .expect("failed to render ui");
//! }
//! ```

use crate::colours::ColourConverter;
use bracket_terminal::prelude::BTerm;
use ratatui::backend::{Backend, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use std::io;

/// Manager that can create Ratatui [Backend]s for a Bracket [`BTerm`].
///
/// Calling code should keep a manager and reuse it between ticks. This interface has two
/// limitations:
/// - The backend will maintain an exclusive borrow on the [BTerm].
/// - The calling code will have to create a new Ratatui [Terminal][ratatui::terminal::Terminal] for each
///   draw.
pub struct BTermBackendManager<C> {
    colours: C,
    cursor_pos: Position,
}

impl<C> BTermBackendManager<C> {
    pub fn new(colours: C) -> Self {
        Self {
            colours,
            cursor_pos: Position::new(0, 0),
        }
    }

    /// Get a Ratatui [Backend] for a [BTerm].
    pub fn get<'a, 'b>(&'a mut self, bterm: &'b mut BTerm) -> BTermBackend<'a, 'b, C> {
        BTermBackend {
            bterm,
            colours: &self.colours,
            cursor_pos: &mut self.cursor_pos,
        }
    }
}

/// A Ratatui [Backend] drawing directly to a Bracket [BTerm], which it maintains a mutable borrow
/// of for its lifetime.
///
/// Use [BTermBackendManager] to create an instance.
pub struct BTermBackend<'a, 'b, C> {
    bterm: &'b mut BTerm,
    colours: &'a C,
    cursor_pos: &'a mut Position,
}

impl<C> Backend for BTermBackend<'_, '_, C>
where
    C: ColourConverter,
{
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            if !cell.skip {
                self.bterm.set(
                    x,
                    y,
                    self.colours.convert_fg(cell.fg, cell.modifier),
                    self.colours.convert_bg(cell.bg, cell.modifier),
                    cell.symbol().chars().next().unwrap() as u32,
                );
            }
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(*self.cursor_pos)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, pos: P) -> io::Result<()> {
        *self.cursor_pos = pos.into();
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.bterm.cls();
        Ok(())
    }

    fn size(&self) -> io::Result<Size> {
        let (width, height) = self.bterm.get_char_size();
        Ok(Size::new(width as u16, height as u16))
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        let (width, height) = self.bterm.get_char_size();
        Ok(WindowSize {
            columns_rows: Size {
                width: width as u16,
                height: height as u16,
            },
            pixels: Size {
                width: self.bterm.width_pixels as u16,
                height: self.bterm.height_pixels as u16,
            },
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
