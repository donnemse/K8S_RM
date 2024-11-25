use tui::{
    backend::Backend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::Spans, widgets::{Block, Borders, Cell, Paragraph, Row, Table}, Frame
};
use crate::{models::app::ViewMode, AppState};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(f.size());

    let banner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[0]);

    let table_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(chunks[1]);

    app_state.visible_height = (table_chunks[0].height as usize).saturating_sub(2);

    let table_widths = app_state.get_widths();

    if !app_state.rows.is_empty() {
        let header_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let header = Row::new(
            app_state.rows[0]
                .iter()
                .skip(app_state.scroll_offset_horizontal) // 가로 스크롤 적용
                .enumerate()
                .map(|(i, h)| {
                    let mut cell = Cell::from(h.as_str()).style(header_style);
                    if app_state.sort_config.column == i + app_state.scroll_offset_horizontal {
                        cell = cell.style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    }
                    cell
                })
                .collect::<Vec<Cell>>(),
        )
        .style(header_style);

        let mut visible_rows = vec![header];
        visible_rows.extend(
            app_state.rows
                .iter()
                .take(app_state.rows.len() - 1)
                .skip(1 + app_state.scroll_offset)
                .take(app_state.visible_height)
                .enumerate()
                .map(|(i, row)| {
                    let style = if i + app_state.scroll_offset == app_state.selected_row {
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    };
                    let cells: Vec<tui::widgets::Cell> = row
                        .iter()
                        .skip(app_state.scroll_offset_horizontal) // 가로 스크롤 적용
                        .map(|s| tui::widgets::Cell::from(s.as_str()))
                        .collect();
                    Row::new(cells).style(style)
                })
        );

        let title = match app_state.view_mode {
            ViewMode::Namespace => "Namespace".to_string(),
            ViewMode::Node => "Node".to_string(),
            ViewMode::Pod => match app_state.search_config.column {
                999 => "Pod".to_string(),
                _ => {
                    let column = app_state.rows[0][app_state.search_config.column].clone();
                    let value = app_state.search_config.get_word();
                    format!("Pod - Filtered -> {}: {}", column, value)
                }
            }
        };

        let table = Table::new(visible_rows)
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::RIGHT | Borders::LEFT)
                    .title(title).style(Style::default().fg(Color::Cyan)))
            .widths(&table_widths);

        f.render_widget(table, table_chunks[0]);
    }

    if app_state.rows.len() > 1 {
        let total_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
        let total_cells: Vec<tui::widgets::Cell> = app_state.rows.last().unwrap()
            .iter()
            .skip(app_state.scroll_offset_horizontal) // 가로 스크롤 적용
            .map(|s| tui::widgets::Cell::from(s.as_str()))
            .collect();
        let total_row = Row::new(total_cells).style(total_style);

        let total_table = Table::new(vec![total_row])
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                    .style(Style::default().fg(Color::Cyan)))
            .widths(&table_widths);

        f.render_widget(total_table, table_chunks[1]);
    }

    let banner_text = vec![
        Spans::from("██╗░░██╗██╗░░░██╗██████╗░███████╗░░░░░░██████╗░███╗░░░███╗"),
        Spans::from("██║░██╔╝██║░░░██║██╔══██╗██╔════╝░░░░░░██╔══██╗████╗░████║"),
        Spans::from("█████═╝░██║░░░██║██████╦╝█████╗░░█████╗██████╔╝██╔████╔██║"),
        Spans::from("██╔═██╗░██║░░░██║██╔══██╗██╔══╝░░╚════╝██╔══██╗██║╚██╔╝██║"),
        Spans::from("██║░╚██╗╚██████╔╝██████╦╝███████╗░░░░░░██║░░██║██║░╚═╝░██║"),
        Spans::from("╚═╝░░╚═╝░╚═════╝░╚═════╝░╚══════╝░░░░░░╚═╝░░╚═╝╚═╝░░░░░╚═╝"),
        Spans::from(""),
        Spans::from("↑/↓: Scroll | ←/→: Sort | Tab: Move | Space Bar : refresh | q: Quit"),
    ];

    let version = env!("CARGO_PKG_VERSION");
    let team_text = vec![
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from("Develop by Data Platform team  "),
        Spans::from("         (dev.dp@igloo.co.kr)  "),
        Spans::from(format!("v{}  ", version)),
    ];

    f.render_widget(
        Paragraph::new(banner_text).block(Block::default().borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)),
        banner_chunks[0],
    );
    f.render_widget(
        Paragraph::new(team_text).block(Block::default().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT))
        .alignment(Alignment::Right),
        banner_chunks[1],
    );
}