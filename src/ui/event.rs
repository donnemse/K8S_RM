use crate::ViewMode;
use crate::AppState;
use crate::fetch_data_with_sort;

use crossterm::event::{KeyCode, Event};
use tokio::sync::mpsc;
use tui::widgets::Row;

pub fn handle_event(
    event: Event,
    app_state: &mut AppState,
    tx: mpsc::Sender<Vec<Row<'static>>>,
) -> bool {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => {
                return false;
            }
            KeyCode::Char(' ') => {
                app_state.is_loading = true;
                let tx_clone = tx.clone();
                let current_mode = app_state.view_mode;
                let current_sort_config = app_state.sort_config; // 정렬 상태 포함
                tokio::spawn(async move {
                    if let Ok(data) = fetch_data_with_sort(current_mode, current_sort_config).await {
                        let _ = tx_clone.send(data).await;
                    }
                });
            }
            KeyCode::Tab => {
                app_state.toggle_view_mode();
                app_state.is_loading = true;
                app_state.selected_row = 0;
                app_state.scroll_offset = 0;
                let tx_clone = tx.clone();
                let current_mode = app_state.view_mode;
                let current_sort_config = app_state.sort_config; // 정렬 상태 포함
                tokio::spawn(async move {
                    if let Ok(data) = fetch_data_with_sort(current_mode, current_sort_config).await {
                        let _ = tx_clone.send(data).await;
                    }
                });
            }
            KeyCode::Left => {
                if app_state.sort_config.column > 0 {
                    app_state.sort_config.column -= 1;
                }
                app_state.is_loading = true;
            }
            KeyCode::Right => {
                let max_columns = match app_state.view_mode {
                    ViewMode::Node => 6,       // Node는 7개 컬럼
                    ViewMode::Pod => 6,        // Pod는 6개 컬럼
                    ViewMode::Namespace => 4,  // Namespace는 5개 컬럼
                };
                if app_state.sort_config.column < max_columns {
                    app_state.sort_config.column += 1;
                }
                app_state.is_loading = true;
            }
            KeyCode::Up => {
                if app_state.selected_row > 0 {
                    app_state.selected_row -= 1;
                    if app_state.selected_row < app_state.scroll_offset {
                        app_state.scroll_offset -= 1; // 스크롤 업
                    }
                }
            }
            KeyCode::Down => {
                if app_state.selected_row < app_state.rows.len() - 3 {
                    app_state.selected_row += 1;
                    if app_state.selected_row >= app_state.scroll_offset + app_state.visible_height {
                        app_state.scroll_offset += 1; // 스크롤 다운
                    }
                }
            }
            KeyCode::PageUp => {
                if app_state.scroll_offset > 0 {
                    let page_size = app_state.visible_height;
                    app_state.scroll_offset = app_state.scroll_offset.saturating_sub(page_size);
                    app_state.selected_row = app_state.selected_row.saturating_sub(page_size);
                }
            }
            KeyCode::PageDown => {
                if app_state.scroll_offset + app_state.visible_height < app_state.rows.len() -3 {
                    let page_size = app_state.visible_height;
                    app_state.scroll_offset = (app_state.scroll_offset + page_size).min(app_state.rows.len().saturating_sub(1));
                    app_state.selected_row = (app_state.selected_row + page_size).min(app_state.rows.len().saturating_sub(1));
                }
            }
            _ => {}
        }
    }
    true
}