use crate::models::config::SearchConfig;
use crate::ViewMode;
use crate::AppState;

use crossterm::event::{KeyCode, Event};

pub fn handle_event(
    event: Event,
    app_state: &mut AppState
) -> bool {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => {
                return false;
            }
            KeyCode::Char(' ') => {
                app_state.is_loading = true;
            }
            KeyCode::Esc => {
                app_state.is_loading = true;
                app_state.search_config = SearchConfig::new(999, "");
            }
            KeyCode::Tab => {
                app_state.toggle_view_mode();
                app_state.is_loading = true;
                app_state.selected_row = 0;
                app_state.scroll_offset = 0;
                app_state.sort_config.column = 0;
                app_state.search_config = SearchConfig::new(999, "");

            }
            KeyCode::Enter => {
                if let Some(row) = app_state.rows.get(app_state.selected_row + 1) {
                    let column_index = match app_state.view_mode {
                        ViewMode::Node => 3,
                        ViewMode::Namespace => 0,
                        _ => return true
                    };
                    app_state.is_loading = true;
                    app_state.selected_row = 0;
                    app_state.scroll_offset = 0;
                    app_state.view_mode = ViewMode::Pod;
                    app_state.search_config.set_word(row[0].as_str());
                    app_state.search_config.column = column_index;
                }
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
                    ViewMode::Pod => 7,        // Pod는 6개 컬럼
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