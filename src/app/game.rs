use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::{Duration, Instant};

use super::state::Screen;
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

    /// Returns `false` to quit the game.
    fn handle_input(&mut self, key: KeyCode) -> bool {
        // Quit confirmation intercepts everything else
        if self.game_state.confirm_quit {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') => return false,
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.game_state.confirm_quit = false;
                }
                _ => {}
            }
            return true;
        }

        match self.game_state.screen {
            Screen::Menu => self.handle_menu_input(key),
            Screen::Typing => self.handle_typing_input(key),
        }
    }

    fn handle_menu_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up | KeyCode::Char('k') => self.game_state.menu_up(),
            KeyCode::Down | KeyCode::Char('j') => self.game_state.menu_down(),
            KeyCode::Enter => self.game_state.select_menu_language(),
            KeyCode::Esc | KeyCode::Char('q') => self.game_state.confirm_quit = true,
            _ => {}
        }
        true
    }

    fn handle_typing_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char(c) => self.game_state.handle_input(c),
            KeyCode::Tab => self.game_state.handle_input('\t'),
            KeyCode::Backspace => self.game_state.handle_backspace(),
            KeyCode::Enter => {
                if self.game_state.game_over {
                    self.game_state.reset();
                } else {
                    self.game_state.handle_input('\n');
                }
            }
            // Switch sample — only before typing starts (handled in state)
            KeyCode::Left | KeyCode::Right => self.game_state.random_sample(),
            // Back to the language menu — only when not mid-typing
            KeyCode::Up | KeyCode::Down => {
                if self.game_state.first_input_time.is_none() || self.game_state.game_over {
                    self.game_state.open_menu();
                }
            }
            KeyCode::Esc => {
                if self.game_state.first_input_time.is_some() && !self.game_state.game_over {
                    // Typing in progress - restart current sample
                    self.game_state.restart_current();
                } else {
                    // Not started or game over - show quit confirmation
                    self.game_state.confirm_quit = true;
                }
            }
            _ => {}
        }
        true
    }
}
