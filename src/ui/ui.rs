use tui::{
    backend::Backend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::Spans, widgets::{Block, Borders, Paragraph, Table}, Frame
};
use crate::AppState;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // 전체 헤더
            Constraint::Min(0),    // Table (Header + Body 포함)
        ])
        .split(f.size());

    let banner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50), // 오른쪽에 팀 정보
        ].as_ref())
        .split(chunks[0]);

    let table_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Body
            Constraint::Length(3), // Total
        ])
        .split(chunks[1]);

    app_state.visible_height = (table_chunks[0].height as usize).saturating_sub(2); // 테이블 크기 계산

    let table_widths = app_state.get_widths();
    if !app_state.rows.is_empty() {
        // 항상 첫 번째 행(Header)을 포함하여 스크롤 적용
        let mut visible_rows = vec![app_state.rows[0].clone()]; // Header 고정
        visible_rows.extend(
            app_state
                .rows
                .iter()
                .take(app_state.rows.len() -1)
                .skip(1 + app_state.scroll_offset) 
                .take(app_state.visible_height) 
                .cloned()
                .enumerate()
                .map(|(i, row)| {
                    // 현재 선택된 행 강조
                    let style = if i + app_state.scroll_offset == app_state.selected_row {
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    row.clone().style(style)
                }),
        );

        let table = Table::new(visible_rows)
        .block(Block::default().borders(Borders::TOP | Borders::RIGHT | Borders::LEFT))
            .widths(&table_widths);

        f.render_widget(table, table_chunks[0]); // Header와 Body를 함께 렌더링
    }

    // Total 렌더링
    if app_state.rows.len() > 1 {
        let total_row = app_state.rows.last().unwrap().clone(); // Total은 마지막 행
        let total_table = Table::new(vec![total_row])
            .block(Block::default().borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT))
            .widths(&table_widths);

        f.render_widget(total_table, table_chunks[1]); // Total을 고정 영역에 렌더링
    }

    let banner_text = vec![
        Spans::from("██╗░██████╗░██╗░░░░░░█████╗░░█████╗░  ░█████╗░░█████╗░██████╗░██████╗░░░░"),
        Spans::from("██║██╔════╝░██║░░░░░██╔══██╗██╔══██╗  ██╔══██╗██╔══██╗██╔══██╗██╔══██╗░░░"),
        Spans::from("██║██║░░██╗░██║░░░░░██║░░██║██║░░██║  ██║░░╚═╝██║░░██║██████╔╝██████╔╝░░░"),
        Spans::from("██║██║░░╚██╗██║░░░░░██║░░██║██║░░██║  ██║░░██╗██║░░██║██╔══██╗██╔═══╝░░░░"),
        Spans::from("██║╚██████╔╝███████╗╚█████╔╝╚█████╔╝  ╚█████╔╝╚█████╔╝██║░░██║██║░░░░░██╗"),
        Spans::from("╚═╝░╚═════╝░╚══════╝░╚════╝░░╚════╝░  ░╚════╝░░╚════╝░╚═╝░░╚═╝╚═╝░░░░░╚═╝"),
        Spans::from(""),
        Spans::from("↑/↓: Scroll | ←/→: Sort | Tab: Move | Space Bar : refresh | q: Quit"),
    ];

    let team_text = vec![
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from("Develop by Data Platform team  "),
        Spans::from("          dev.dp@igloo.co.kr   "),
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