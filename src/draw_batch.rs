//! Backends which draw to a Bracket [DrawBatch].

use crate::colours::ColourConverter;
use bracket_terminal::prelude::{BTerm, ColorPair, DrawBatch, Point};
use object_pool::Reusable;
use ratatui::backend::{Backend, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Rect, Size};
use std::io;

/// Ratatui [Backend] which draws to a Bracket [DrawBatch] which it creates.
///
/// With this interface the calling code needs to manually call [Self::update()]. However
/// - The backend is static, and especially does not maintain any borrow on Bracket state.
/// - You can keep a Ratatui [Terminal][ratatui::terminal::Terminal] on this backend and reuse it
///   for each tick.
pub struct DrawBatchBackend<C> {
    colours: C,
    cursor_pos: (u16, u16),
    batch: Reusable<'static, DrawBatch>,
    size: Rect,
    window_size: WindowSize,
}

impl<C> DrawBatchBackend<C> {
    pub fn new(colours: C) -> Self {
        Self {
            colours,
            cursor_pos: (0, 0),
            batch: DrawBatch::new(),
            size: Rect::new(0, 0, 0, 0),
            window_size: WindowSize {
                columns_rows: Size {
                    width: 0,
                    height: 0,
                },
                pixels: Size {
                    width: 0,
                    height: 0,
                },
            },
        }
    }

    pub fn batch(&self) -> &DrawBatch {
        &self.batch
    }

    pub fn batch_mut(&mut self) -> &mut DrawBatch {
        &mut self.batch
    }

    /// Update to match the Bracket [BTerm] state (size information). Needs to be called on each
    /// tick before any use of the backend, to ensure consistent state.
    pub fn update(&mut self, bterm: &BTerm) {
        let (width, height) = bterm.get_char_size();
        self.size = Rect::new(0, 0, width.try_into().unwrap(), height.try_into().unwrap());
        self.window_size = WindowSize {
            columns_rows: Size {
                width: width as u16,
                height: height as u16,
            },
            pixels: Size {
                width: bterm.width_pixels as u16,
                height: bterm.height_pixels as u16,
            },
        };
    }
}

impl<C> Backend for DrawBatchBackend<C>
where
    C: ColourConverter,
{
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            if !cell.skip {
                self.batch.set(
                    Point::new(x, y),
                    ColorPair::new(
                        self.colours.convert_fg(cell.fg, cell.modifier),
                        self.colours.convert_bg(cell.bg, cell.modifier),
                    ),
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

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        Ok(self.cursor_pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.cursor_pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.batch.cls();
        Ok(())
    }

    fn size(&self) -> io::Result<Rect> {
        Ok(self.size)
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        Ok(self.window_size)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
