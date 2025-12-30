use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::{Duration, Instant};

use super::{ui, GameState};

pub struct Game {
    game_state: GameState,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Game {
    pub fn new(terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Self {
        Game {
            game_state: GameState::new(),
            terminal,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        loop {
            self.terminal.draw(|f| ui::draw(f, &self.game_state))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if !self.handle_input(key.code) {
                        return Ok(());
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char(c) => {
                self.game_state.handle_input(c);
                true
            }
            KeyCode::Tab => {
                self.game_state.handle_input('\t');
                true
            }
            KeyCode::Backspace => {
                self.game_state.handle_backspace();
                true
            }
            KeyCode::Enter => {
                if self.game_state.game_over {
                    self.game_state.reset();
                } else {
                    self.game_state.handle_input('\n');
                }
                true
            }
            KeyCode::Esc => false,
            _ => true,
        }
    }
}
