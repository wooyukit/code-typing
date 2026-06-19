use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, LineGauge, Padding, Paragraph, Wrap},
    Frame,
};
use std::time::Instant;

use super::language::ALL;
use super::state::Screen;
use super::syntax;
use super::GameState;

// Color palette
const COLOR_CYAN: Color = Color::Rgb(125, 207, 255); // Main/Primary cyan
const COLOR_GREEN: Color = Color::Rgb(158, 206, 106); // Success green
const COLOR_YELLOW: Color = Color::Rgb(224, 175, 104); // Warning yellow
const COLOR_RED: Color = Color::Rgb(247, 118, 142); // Error red
const COLOR_BLUE: Color = Color::Rgb(122, 162, 247); // Secondary blue
const COLOR_GOLD: Color = Color::Rgb(255, 215, 0); // Gold for code frame
const COLOR_ORANGE: Color = Color::Rgb(255, 158, 100); // Orange for rainbow
const COLOR_PURPLE: Color = Color::Rgb(187, 154, 247); // Purple for rainbow
const COLOR_WHITE: Color = Color::Rgb(220, 225, 252); // Bright text
const COLOR_CODE: Color = Color::Rgb(90, 100, 130); // Untyped code - dimmed
const COLOR_GRAY: Color = Color::Rgb(100, 110, 150); // Muted but visible
const COLOR_DARK: Color = Color::Rgb(26, 27, 38); // Dark bg
const COLOR_SURFACE: Color = Color::Rgb(36, 40, 59); // Surface
const COLOR_CURSOR_BG: Color = Color::Rgb(255, 220, 100); // Bright cursor background

pub fn draw(f: &mut Frame, game_state: &GameState) {
    match game_state.screen {
        Screen::Menu => draw_menu(f, game_state),
        Screen::Typing => draw_typing(f, game_state),
    }

    if game_state.confirm_quit {
        render_quit_confirmation(f);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MENU SCREEN — language selection
// ═══════════════════════════════════════════════════════════════════════════
fn draw_menu(f: &mut Frame, game_state: &GameState) {
    let area = popup_area(f.area(), 50, 80);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ⌨ ", Style::default().fg(COLOR_GOLD).bold()),
            Span::styled("code-typing", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" ⌨ ", Style::default().fg(COLOR_GOLD).bold()),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_GOLD))
        .padding(Padding::new(3, 3, 1, 1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![
        Line::from(Span::styled(
            "Select a language to practice",
            Style::default().fg(COLOR_WHITE).bold(),
        )),
        Line::from(""),
    ];

    for (i, &lang) in ALL.iter().enumerate() {
        let selected = i == game_state.menu_index;
        let (marker, name_style) = if selected {
            ("▸ ", Style::default().fg(COLOR_CYAN).bold())
        } else {
            ("  ", Style::default().fg(COLOR_GRAY))
        };
        lines.push(Line::from(vec![
            Span::styled(marker, name_style),
            Span::styled(format!("{} ", lang.spec().emoji), name_style),
            Span::styled(format!("{:<12}", lang.name()), name_style),
            Span::styled(
                format!("{} snippets", lang.spec().samples.len()),
                Style::default().fg(COLOR_CODE),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(COLOR_CYAN).bold()),
        Span::styled(" navigate   ", Style::default().fg(COLOR_GRAY)),
        Span::styled("Enter", Style::default().fg(COLOR_GREEN).bold()),
        Span::styled(" start   ", Style::default().fg(COLOR_GRAY)),
        Span::styled("Esc", Style::default().fg(COLOR_RED).bold()),
        Span::styled(" quit", Style::default().fg(COLOR_GRAY)),
    ]));

    f.render_widget(Paragraph::new(lines), inner);
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPING SCREEN
// ═══════════════════════════════════════════════════════════════════════════
fn draw_typing(f: &mut Frame, game_state: &GameState) {
    // Determine if we should show output (game over and has expected output)
    let show_output = game_state.game_over && !game_state.expected_output.is_empty();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(8),    // Code area (includes output when game over)
            Constraint::Length(5), // Stats
        ])
        .split(f.area());

    // ═══════════════════════════════════════════════════════════════════════
    // TITLE SECTION - Rainbow single line
    // ═══════════════════════════════════════════════════════════════════════
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_GOLD));

    let title_text = vec![
        Span::styled("━━━", Style::default().fg(COLOR_RED)),
        Span::styled("━━━", Style::default().fg(COLOR_ORANGE)),
        Span::styled(" ⌨ ", Style::default().fg(COLOR_GOLD).bold()),
        Span::styled("C", Style::default().fg(COLOR_RED).bold()),
        Span::styled("o", Style::default().fg(COLOR_ORANGE).bold()),
        Span::styled("d", Style::default().fg(COLOR_YELLOW).bold()),
        Span::styled("e", Style::default().fg(COLOR_GREEN).bold()),
        Span::styled("::", Style::default().fg(COLOR_GRAY)),
        Span::styled("t", Style::default().fg(COLOR_CYAN).bold()),
        Span::styled("y", Style::default().fg(COLOR_BLUE).bold()),
        Span::styled("p", Style::default().fg(COLOR_PURPLE).bold()),
        Span::styled("e", Style::default().fg(COLOR_RED).bold()),
        Span::styled("()", Style::default().fg(COLOR_GRAY)),
        Span::styled(" ⌨ ", Style::default().fg(COLOR_GOLD).bold()),
        Span::styled("━━━", Style::default().fg(COLOR_CYAN)),
        Span::styled("━━━", Style::default().fg(COLOR_BLUE)),
    ];

    let title = Paragraph::new(Line::from(title_text))
        .block(title_block)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // ═══════════════════════════════════════════════════════════════════════
    // CODE SECTION - Main typing area
    // ═══════════════════════════════════════════════════════════════════════
    let code_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if show_output {
            vec![
                Constraint::Min(5),    // Code display
                Constraint::Length(8), // Output section
                Constraint::Length(3), // Progress bar
            ]
        } else {
            vec![
                Constraint::Min(5),    // Code display
                Constraint::Length(0), // No output
                Constraint::Length(3), // Progress bar
            ]
        })
        .split(chunks[1]);

    // Calculate progress for title (with division by zero protection)
    let progress = if game_state.current_code_chars.is_empty() {
        0.0
    } else {
        (game_state.user_input_chars.len() as f64 / game_state.current_code_chars.len() as f64)
            .min(1.0)
    };
    let percent = (progress * 100.0) as u16;

    let title_color = if game_state.game_over {
        COLOR_GOLD
    } else if percent >= 66 {
        COLOR_GREEN
    } else if percent >= 33 {
        COLOR_CYAN
    } else {
        COLOR_BLUE
    };

    let code_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" 📝 ", Style::default()),
            Span::styled("Code ", Style::default().fg(title_color).bold()),
            Span::styled("· ", Style::default().fg(COLOR_GRAY)),
            Span::styled(
                format!("{} ", game_state.language.spec().emoji),
                Style::default(),
            ),
            Span::styled(
                format!("{} ", game_state.language.name()),
                Style::default().fg(COLOR_WHITE).bold(),
            ),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(title_color))
        .padding(Padding::new(3, 3, 1, 1));

    let mut code_lines = Vec::new();
    let mut char_index = 0;

    let total_lines = game_state.current_code.lines().count();
    let line_num_width = total_lines.to_string().len().max(2);

    // Pre-compute syntax highlighting for the entire code
    let syntax_colors = syntax::highlight(&game_state.current_code, game_state.language.spec());

    for (line_idx, line) in game_state.current_code.lines().enumerate() {
        let mut line_spans = Vec::new();

        // Line number with separator
        let line_num = format!("{:>width$}", line_idx + 1, width = line_num_width);
        line_spans.push(Span::styled(line_num, Style::default().fg(COLOR_GRAY)));
        line_spans.push(Span::styled(" │ ", Style::default().fg(COLOR_DARK)));

        for ch in line.chars() {
            let style = if char_index < game_state.user_input_chars.len() {
                // O(1) access using pre-computed Vec<char>
                let user_char = game_state.user_input_chars[char_index];
                if user_char == ch {
                    // Correct - use syntax highlighting color
                    let syn_color = syntax_colors
                        .get(char_index)
                        .map(|t| t.color())
                        .unwrap_or(COLOR_CODE);
                    Style::default().fg(syn_color)
                } else {
                    // Wrong - bright red on dark red background with underline for high visibility
                    Style::default()
                        .fg(Color::Rgb(255, 100, 100))
                        .bg(Color::Rgb(80, 20, 30))
                        .add_modifier(Modifier::UNDERLINED)
                }
            } else if char_index == game_state.user_input_chars.len() && !game_state.game_over {
                // Current cursor position - bright yellow background for high visibility
                Style::default().fg(COLOR_DARK).bg(COLOR_CURSOR_BG).bold()
            } else {
                // Untyped code - dimmed
                Style::default().fg(COLOR_CODE)
            };
            line_spans.push(Span::styled(ch.to_string(), style));
            char_index += 1;
        }

        // Blinking cursor at end of line (for Enter/newline)
        if char_index == game_state.user_input_chars.len() && !game_state.game_over {
            line_spans.push(Span::styled(
                "↵",
                Style::default()
                    .fg(COLOR_CURSOR_BG)
                    .bold()
                    .add_modifier(Modifier::SLOW_BLINK),
            ));
        }

        code_lines.push(Line::from(line_spans));
        char_index += 1;
    }

    let code_display = Paragraph::new(code_lines)
        .block(code_block)
        .wrap(Wrap { trim: false });
    f.render_widget(code_display, code_area_chunks[0]);

    // ═══════════════════════════════════════════════════════════════════════
    // OUTPUT SECTION - Expected output when game is over (above progress bar)
    // ═══════════════════════════════════════════════════════════════════════
    if show_output {
        let output_block = Block::default()
            .title(Line::from(vec![
                Span::styled(" 📤 ", Style::default()),
                Span::styled("Output ", Style::default().fg(COLOR_GREEN).bold()),
            ]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(COLOR_GREEN))
            .padding(Padding::new(2, 3, 0, 0));

        let output_lines: Vec<Line> = game_state
            .expected_output
            .lines()
            .map(|line| Line::from(Span::styled(line, Style::default().fg(COLOR_WHITE))))
            .collect();

        let output_display = Paragraph::new(output_lines)
            .block(output_block)
            .wrap(Wrap { trim: false });
        f.render_widget(output_display, code_area_chunks[1]);
    }

    // Progress bar / completion celebration (at code_area_chunks[2])
    if game_state.game_over {
        // Completion celebration UI
        let rating = if game_state.wpm >= 80.0 && game_state.accuracy >= 98.0 {
            ("🏆 LEGENDARY", COLOR_GOLD)
        } else if game_state.wpm >= 60.0 && game_state.accuracy >= 95.0 {
            ("⭐ EXCELLENT", COLOR_GREEN)
        } else if game_state.wpm >= 40.0 && game_state.accuracy >= 85.0 {
            ("✓ GOOD", COLOR_BLUE)
        } else {
            ("→ COMPLETE", COLOR_CYAN)
        };

        let completion_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(rating.1))
            .style(Style::default().bg(COLOR_SURFACE));

        // Calculate elapsed time
        let elapsed_secs = match (game_state.first_input_time, game_state.end_time) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f32(),
            _ => 0.0,
        };
        let mins = (elapsed_secs / 60.0).floor() as u32;
        let secs = (elapsed_secs % 60.0).floor() as u32;

        let completion_text = Line::from(vec![
            Span::styled(
                format!(" {} ", rating.0),
                Style::default().fg(rating.1).bold(),
            ),
            Span::styled(" │ ", Style::default().fg(COLOR_GRAY)),
            Span::styled(
                format!("⚡ {:.0} wpm  ", game_state.wpm),
                Style::default().fg(COLOR_WHITE).bold(),
            ),
            Span::styled(
                format!("🎯 {:.1}%  ", game_state.accuracy),
                Style::default().fg(COLOR_WHITE).bold(),
            ),
            Span::styled(
                format!("⏱ {}:{:02}  ", mins, secs),
                Style::default().fg(COLOR_WHITE).bold(),
            ),
            Span::styled("│ ", Style::default().fg(COLOR_GRAY)),
            Span::styled("↵", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" next  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("↑↓", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" language  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" quit ", Style::default().fg(COLOR_GRAY)),
        ]);

        let completion_para = Paragraph::new(completion_text)
            .block(completion_block)
            .alignment(Alignment::Center);
        f.render_widget(completion_para, code_area_chunks[2]);
    } else {
        // Progress bar with percentage and char info in frame title
        let progress_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(title_color))
            .padding(Padding::new(1, 2, 0, 0))
            .title(Line::from(vec![
                Span::styled(" Progress ", Style::default().fg(COLOR_WHITE).bold()),
                Span::styled(
                    format!("{}% ", percent),
                    Style::default().fg(title_color).bold(),
                ),
                Span::styled(
                    format!(
                        "({}/{}) ",
                        game_state.user_input_chars.len(),
                        game_state.current_code_chars.len()
                    ),
                    Style::default().fg(COLOR_GRAY),
                ),
            ]));

        let progress_gauge = LineGauge::default()
            .block(progress_block)
            .filled_style(Style::default().fg(title_color).bold())
            .unfilled_style(Style::default().fg(COLOR_SURFACE))
            .line_set(symbols::line::THICK)
            .label("")
            .ratio(progress);
        f.render_widget(progress_gauge, code_area_chunks[2]);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STATS SECTION - Performance metrics
    // ═══════════════════════════════════════════════════════════════════════
    let stats_chunk = chunks[2];
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(stats_chunk);

    // Calculate elapsed time
    let elapsed = if let Some(first_input) = game_state.first_input_time {
        if let Some(end_time) = game_state.end_time {
            end_time.duration_since(first_input)
        } else {
            Instant::now().duration_since(first_input)
        }
    } else {
        std::time::Duration::from_secs(0)
    };

    let total_ms = elapsed.as_millis();
    let mins = total_ms / 60000;
    let secs = (total_ms % 60000) / 1000;
    let ms = total_ms % 1000;
    let timer_str = format!("{}:{:02}.{:03}", mins, secs, ms);

    // WPM Card
    let wpm_color = if game_state.wpm >= 60.0 {
        COLOR_GREEN
    } else if game_state.wpm >= 40.0 {
        COLOR_BLUE
    } else {
        COLOR_YELLOW
    };
    render_stat_card(
        f,
        stats_chunks[0],
        "⚡ WPM",
        &format!("{:.0}", game_state.wpm),
        wpm_color,
    );

    // Accuracy Card
    let acc_color = if game_state.accuracy >= 95.0 {
        COLOR_GREEN
    } else if game_state.accuracy >= 85.0 {
        COLOR_YELLOW
    } else {
        COLOR_RED
    };
    render_stat_card(
        f,
        stats_chunks[1],
        "🎯 ACC",
        &format!("{:.1}%", game_state.accuracy),
        acc_color,
    );

    // Time Card
    render_stat_card(f, stats_chunks[2], "⏱ TIME", &timer_str, COLOR_CYAN);

    // Controls
    let controls_area = Rect {
        x: stats_chunk.x,
        y: stats_chunk.y + stats_chunk.height,
        width: stats_chunk.width,
        height: 1,
    };

    let controls_text = if game_state.game_over {
        vec![
            Span::styled("↵", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" next  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("↑↓", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" language  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" quit", Style::default().fg(COLOR_GRAY)),
        ]
    } else if game_state.first_input_time.is_none() {
        vec![
            Span::styled("◀▶", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" sample  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("↑↓", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" language  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("TAB", Style::default().fg(COLOR_YELLOW).bold()),
            Span::styled(" indent  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" quit", Style::default().fg(COLOR_GRAY)),
        ]
    } else {
        vec![
            Span::styled("↵", Style::default().fg(COLOR_GREEN).bold()),
            Span::styled(" newline  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("TAB", Style::default().fg(COLOR_YELLOW).bold()),
            Span::styled(" indent  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("⌫", Style::default().fg(COLOR_BLUE).bold()),
            Span::styled(" delete  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" restart", Style::default().fg(COLOR_GRAY)),
        ]
    };

    let controls = Paragraph::new(Line::from(controls_text)).alignment(Alignment::Center);
    f.render_widget(controls, controls_area);
}

/// Calculate a centered popup area
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// Render quit confirmation dialog
fn render_quit_confirmation(f: &mut Frame) {
    let popup = popup_area(f.area(), 35, 22);
    f.render_widget(Clear, popup);

    let popup_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ⚠ ", Style::default().fg(COLOR_YELLOW)),
            Span::styled("QUIT?", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" ⚠ ", Style::default().fg(COLOR_YELLOW)),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_RED))
        .style(Style::default().bg(COLOR_SURFACE));

    let inner = popup_block.inner(popup);
    f.render_widget(popup_block, popup);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let message = Paragraph::new(Line::from(vec![Span::styled(
        "Are you sure?",
        Style::default().fg(COLOR_WHITE),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(message, content_chunks[0]);

    let options = Paragraph::new(Line::from(vec![
        Span::styled("Y", Style::default().fg(COLOR_GREEN).bold()),
        Span::styled("es  ", Style::default().fg(COLOR_GRAY)),
        Span::styled("N", Style::default().fg(COLOR_BLUE).bold()),
        Span::styled("o", Style::default().fg(COLOR_GRAY)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(options, content_chunks[2]);
}

fn render_stat_card(f: &mut Frame, area: Rect, title: &str, value: &str, color: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let title_line = Line::from(Span::styled(title, Style::default().fg(COLOR_GRAY)));
    let title_widget = Paragraph::new(title_line).alignment(Alignment::Center);

    let value_line = Line::from(Span::styled(value, Style::default().fg(color).bold()));
    let value_widget = Paragraph::new(value_line).alignment(Alignment::Center);

    if inner.height >= 2 {
        let title_area = Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: 1,
        };
        let value_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: 1,
        };
        f.render_widget(title_widget, title_area);
        f.render_widget(value_widget, value_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::language::ALL;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    /// Render a frame headlessly and return its text content (all cell symbols).
    fn render(game_state: &GameState, w: u16, h: u16) -> String {
        let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
        terminal.draw(|f| draw(f, game_state)).unwrap();
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn menu_screen_renders_language_list() {
        let out = render(&GameState::new(), 80, 30);
        assert!(out.contains("code-typing"));
        assert!(out.contains("Rust"));
        assert!(out.contains("Python"));
        assert!(out.contains("C++"));
        assert!(out.contains("🦀")); // language emoji rendered
    }

    /// Drawing must not panic for any language, fresh or completed (the latter
    /// exercises the output panel + rating). This walks the char-index render loop.
    #[test]
    fn typing_and_completion_render_for_every_language() {
        for (i, _) in ALL.iter().enumerate() {
            let mut gs = GameState::new();
            gs.menu_index = i;
            gs.select_menu_language();
            let _ = render(&gs, 80, 30);
            gs.finish_game();
            let _ = render(&gs, 80, 30);
        }
    }

    #[test]
    fn renders_in_a_small_terminal_without_panicking() {
        let _ = render(&GameState::new(), 40, 12);
        let mut gs = GameState::new();
        gs.select_menu_language();
        let _ = render(&gs, 40, 12);
    }
}
