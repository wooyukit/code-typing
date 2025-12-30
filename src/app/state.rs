use std::time::Instant;
use super::samples::CODE_SAMPLES;

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
}

impl GameState {
    pub fn new() -> Self {
        let idx = (Instant::now().elapsed().as_nanos() as usize) % CODE_SAMPLES.len();
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
        let chars_to_add = if c == '\t' {
            "    ".to_string() // 4 spaces
        } else {
            c.to_string()
        };

        // Count actual characters being added for accuracy calculation
        self.total_chars += chars_to_add.len();

        // Add characters and check for correctness
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

        self.update_stats();
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
        let idx = (Instant::now().elapsed().as_nanos() as usize) % CODE_SAMPLES.len();
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
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
