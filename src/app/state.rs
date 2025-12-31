use std::time::Instant;
use super::samples::CODE_SAMPLES;
use rand::Rng;

#[derive(Clone)]
pub struct GameState {
    pub current_code: String,
    pub user_input: String,
    pub start_time: Instant,
    pub first_input_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub correct_chars: usize,
    pub total_chars: usize,
    pub wpm: f32,
    pub accuracy: f32,
    pub game_over: bool,
    pub message: String,
    pub confirm_quit: bool,  // Whether showing quit confirmation
}

impl GameState {
    pub fn new() -> Self {
        let idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
        GameState {
            current_code: CODE_SAMPLES[idx].to_string(),
            user_input: String::new(),
            start_time: Instant::now(),
            first_input_time: None,
            end_time: None,
            correct_chars: 0,
            total_chars: 0,
            wpm: 0.0,
            accuracy: 0.0,
            game_over: false,
            message: String::new(),
            confirm_quit: false,
        }
    }

    /// Switch to a random code sample (only works before typing starts)
    pub fn random_sample(&mut self) {
        if self.first_input_time.is_none() {
            let idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
            self.current_code = CODE_SAMPLES[idx].to_string();
            self.user_input.clear();
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
        let chars_to_add = if c == '\t' {
            "    ".to_string() // 4 spaces
        } else if c == '\n' {
            // Auto-indent: add newline plus the leading whitespace of the next line
            self.get_auto_indent_chars()
        } else {
            c.to_string()
        };

        // For auto-indent (Enter), count as 1 keystroke and check if ALL chars are correct
        // For other inputs, count each character
        let is_auto_indent = c == '\n' && chars_to_add.len() > 1;
        
        if is_auto_indent {
            // Count as 1 keystroke
            self.total_chars += 1;
            
            // Check if ALL auto-indented characters match
            let mut all_correct = true;
            for (i, ch) in chars_to_add.chars().enumerate() {
                let target_pos = self.user_input.len() + i;
                if target_pos < self.current_code.len() {
                    let target_char = self.current_code.chars().nth(target_pos);
                    if Some(ch) != target_char {
                        all_correct = false;
                    }
                }
            }
            
            // Add characters to input
            for ch in chars_to_add.chars() {
                self.user_input.push(ch);
                if self.user_input.len() >= self.current_code.len() {
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
            // Normal input: count each character
            self.total_chars += chars_to_add.len();
            
            for ch in chars_to_add.chars() {
                self.user_input.push(ch);

                if self.user_input.len() <= self.current_code.len() {
                    let current_char = self.current_code.chars().nth(self.user_input.len() - 1);
                    if Some(ch) == current_char {
                        self.correct_chars += 1;
                    }
                }

                if self.user_input.len() >= self.current_code.len() {
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
        let current_pos = self.user_input.len();
        
        // Check if the next character in target code is a newline
        if let Some('\n') = self.current_code.chars().nth(current_pos) {
            let mut result = String::from("\n");
            
            // Look at characters after the newline and collect leading whitespace
            let remaining: String = self.current_code.chars().skip(current_pos + 1).collect();
            for ch in remaining.chars() {
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
        if !self.game_over && !self.user_input.is_empty() {
            self.user_input.pop();
            self.update_stats();
        }
    }

    pub fn update_stats(&mut self) {
        let elapsed_secs = self.start_time.elapsed().as_secs_f32();
        if elapsed_secs > 0.0 && self.total_chars > 0 {
            let words_typed = self.user_input.len() as f32 / 5.0;
            self.wpm = (words_typed / elapsed_secs) * 60.0;
            self.accuracy = if self.total_chars > 0 {
                (self.correct_chars as f32 / self.total_chars as f32) * 100.0
            } else {
                0.0
            };
        }
    }

    pub fn finish_game(&mut self) {
        self.game_over = true;
        self.end_time = Some(Instant::now());
        self.update_stats();

        let is_perfect = self.user_input == self.current_code;
        if is_perfect {
            self.message = format!(
                "Perfect! WPM: {:.1} | Accuracy: {:.1}%",
                self.wpm, self.accuracy
            );
        } else {
            self.message = format!(
                "Complete! WPM: {:.1} | Accuracy: {:.1}%",
                self.wpm, self.accuracy
            );
        }
    }

    pub fn reset(&mut self) {
        let idx = rand::thread_rng().gen_range(0..CODE_SAMPLES.len());
        self.current_code = CODE_SAMPLES[idx].to_string();
        self.user_input.clear();
        self.start_time = Instant::now();
        self.first_input_time = None;
        self.end_time = None;
        self.correct_chars = 0;
        self.total_chars = 0;
        self.wpm = 0.0;
        self.accuracy = 0.0;
        self.game_over = false;
        self.message.clear();
        self.confirm_quit = false;
    }

    /// Restart the current sample (keep same code, reset progress)
    pub fn restart_current(&mut self) {
        self.user_input.clear();
        self.start_time = Instant::now();
        self.first_input_time = None;
        self.end_time = None;
        self.correct_chars = 0;
        self.total_chars = 0;
        self.wpm = 0.0;
        self.accuracy = 0.0;
        self.game_over = false;
        self.message.clear();
        self.confirm_quit = false;
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
