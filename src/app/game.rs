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
        // If in confirm quit mode, handle Y/N/ESC
        if self.game_state.confirm_quit {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    return false; // Quit the game
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.game_state.confirm_quit = false; // Cancel quit
                    return true;
                }
                _ => return true, // Ignore other keys
            }
        }

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
            KeyCode::Left | KeyCode::Right => {
                // Switch to random sample (only works before typing starts)
                self.game_state.random_sample();
                true
            }
            KeyCode::Esc => {
                if self.game_state.first_input_time.is_some() && !self.game_state.game_over {
                    // Typing in progress - restart current sample
                    self.game_state.restart_current();
                    true
                } else {
                    // Not started or game over - show quit confirmation
                    self.game_state.confirm_quit = true;
                    true
                }
            }
            _ => true,
        }
    }
}
