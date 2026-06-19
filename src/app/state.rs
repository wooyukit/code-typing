use rand::Rng;
use std::time::Instant;

use super::language::{Language, ALL};

/// Which screen the app is showing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    /// Language selection menu (shown at launch).
    Menu,
    /// The typing screen.
    Typing,
}

#[derive(Clone)]
pub struct GameState {
    pub screen: Screen,
    pub menu_index: usize, // highlighted language in the menu
    pub language: Language,
    pub current_code: String,
    pub current_code_chars: Vec<char>, // Pre-computed for O(1) access
    pub expected_output: String,       // Expected output when code is run
    pub user_input: String,
    pub user_input_chars: Vec<char>, // Pre-computed for O(1) access
    pub first_input_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub correct_chars: usize,
    pub wpm: f32,
    pub accuracy: f32,
    pub game_over: bool,
    pub confirm_quit: bool,
    current_sample_idx: usize, // Track current sample to avoid repeat
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            screen: Screen::Menu,
            menu_index: 0,
            language: ALL[0],
            current_code: String::new(),
            current_code_chars: Vec::new(),
            expected_output: String::new(),
            user_input: String::new(),
            user_input_chars: Vec::new(),
            first_input_time: None,
            end_time: None,
            correct_chars: 0,
            wpm: 0.0,
            accuracy: 0.0,
            game_over: false,
            confirm_quit: false,
            current_sample_idx: usize::MAX, // sentinel: allow any first sample
        };
        state.load_random_sample();
        state
    }

    /// Samples for the currently selected language.
    fn samples(&self) -> &'static [(&'static str, &'static str)] {
        self.language.spec().samples
    }

    // ── Menu navigation ──────────────────────────────────────────────────────

    pub fn menu_up(&mut self) {
        self.menu_index = if self.menu_index == 0 {
            ALL.len() - 1
        } else {
            self.menu_index - 1
        };
    }

    pub fn menu_down(&mut self) {
        self.menu_index = (self.menu_index + 1) % ALL.len();
    }

    /// Open the language menu, positioning the cursor on the current language.
    pub fn open_menu(&mut self) {
        self.menu_index = ALL.iter().position(|&l| l == self.language).unwrap_or(0);
        self.screen = Screen::Menu;
    }

    /// Confirm the menu selection: switch language and start a fresh sample.
    pub fn select_menu_language(&mut self) {
        self.language = ALL[self.menu_index];
        self.current_sample_idx = usize::MAX; // allow any first sample for the new language
        self.screen = Screen::Typing;
        self.load_random_sample();
    }

    // ── Sample loading ─────────────────────────────────────────────────────────

    /// Load a specific sample within the current language and reset typing progress.
    fn load_sample(&mut self, idx: usize) {
        let (code, output) = self.samples()[idx];
        self.current_sample_idx = idx;
        self.current_code = code.to_string();
        self.current_code_chars = self.current_code.chars().collect();
        self.expected_output = output.to_string();
        self.reset_progress();
    }

    /// Pick a random sample in the current language, avoiding an immediate repeat.
    fn load_random_sample(&mut self) {
        let len = self.samples().len();
        let mut idx = rand::thread_rng().gen_range(0..len);
        if len > 1 {
            while idx == self.current_sample_idx {
                idx = rand::thread_rng().gen_range(0..len);
            }
        }
        self.load_sample(idx);
    }

    /// Switch to a random code sample (only works before typing starts).
    pub fn random_sample(&mut self) {
        if self.first_input_time.is_none() && self.samples().len() > 1 {
            self.load_random_sample();
        }
    }

    pub fn handle_input(&mut self, c: char) {
        if self.game_over {
            return;
        }

        // Record the time of first input
        if self.first_input_time.is_none() {
            self.first_input_time = Some(Instant::now());
        }

        // Convert tab to the language's indent width to match the code samples.
        // For Enter, auto-indent by matching the next line's leading whitespace.
        let chars_to_add: Vec<char> = if c == '\t' {
            vec![' '; self.language.spec().indent_spaces]
        } else if c == '\n' {
            self.get_auto_indent_chars().chars().collect()
        } else {
            vec![c]
        };

        // For auto-indent (Enter), count as 1 keystroke and check if ALL chars are correct.
        // For other inputs, count each character.
        let is_auto_indent = c == '\n' && chars_to_add.len() > 1;

        if is_auto_indent {
            // Check if ALL auto-indented characters match (O(1) access)
            let mut all_correct = true;
            for (i, &ch) in chars_to_add.iter().enumerate() {
                let target_pos = self.user_input_chars.len() + i;
                if target_pos < self.current_code_chars.len()
                    && ch != self.current_code_chars[target_pos]
                {
                    all_correct = false;
                }
            }

            // Add characters to input
            for ch in chars_to_add {
                self.user_input.push(ch);
                self.user_input_chars.push(ch);
                if self.user_input_chars.len() >= self.current_code_chars.len() {
                    if all_correct {
                        self.correct_chars += 1;
                    }
                    self.finish_game();
                    return;
                }
            }

            // Count as 1 correct if all matched
            if all_correct {
                self.correct_chars += 1;
            }
        } else {
            // Normal input
            for ch in chars_to_add {
                self.user_input.push(ch);
                self.user_input_chars.push(ch);

                let pos = self.user_input_chars.len() - 1;
                if pos < self.current_code_chars.len() && ch == self.current_code_chars[pos] {
                    self.correct_chars += 1;
                }

                if self.user_input_chars.len() >= self.current_code_chars.len() {
                    self.finish_game();
                    return;
                }
            }
        }

        self.update_stats();
    }

    /// Get the characters to add for auto-indent when Enter is pressed.
    /// Returns the newline character plus any leading whitespace from the next line.
    fn get_auto_indent_chars(&self) -> String {
        let current_pos = self.user_input_chars.len();

        // Check if the next character in target code is a newline (O(1) access)
        if current_pos < self.current_code_chars.len()
            && self.current_code_chars[current_pos] == '\n'
        {
            let mut result = String::from("\n");

            // Look at characters after the newline and collect leading whitespace
            for i in (current_pos + 1)..self.current_code_chars.len() {
                let ch = self.current_code_chars[i];
                if ch == ' ' || ch == '\t' {
                    result.push(ch);
                } else {
                    break;
                }
            }

            result
        } else {
            // If next char isn't a newline, just add the newline (user made a mistake)
            String::from("\n")
        }
    }

    pub fn handle_backspace(&mut self) {
        if !self.game_over && !self.user_input_chars.is_empty() {
            // Remove last character
            self.user_input.pop();
            self.user_input_chars.pop();
            // update_stats will recalculate correct_chars and accuracy
            self.update_stats();
        }
    }

    pub fn update_stats(&mut self) {
        let elapsed_secs = self
            .first_input_time
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(0.0);

        // Recalculate correct_chars to ensure consistency
        self.correct_chars = self
            .user_input_chars
            .iter()
            .zip(self.current_code_chars.iter())
            .filter(|(a, b)| a == b)
            .count();

        let input_len = self.user_input_chars.len();
        if elapsed_secs > 0.0 && input_len > 0 {
            let words_typed = input_len as f32 / 5.0;
            self.wpm = (words_typed / elapsed_secs) * 60.0;
            // Use input length for accuracy to ensure it never exceeds 100%
            self.accuracy = (self.correct_chars as f32 / input_len as f32) * 100.0;
        }
    }

    pub fn finish_game(&mut self) {
        self.game_over = true;
        self.end_time = Some(Instant::now());
        self.update_stats();
    }

    /// Move to a new random sample in the current language (used after completion).
    pub fn reset(&mut self) {
        self.load_random_sample();
    }

    /// Restart the current sample (keep same code, reset progress).
    pub fn restart_current(&mut self) {
        self.reset_progress();
    }

    /// Clear typing progress and stats while keeping the current code + language.
    fn reset_progress(&mut self) {
        self.user_input.clear();
        self.user_input_chars.clear();
        self.first_input_time = None;
        self.end_time = None;
        self.correct_chars = 0;
        self.wpm = 0.0;
        self.accuracy = 0.0;
        self.game_over = false;
        self.confirm_quit = false;
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_menu_with_a_loaded_sample() {
        let gs = GameState::new();
        assert_eq!(gs.screen, Screen::Menu);
        assert!(!gs.current_code.is_empty());
        // The String and the pre-computed Vec<char> must stay in lockstep.
        assert_eq!(gs.current_code_chars.len(), gs.current_code.chars().count());
    }

    #[test]
    fn selecting_a_language_enters_typing() {
        let mut gs = GameState::new();
        gs.menu_index = ALL.iter().position(|&l| l == Language::Python).unwrap();
        gs.select_menu_language();
        assert_eq!(gs.screen, Screen::Typing);
        assert_eq!(gs.language, Language::Python);
        assert!(!gs.current_code.is_empty());
    }

    #[test]
    fn tab_expands_to_the_language_indent_width() {
        let mut gs = GameState::new();
        gs.select_menu_language(); // Rust (menu_index 0)
        let width = gs.language.spec().indent_spaces;
        gs.handle_input('\t');
        assert_eq!(gs.user_input_chars.len(), width);
        assert!(gs.user_input_chars.iter().all(|&c| c == ' '));
    }

    #[test]
    fn menu_navigation_wraps() {
        let mut gs = GameState::new();
        gs.menu_index = 0;
        gs.menu_up();
        assert_eq!(gs.menu_index, ALL.len() - 1);
        gs.menu_down();
        assert_eq!(gs.menu_index, 0);
    }
}
