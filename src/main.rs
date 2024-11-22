mod util;
mod api;
mod ui;
mod models;
mod resources;

use std::{io, path::Path};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc;
use tui::{
    backend::CrosstermBackend, Terminal
};

use models::app::{AppState, ViewMode};
use ui::ui::draw_ui;
use ui::event::handle_event;

use crate::models::error::AppError;

struct TerminalSetup {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalSetup {
    fn new() -> Result<Self, AppError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl Drop for TerminalSetup {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}



async fn fetch_data_with_sort(
    view_mode: ViewMode,
    search_config: models::search::SearchConfig,
    sort_config: models::sort::SortConfig,
) -> Result<Vec<Vec<String>>, AppError> {
    match view_mode {
        ViewMode::Node => api::node::handle_node_command(Some(sort_config))
            .await
            .map_err(|e| AppError::KubeError(e.to_string())),
        ViewMode::Pod => api::pod::handle_pod_command(Some(search_config), Some(sort_config))
            .await
            .map_err(|e| AppError::KubeError(e.to_string())), // 추후 Pod도 정렬 추가 가능
        ViewMode::Namespace => api::namespace::handle_namespace_command(Some(sort_config))
            .await
            .map_err(|e| AppError::KubeError(e.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {

    let kube_config_path = dirs::home_dir()
        .map(|home| home.join(".kube/config"))
        .ok_or_else(|| AppError::IoError("Unable to determine home directory".to_string()))?;

    if !Path::new(&kube_config_path).exists() {
        eprintln!("Error: Kubernetes config file not found at {:?}", kube_config_path);
        return Err(AppError::IoError("Kubernetes config file is missing".to_string()));
    }

    let mut terminal_setup = TerminalSetup::new()?;
    let mut app_state = AppState::new();
    let (tx, mut rx) = mpsc::channel(100);

    // 초기 데이터 로드
    app_state.is_loading = true;
    let tx_clone = tx.clone();
    let initial_mode = app_state.view_mode;
    let current_sort_config = app_state.sort_config;
    let current_search_config = app_state.search_config.clone();
    tokio::spawn(async move {
        if let Ok(data) = fetch_data_with_sort(initial_mode, current_search_config, current_sort_config).await {
            let _ = tx_clone.send(data).await;
        }
    });

    loop {
        if app_state.is_loading {
            app_state.is_loading = false; // 플래그 해제
            let tx_clone = tx.clone();
            let current_mode = app_state.view_mode;
            let current_sort_config = app_state.sort_config; // 정렬 상태 전달
            let current_search_config = app_state.search_config.clone();
            tokio::spawn(async move {
                if let Ok(data) = fetch_data_with_sort(current_mode, current_search_config, current_sort_config).await {
                    let _ = tx_clone.send(data).await;
                }
            });
        }
        // 데이터 업데이트 처리
        if let Ok(data) = rx.try_recv() {
            app_state.rows = data;
            app_state.is_loading = false;
            app_state.scroll_offset = 0; // 스크롤 초기화
        }

        terminal_setup.terminal.draw(|f| draw_ui(f, &mut app_state))?;

        // 이벤트 처리
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Ok(data) = rx.try_recv() {
                app_state.rows = data;
                app_state.is_loading = false;
            }
            let event = crossterm::event::read()?;
            if !handle_event(event, &mut app_state, tx.clone()) {
                break;
            }
        }
    }
    Ok(())
}