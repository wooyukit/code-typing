use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Gauge, Padding, Paragraph, Wrap},
    Frame,
};
use std::time::Instant;

use super::GameState;

// Color palette
const COLOR_CYAN: Color = Color::Rgb(125, 207, 255);       // Main/Primary cyan
const COLOR_GREEN: Color = Color::Rgb(158, 206, 106);      // Success green  
const COLOR_YELLOW: Color = Color::Rgb(224, 175, 104);     // Warning yellow
const COLOR_RED: Color = Color::Rgb(247, 118, 142);        // Error red
const COLOR_BLUE: Color = Color::Rgb(122, 162, 247);       // Secondary blue
const COLOR_GOLD: Color = Color::Rgb(255, 215, 0);         // Gold for code frame
const COLOR_ORANGE: Color = Color::Rgb(255, 158, 100);     // Orange for rainbow
const COLOR_PURPLE: Color = Color::Rgb(187, 154, 247);     // Purple for rainbow
const COLOR_WHITE: Color = Color::Rgb(192, 202, 245);      // Text
const COLOR_DIM_TEXT: Color = Color::Rgb(65, 72, 104);     // Dimmed text for untyped code
const COLOR_GRAY: Color = Color::Rgb(86, 95, 137);         // Muted
const COLOR_DARK: Color = Color::Rgb(26, 27, 38);          // Dark bg
const COLOR_SURFACE: Color = Color::Rgb(36, 40, 59);       // Surface

pub fn draw(f: &mut Frame, game_state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(8),     // Code area
            Constraint::Length(5),  // Stats
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
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(chunks[1]);

    let code_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" 📝 ", Style::default()),
            Span::styled("Code", Style::default().fg(COLOR_GREEN).bold()),
            Span::styled(" ", Style::default()),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_GREEN))
        .padding(Padding::new(3, 3, 1, 1));

    let mut code_lines = Vec::new();
    let mut char_index = 0;
    let mut cursor_pos = None;

    let total_lines = game_state.current_code.lines().count();
    let line_num_width = total_lines.to_string().len().max(2);

    for (line_idx, line) in game_state.current_code.lines().enumerate() {
        let mut line_spans = Vec::new();

        // Line number with separator
        let line_num = format!("{:>width$}", line_idx + 1, width = line_num_width);
        line_spans.push(Span::styled(line_num, Style::default().fg(COLOR_GRAY)));
        line_spans.push(Span::styled(" │ ", Style::default().fg(COLOR_DARK)));

        for ch in line.chars() {
            if char_index == game_state.user_input.len() && !game_state.game_over {
                cursor_pos = Some(line_spans.len());
            }

            let style = if char_index < game_state.user_input.len() {
                let user_char = game_state.user_input.chars().nth(char_index).unwrap_or(' ');
                if user_char == ch {
                    // Correct - green
                    Style::default().fg(COLOR_GREEN)
                } else {
                    // Wrong - red with underline
                    Style::default().fg(COLOR_RED).add_modifier(Modifier::UNDERLINED)
                }
            } else if char_index == game_state.user_input.len() && !game_state.game_over {
                // Cursor position - inverted
                Style::default().fg(COLOR_DARK).bg(COLOR_CYAN).bold()
            } else {
                // Untyped - dimmed
                Style::default().fg(COLOR_DIM_TEXT)
            };
            line_spans.push(Span::styled(ch.to_string(), style));
            char_index += 1;
        }

        // Blinking cursor at end of line
        if char_index == game_state.user_input.len()
            && cursor_pos.is_none()
            && !game_state.game_over
        {
            cursor_pos = Some(line_spans.len());
            line_spans.push(Span::styled(
                "█",
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::SLOW_BLINK),
            ));
        }

        code_lines.push(Line::from(line_spans));
        char_index += 1;
    }

    let code_display = Paragraph::new(code_lines)
        .block(code_block)
        .wrap(Wrap { trim: false });
    f.render_widget(code_display, code_area_chunks[0]);

    // Progress bar - full gauge style
    let progress = game_state.user_input.len() as f64 / game_state.current_code.len() as f64;
    let percent = (progress * 100.0) as u16;
    
    let progress_color = if percent >= 100 {
        COLOR_GREEN
    } else if percent >= 66 {
        COLOR_BLUE
    } else if percent >= 33 {
        COLOR_CYAN
    } else {
        COLOR_GRAY
    };

    let progress_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_GRAY))
        .title(Span::styled(" Progress ", Style::default().fg(COLOR_GRAY)));

    let progress_gauge = Gauge::default()
        .block(progress_block)
        .gauge_style(Style::default().fg(progress_color).bg(COLOR_DARK))
        .percent(percent)
        .label(Span::styled(
            format!("{}%  ({}/{})", percent, game_state.user_input.len(), game_state.current_code.len()),
            Style::default().fg(COLOR_WHITE).bold(),
        ));
    f.render_widget(progress_gauge, code_area_chunks[1]);

    // ═══════════════════════════════════════════════════════════════════════
    // STATS SECTION - Performance metrics
    // ═══════════════════════════════════════════════════════════════════════
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(chunks[2]);

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

    let total_secs = elapsed.as_secs_f32();
    let mins = (total_secs / 60.0) as u32;
    let secs = total_secs % 60.0;
    let timer_str = format!("{}:{:04.1}", mins, secs);

    // WPM Card
    let wpm_color = if game_state.wpm >= 60.0 {
        COLOR_GREEN
    } else if game_state.wpm >= 40.0 {
        COLOR_BLUE
    } else {
        COLOR_YELLOW
    };
    render_stat_card(f, stats_chunks[0], "⚡ WPM", &format!("{:.0}", game_state.wpm), wpm_color);

    // Accuracy Card
    let acc_color = if game_state.accuracy >= 95.0 {
        COLOR_GREEN
    } else if game_state.accuracy >= 85.0 {
        COLOR_YELLOW
    } else {
        COLOR_RED
    };
    render_stat_card(f, stats_chunks[1], "🎯 ACC", &format!("{:.1}%", game_state.accuracy), acc_color);

    // Time Card
    render_stat_card(f, stats_chunks[2], "⏱ TIME", &timer_str, COLOR_CYAN);

    // Controls
    let controls_area = Rect {
        x: chunks[2].x,
        y: chunks[2].y + chunks[2].height,
        width: chunks[2].width,
        height: 1,
    };

    let controls_text = if game_state.game_over {
        vec![
            Span::styled(" ✓ DONE ", Style::default().fg(COLOR_DARK).bg(COLOR_GREEN).bold()),
            Span::raw("  "),
            Span::styled("↵", Style::default().fg(COLOR_BLUE).bold()),
            Span::styled(" next  ", Style::default().fg(COLOR_GRAY)),
            Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
            Span::styled(" quit", Style::default().fg(COLOR_GRAY)),
        ]
    } else if game_state.first_input_time.is_none() {
        vec![
            Span::styled("◀▶", Style::default().fg(COLOR_CYAN).bold()),
            Span::styled(" sample  ", Style::default().fg(COLOR_GRAY)),
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

    // ═══════════════════════════════════════════════════════════════════════
    // POPUPS
    // ═══════════════════════════════════════════════════════════════════════
    if game_state.confirm_quit {
        render_quit_confirmation(f);
    } else if game_state.game_over {
        render_completion_popup(f, game_state, elapsed);
    }
}

/// Calculate a centered popup area
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// Render completion popup with stats
fn render_completion_popup(f: &mut Frame, game_state: &GameState, elapsed: std::time::Duration) {
    let popup = popup_area(f.area(), 50, 45);
    f.render_widget(Clear, popup);

    // Rating based on performance
    let (rating, rating_color, rating_emoji) = if game_state.wpm >= 80.0 && game_state.accuracy >= 98.0 {
        ("LEGENDARY", COLOR_YELLOW, "🏆")
    } else if game_state.wpm >= 60.0 && game_state.accuracy >= 95.0 {
        ("EXCELLENT", COLOR_GREEN, "⭐")
    } else if game_state.wpm >= 40.0 && game_state.accuracy >= 90.0 {
        ("GOOD", COLOR_BLUE, "✓")
    } else if game_state.accuracy >= 80.0 {
        ("KEEP GOING", COLOR_CYAN, "→")
    } else {
        ("TRY AGAIN", COLOR_RED, "↻")
    };

    let popup_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" 🎉 ", Style::default()),
            Span::styled("COMPLETE", Style::default().fg(COLOR_GREEN).bold()),
            Span::styled(" 🎉 ", Style::default()),
        ]))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_GREEN))
        .style(Style::default().bg(COLOR_SURFACE));

    let inner = popup_block.inner(popup);
    f.render_widget(popup_block, popup);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // Rating
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Stats
            Constraint::Length(1),  // Spacer
            Constraint::Length(1),  // Time
            Constraint::Min(1),     // Spacer
            Constraint::Length(1),  // Controls
        ])
        .split(inner);

    // Rating
    let rating_text = Paragraph::new(Line::from(vec![
        Span::styled(format!("{} ", rating_emoji), Style::default()),
        Span::styled(rating, Style::default().fg(rating_color).bold()),
        Span::styled(format!(" {}", rating_emoji), Style::default()),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(rating_text, inner_chunks[0]);

    // Stats row
    let stats_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(inner_chunks[2]);

    // WPM
    let wpm_color = if game_state.wpm >= 60.0 { COLOR_GREEN } else if game_state.wpm >= 40.0 { COLOR_BLUE } else { COLOR_YELLOW };
    let wpm_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(wpm_color));
    let wpm_inner = wpm_block.inner(stats_row[0]);
    f.render_widget(wpm_block, stats_row[0]);
    
    let wpm_text = Paragraph::new(Line::from(vec![
        Span::styled("⚡ ", Style::default().fg(wpm_color)),
        Span::styled(format!("{:.1}", game_state.wpm), Style::default().fg(wpm_color).bold()),
        Span::styled(" wpm", Style::default().fg(COLOR_GRAY)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(wpm_text, wpm_inner);

    // Accuracy
    let acc_color = if game_state.accuracy >= 95.0 { COLOR_GREEN } else if game_state.accuracy >= 85.0 { COLOR_YELLOW } else { COLOR_RED };
    let acc_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(acc_color));
    let acc_inner = acc_block.inner(stats_row[1]);
    f.render_widget(acc_block, stats_row[1]);
    
    let acc_text = Paragraph::new(Line::from(vec![
        Span::styled("🎯 ", Style::default().fg(acc_color)),
        Span::styled(format!("{:.1}%", game_state.accuracy), Style::default().fg(acc_color).bold()),
        Span::styled(" acc", Style::default().fg(COLOR_GRAY)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(acc_text, acc_inner);

    // Time
    let secs = elapsed.as_secs_f32();
    let time_text = Paragraph::new(Line::from(vec![
        Span::styled("⏱ ", Style::default().fg(COLOR_CYAN)),
        Span::styled(format!("{:.1}s", secs), Style::default().fg(COLOR_CYAN).bold()),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(time_text, inner_chunks[4]);

    // Controls
    let controls = Paragraph::new(Line::from(vec![
        Span::styled("↵", Style::default().fg(COLOR_BLUE).bold()),
        Span::styled(" next  ", Style::default().fg(COLOR_GRAY)),
        Span::styled("ESC", Style::default().fg(COLOR_RED).bold()),
        Span::styled(" quit", Style::default().fg(COLOR_GRAY)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(controls, inner_chunks[6]);
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

    let message = Paragraph::new(Line::from(vec![
        Span::styled("Are you sure?", Style::default().fg(COLOR_WHITE)),
    ]))
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
        let title_area = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        let value_area = Rect { x: inner.x, y: inner.y + 1, width: inner.width, height: 1 };
        f.render_widget(title_widget, title_area);
        f.render_widget(value_widget, value_area);
    }
}
