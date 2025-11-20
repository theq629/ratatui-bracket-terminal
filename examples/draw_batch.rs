#[path = "common/ui.rs"]
mod ui;

use bracket_terminal::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ratatui::Terminal;
use ratatui_bracket_terminal::draw_batch::DrawBatchBackend;
use ratatui_bracket_terminal::BasicColourConverter;
use std::collections::VecDeque;
use ui::{render_ui, update_data};

struct State {
    rng: SmallRng,
    term: Terminal<DrawBatchBackend<BasicColourConverter>>,
    data: VecDeque<u64>,
}

impl State {
    fn new() -> Self {
        Self {
            rng: SmallRng::seed_from_u64(0),
            term: Terminal::new(DrawBatchBackend::new(Default::default()))
                .expect("failed to make ratatui terminal"),
            data: VecDeque::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        update_data(&mut self.data, &mut self.rng);
        self.term.backend_mut().update(ctx);
        self.term
            .draw(|f| render_ui(f, &mut self.data))
            .expect("failed to render ui");
        self.term
            .backend_mut()
            .batch_mut()
            .submit(0)
            .expect("failed to submit batch");
        render_draw_buffer(ctx).expect("failed to render draw buffer");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket-Ratatui test with batch")
        .with_vsync(false)
        .build()?;
    let gs: State = State::new();
    main_loop(context, gs)
}
