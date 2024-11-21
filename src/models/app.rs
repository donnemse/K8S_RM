use tui::{layout::Constraint, widgets::Row};

use crate::models::sort::SortConfig;

#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    Node,
    Pod,
    Namespace,
}

pub struct AppState {
    pub rows: Vec<Row<'static>>,
    pub is_loading: bool,
    pub view_mode: ViewMode,
    pub sort_config: SortConfig,
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
            scroll_offset: 0,
            visible_height: 0,
            selected_row: 0,
        }
    }

    pub fn get_widths(&self) -> Vec<Constraint> {
        match self.view_mode {
            ViewMode::Node => vec![
                Constraint::Length(20), // Node Name
                Constraint::Length(15), // CPU Allocated
                Constraint::Length(15), // Memory Allocated
                Constraint::Length(15), // CPU Request
                Constraint::Length(15), // CPU Limit
                Constraint::Length(15), // Memory Request
                Constraint::Length(15), // Memory Limit
            ],
            ViewMode::Pod => vec![
                Constraint::Length(20), // Namespace
                Constraint::Length(30), // Pod Name
                Constraint::Length(15), // Status
                Constraint::Length(15), // CPU Request
                Constraint::Length(15), // CPU Limit
                Constraint::Length(15), // Memory Request
                Constraint::Length(15), // Memory Limit
            ],
            ViewMode::Namespace => vec![
                Constraint::Length(30), // Namespace
                Constraint::Length(15), // CPU Request
                Constraint::Length(15), // CPU Limit
                Constraint::Length(15), // Memory Request
                Constraint::Length(15), // Memory Limit
            ],
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Node => ViewMode::Pod,
            ViewMode::Pod => ViewMode::Namespace,
            ViewMode::Namespace => ViewMode::Node,
        };
    }
}