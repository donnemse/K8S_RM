use tui::layout::Constraint;
use crate::models::config::{SortConfig, SearchConfig};

#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    Node,
    Pod,
    Namespace,
}

pub struct AppState {
    pub rows: Vec<Vec<String>>,
    pub is_loading: bool,
    pub view_mode: ViewMode,
    pub sort_config: SortConfig,
    pub search_config: SearchConfig,
    pub scroll_offset_horizontal: usize,
    pub scroll_offset: usize,
    pub visible_height: usize,
    pub selected_row: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            is_loading: false,
            view_mode: ViewMode::Node,
            sort_config: SortConfig::new(0),
            search_config: SearchConfig::new(999, ""),
            scroll_offset_horizontal: 0,
            scroll_offset: 0,
            visible_height: 0,
            selected_row: 0,
        }
    }

    pub fn get_widths(&self) -> Vec<Constraint> {
        // 터미널 너비와 여백 설정
        let terminal_width = self.get_table_area_width() as usize;
        let horizontal_padding = 4; // 고정된 좌우 여백
        let available_width = terminal_width.saturating_sub(horizontal_padding);
    
        // 현재 ViewMode에 따른 기본 컬럼 너비
        let mut widths = match self.view_mode {
            ViewMode::Node => vec![20, 15, 15, 15, 15, 15, 15], // Node
            ViewMode::Pod => vec![20, 35, 15, 25, 15, 15, 15, 15], // Pod
            ViewMode::Namespace => vec![30, 15, 15, 15, 15], // Namespace
        };
    
        // 모든 컬럼 너비의 합 계산
        let total_width: usize = widths.iter().sum();
    
        // 필요할 경우 너비 조정
        if available_width < total_width {
            let excess = total_width.saturating_sub(available_width);
    
            // 각 컬럼의 너비를 균등하게 줄임 (최소 5를 유지)
            let decrement_per_column = (excess as f64 / widths.len() as f64).ceil() as usize;
    
            widths.iter_mut().for_each(|w| {
                if *w > decrement_per_column + 5 {
                    *w -= decrement_per_column;
                }
            });
        }
    
        // `Constraint::Length`로 변환하여 반환
        widths.iter().map(|&w| Constraint::Length(w as u16)).collect()
    }

    fn get_table_area_width(&self) -> u16 {
        let terminal_size = crossterm::terminal::size().unwrap_or((80, 25)); // 기본 크기: 80x25
        terminal_size.0 // 가로 크기 반환
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Node => ViewMode::Pod,
            ViewMode::Pod => ViewMode::Namespace,
            ViewMode::Namespace => ViewMode::Node,
        };
    }
}