use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};
use std::time::Instant;

use super::GameState;

pub fn draw(f: &mut Frame, game_state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title with decorative border
    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default().fg(Color::Cyan));
    let title = Paragraph::new("⌨️  CODE TYPING GAME")
        .block(title_block)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Progress bar at the bottom of code area
    let progress = (game_state.user_input.len() as f64 / game_state.current_code.len() as f64) * 100.0;
    
    // Split the main code area to add progress bar at bottom
    let code_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)].as_ref())
        .split(chunks[1]);
    
    let progress_gauge = Gauge::default()
        .block(Block::default().style(Style::default().fg(Color::Cyan)))
        .gauge_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .label(format!("{}%", progress as u32))
        .ratio(progress / 100.0);

    // Combined Code to Type and Your Input
    let code_block = Block::default()
        .title("  CODE  ")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightYellow))
        .border_style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
        .padding(ratatui::widgets::Padding::symmetric(2, 1)); // Add horizontal and vertical padding

    let mut code_lines = Vec::new();
    let mut char_index = 0;
    let mut cursor_pos = None;

    for (_line_idx, line) in game_state.current_code.lines().enumerate() {
        let mut line_spans = Vec::new();
        for ch in line.chars() {
            if char_index == game_state.user_input.len() && !game_state.game_over {
                // Mark cursor position
                cursor_pos = Some(line_spans.len());
            }

            let style = if char_index < game_state.user_input.len() {
                let user_char = game_state.user_input.chars().nth(char_index).unwrap_or(' ');
                if user_char == ch {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                }
            } else if char_index == game_state.user_input.len() && !game_state.game_over {
                // Cursor style - highlighted next character
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            line_spans.push(Span::styled(ch.to_string(), style));
            char_index += 1;
        }

        // If cursor is at the end of this line and no character matched yet
        if char_index == game_state.user_input.len()
            && cursor_pos.is_none()
            && !game_state.game_over
        {
            cursor_pos = Some(line_spans.len());
            // Add cursor at end of line
            line_spans.push(Span::styled(
                "▋".to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
            ));
        }

        code_lines.push(Line::from(line_spans));
        // Account for newline character by incrementing char_index
        // The newline is in the original code but not in lines()
        char_index += 1;
    }

    let code_display = Paragraph::new(code_lines)
        .block(code_block)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(code_display, code_area_chunks[0]);
    f.render_widget(progress_gauge, code_area_chunks[1]);

    // Stats and controls with better layout
    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    // Calculate elapsed time starting from first input
    let elapsed = if let Some(first_input) = game_state.first_input_time {
        if let Some(end_time) = game_state.end_time {
            end_time.duration_since(first_input)
        } else {
            Instant::now().duration_since(first_input)
        }
    } else {
        std::time::Duration::from_secs(0)
    };

    let total_millis = elapsed.as_millis();
    let minutes = total_millis / 60000;
    let seconds = (total_millis % 60000) / 1000;
    let millis = total_millis % 1000;
    let timer_str = format!("{}:{:02}.{:03}", minutes, seconds, millis);

    // Stats row with better formatting
    let stats_line1 = vec![
        Span::styled("WPM ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{:6.1}", game_state.wpm),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("  │  "),
        Span::styled("ACC ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{:6.1}%", game_state.accuracy),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("  │  "),
        Span::styled("TIME ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled(timer_str, Style::default().fg(Color::Yellow)),
    ];

    // Controls
    let controls_text = if game_state.game_over {
        vec![Line::from(vec![
            Span::raw("  "),
            Span::styled("✓ COMPLETE", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  │  "),
            Span::styled("ENTER", Style::default().fg(Color::Cyan)),
            Span::raw(" next  │  "),
            Span::styled("ESC", Style::default().fg(Color::Cyan)),
            Span::raw(" quit"),
        ])]
    } else {
        vec![Line::from(vec![
            Span::raw("  "),
            Span::styled("TAB", Style::default().fg(Color::Cyan)),
            Span::raw(" indent  │  "),
            Span::styled("BACKSPACE", Style::default().fg(Color::Cyan)),
            Span::raw(" delete  │  "),
            Span::styled("ESC", Style::default().fg(Color::Cyan)),
            Span::raw(" quit"),
        ])]
    };

    let stats_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let stats_widget = Paragraph::new(Line::from(stats_line1))
        .block(stats_block)
        .style(Style::default());

    f.render_widget(stats_widget, stats_chunks[0]);

    let controls_widget = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::White));

    f.render_widget(controls_widget, stats_chunks[1]);
}
