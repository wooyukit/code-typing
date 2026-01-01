use std::time::Instant;
use super::samples::CODE_SAMPLES;
use rand::Rng;

#[derive(Clone)]
pub struct GameState {
    pub current_code: String,
    pub current_code_chars: Vec<char>,  // Pre-computed for O(1) access
    pub user_input: String,
    pub user_input_chars: Vec<char>,    // Pre-computed for O(1) access
    pub first_input_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub correct_chars: usize,
    pub wpm: f32,
    pub accuracy: f32,
    pub game_over: bool,
    pub confirm_quit: bool,
    current_sample_idx: usize,  // Track current sample to avoid repeat
}

impl GameState {
    pub fn new() -> Self {
        let idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
        let code = CODE_SAMPLES[idx].to_string();
        let code_chars = code.chars().collect();
        GameState {
            current_code: code,
            current_code_chars: code_chars,
            user_input: String::new(),
            user_input_chars: Vec::new(),
            first_input_time: None,
            end_time: None,
            correct_chars: 0,
            wpm: 0.0,
            accuracy: 0.0,
            game_over: false,
            confirm_quit: false,
            current_sample_idx: idx,
        }
    }

    /// Switch to a random code sample (only works before typing starts)
    /// Ensures a different sample is selected
    pub fn random_sample(&mut self) {
        if self.first_input_time.is_none() && CODE_SAMPLES.len() > 1 {
            let mut idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
            // Ensure we pick a different sample
            while idx == self.current_sample_idx {
                idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
            }
            self.current_sample_idx = idx;
            self.current_code = CODE_SAMPLES[idx].to_string();
            self.current_code_chars = self.current_code.chars().collect();
            self.user_input.clear();
            self.user_input_chars.clear();
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

        // Convert tab to 4 spaces to match indentation in code samples
        // For Enter, auto-indent by looking at what the next line's indentation should be
        let chars_to_add: Vec<char> = if c == '\t' {
            vec![' ', ' ', ' ', ' '] // 4 spaces
        } else if c == '\n' {
            // Auto-indent: add newline plus the leading whitespace of the next line
            self.get_auto_indent_chars().chars().collect()
        } else {
            vec![c]
        };

        // For auto-indent (Enter), count as 1 keystroke and check if ALL chars are correct
        // For other inputs, count each character
        let is_auto_indent = c == '\n' && chars_to_add.len() > 1;

        if is_auto_indent {
            // Check if ALL auto-indented characters match (O(1) access)
            let mut all_correct = true;
            for (i, &ch) in chars_to_add.iter().enumerate() {
                let target_pos = self.user_input_chars.len() + i;
                if target_pos < self.current_code_chars.len() {
                    if ch != self.current_code_chars[target_pos] {
                        all_correct = false;
                    }
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
                if pos < self.current_code_chars.len() {
                    if ch == self.current_code_chars[pos] {
                        self.correct_chars += 1;
                    }
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
        if current_pos < self.current_code_chars.len() && self.current_code_chars[current_pos] == '\n' {
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
        let elapsed_secs = self.first_input_time
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(0.0);

        // Recalculate correct_chars to ensure consistency
        self.correct_chars = self.user_input_chars.iter()
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

    pub fn reset(&mut self) {
        // Ensure we pick a different sample
        let mut idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
        if CODE_SAMPLES.len() > 1 {
            while idx == self.current_sample_idx {
                idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
            }
        }
        self.current_sample_idx = idx;
        self.current_code = CODE_SAMPLES[idx].to_string();
        self.current_code_chars = self.current_code.chars().collect();
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

    /// Restart the current sample (keep same code, reset progress)
    pub fn restart_current(&mut self) {
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
