#[path = "common/ui.rs"]
mod ui;

use bracket_terminal::prelude::*;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use ratatui::Terminal;
use ratatui_bracket_terminal::bterm::BTermBackendManager;
use ratatui_bracket_terminal::BasicColourConverter;
use std::collections::VecDeque;
use ui::{render_ui, update_data};

struct State {
    rng: ThreadRng,
    backend_man: BTermBackendManager<BasicColourConverter>,
    data: VecDeque<u64>,
}

impl State {
    fn new() -> Self {
        Self {
            rng: thread_rng(),
            backend_man: BTermBackendManager::new(Default::default()),
            data: VecDeque::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        update_data(&mut self.data, &mut self.rng);
        Terminal::new(self.backend_man.get(ctx))
            .expect("failed to make ratatui terminal")
            .draw(|f| render_ui(f, &mut self.data))
            .expect("failed to render ui");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket-Ratatui test with BTerm")
        .with_vsync(false)
        .build()?;
    let gs: State = State::new();
    main_loop(context, gs)
}
