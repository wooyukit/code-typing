use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, LineGauge, Padding, Paragraph, Wrap},
    Frame,
};
use std::time::Instant;

use super::GameState;

// Color palette for a cohesive look
const COLOR_PRIMARY: Color = Color::Rgb(138, 180, 248);    // Soft blue
const COLOR_SECONDARY: Color = Color::Rgb(129, 199, 132);  // Soft green
const COLOR_ACCENT: Color = Color::Rgb(255, 213, 79);      // Warm yellow
const COLOR_ERROR: Color = Color::Rgb(239, 83, 80);        // Soft red
const COLOR_DIM: Color = Color::Rgb(100, 100, 100);        // Gray
const COLOR_BG_DARK: Color = Color::Rgb(30, 30, 46);       // Dark background

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
    // TITLE SECTION
    // ═══════════════════════════════════════════════════════════════════════
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_PRIMARY));

    let title_text = vec![
        Span::styled("⌨  ", Style::default().fg(COLOR_ACCENT)),
        Span::styled("CODE", Style::default().fg(COLOR_PRIMARY).bold()),
        Span::styled(" TYPING ", Style::default().fg(Color::White).bold()),
        Span::styled("GAME", Style::default().fg(COLOR_SECONDARY).bold()),
        Span::styled("  ⌨", Style::default().fg(COLOR_ACCENT)),
    ];

    let title = Paragraph::new(Line::from(title_text))
        .block(title_block)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // ═══════════════════════════════════════════════════════════════════════
    // CODE SECTION
    // ═══════════════════════════════════════════════════════════════════════
    let code_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)])
        .split(chunks[1]);

    // Code block with rounded borders
    let code_block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled("📝", Style::default()),
            Span::styled(" CODE ", Style::default().fg(COLOR_ACCENT).bold()),
        ]))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_ACCENT))
        .padding(Padding::new(2, 2, 1, 1));

    let mut code_lines = Vec::new();
    let mut char_index = 0;
    let mut cursor_pos = None;

    for (_line_idx, line) in game_state.current_code.lines().enumerate() {
        let mut line_spans = Vec::new();
        for ch in line.chars() {
            if char_index == game_state.user_input.len() && !game_state.game_over {
                cursor_pos = Some(line_spans.len());
            }

            let style = if char_index < game_state.user_input.len() {
                let user_char = game_state.user_input.chars().nth(char_index).unwrap_or(' ');
                if user_char == ch {
                    Style::default().fg(COLOR_SECONDARY).bold()
                } else {
                    Style::default().fg(COLOR_ERROR).bold()
                }
            } else if char_index == game_state.user_input.len() && !game_state.game_over {
                Style::default().fg(COLOR_BG_DARK).bg(COLOR_ACCENT).bold()
            } else {
                Style::default().fg(COLOR_DIM)
            };
            line_spans.push(Span::styled(ch.to_string(), style));
            char_index += 1;
        }

        if char_index == game_state.user_input.len()
            && cursor_pos.is_none()
            && !game_state.game_over
        {
            cursor_pos = Some(line_spans.len());
            line_spans.push(Span::styled(
                "▋",
                Style::default().fg(COLOR_ACCENT).add_modifier(Modifier::SLOW_BLINK),
            ));
        }

        code_lines.push(Line::from(line_spans));
        char_index += 1;
    }

    let code_display = Paragraph::new(code_lines)
        .block(code_block)
        .wrap(Wrap { trim: false });
    f.render_widget(code_display, code_area_chunks[0]);

    // Progress bar using LineGauge for a sleeker look
    let progress = game_state.user_input.len() as f64 / game_state.current_code.len() as f64;
    let progress_label = format!(
        " {} / {} chars  ({}%) ",
        game_state.user_input.len(),
        game_state.current_code.len(),
        (progress * 100.0) as u32
    );

    let progress_gauge = LineGauge::default()
        .filled_style(Style::default().fg(COLOR_SECONDARY))
        .unfilled_style(Style::default().fg(COLOR_DIM))
        .line_set(symbols::line::THICK)
        .ratio(progress)
        .label(Span::styled(progress_label, Style::default().fg(Color::White)));
    f.render_widget(progress_gauge, code_area_chunks[1]);

    // ═══════════════════════════════════════════════════════════════════════
    // STATS SECTION
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

    let total_millis = elapsed.as_millis();
    let minutes = total_millis / 60000;
    let seconds = (total_millis % 60000) / 1000;
    let millis = total_millis % 1000;
    let timer_str = format!("{}:{:02}.{:03}", minutes, seconds, millis);

    // WPM Card
    render_stat_card(
        f,
        stats_chunks[0],
        "⚡ WPM",
        &format!("{:.1}", game_state.wpm),
        COLOR_PRIMARY,
        game_state.wpm > 40.0,
    );

    // Accuracy Card
    let acc_color = if game_state.accuracy >= 95.0 {
        COLOR_SECONDARY
    } else if game_state.accuracy >= 80.0 {
        COLOR_ACCENT
    } else {
        COLOR_ERROR
    };
    render_stat_card(
        f,
        stats_chunks[1],
        "🎯 ACC",
        &format!("{:.1}%", game_state.accuracy),
        acc_color,
        game_state.accuracy >= 95.0,
    );

    // Time Card
    render_stat_card(
        f,
        stats_chunks[2],
        "⏱  TIME",
        &timer_str,
        COLOR_ACCENT,
        false,
    );

    // Controls overlay at bottom
    let controls_area = Rect {
        x: chunks[2].x,
        y: chunks[2].y + chunks[2].height,
        width: chunks[2].width,
        height: 1,
    };

    let controls_text = if game_state.game_over {
        vec![
            Span::styled(" ✓ COMPLETE ", Style::default().fg(COLOR_BG_DARK).bg(COLOR_SECONDARY).bold()),
            Span::raw("  "),
            Span::styled("ENTER", Style::default().fg(COLOR_PRIMARY).bold()),
            Span::raw(" next  "),
            Span::styled("ESC", Style::default().fg(COLOR_ERROR).bold()),
            Span::raw(" quit"),
        ]
    } else {
        vec![
            Span::styled("ENTER", Style::default().fg(COLOR_SECONDARY).bold()),
            Span::raw(" auto-indent  "),
            Span::styled("TAB", Style::default().fg(COLOR_PRIMARY).bold()),
            Span::raw(" indent  "),
            Span::styled("BACKSPACE", Style::default().fg(COLOR_ACCENT).bold()),
            Span::raw(" delete  "),
            Span::styled("ESC", Style::default().fg(COLOR_ERROR).bold()),
            Span::raw(" quit"),
        ]
    };

    let controls = Paragraph::new(Line::from(controls_text)).alignment(Alignment::Center);
    f.render_widget(controls, controls_area);
}

fn render_stat_card(f: &mut Frame, area: Rect, title: &str, value: &str, color: Color, highlight: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Title
    let title_line = Line::from(Span::styled(
        title,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ));
    let title_widget = Paragraph::new(title_line).alignment(Alignment::Center);

    // Value
    let value_style = if highlight {
        Style::default().fg(color).bold()
    } else {
        Style::default().fg(Color::White).bold()
    };
    let value_line = Line::from(Span::styled(value, value_style));
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
