use kube::{Api, Client};
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use tui::{
    widgets::{Cell, Row},
    style::{Color, Modifier, Style},
};
use crate::resources::pod_resources::PodResources; // 새로 추출한 모듈 사용
use crate::util::common::{format_cpu, format_memory};
use crate::AppError;
use crate::models::sort::SortConfig;

pub async fn handle_pod_command(sort_config: Option<SortConfig>) -> Result<Vec<Row<'static>>, AppError> {
    let client = Client::try_default().await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let pods: Api<Pod> = Api::all(client);

    let pod_list = pods.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let mut pod_rows = Vec::new();

    let mut total_resources = PodResources::new(); // Total 행을 위한 변수

    // Pod 데이터 수집
    for pod in pod_list {
        let namespace = pod.metadata.namespace.unwrap_or_default();
        let name = pod.metadata.name.unwrap_or_default();
        let status = pod
            .status
            .as_ref()
            .and_then(|s| s.phase.clone())
            .unwrap_or_default();

        let mut pod_resources = PodResources::new();

        if let Some(spec) = &pod.spec {
            for container in &spec.containers {
                if let Some(resources) = &container.resources {
                    pod_resources.add_container_resources(resources);
                }
            }
        }

        total_resources.add(&pod_resources);

        pod_rows.push((
            namespace,
            name,
            status,
            pod_resources.cpu_request,
            pod_resources.cpu_limit,
            pod_resources.memory_request,
            pod_resources.memory_limit,
        ));
    }

    // 정렬
    if let Some(sort_config) = sort_config {
        pod_rows.sort_by(|a, b| {
            let column_index = sort_config.column;
            let compare = match column_index {
                0 => a.0.cmp(&b.0), // Namespace
                1 => a.1.cmp(&b.1), // Pod Name
                2 => a.2.cmp(&b.2), // Status
                3 => a.3.0.cmp(&b.3.0), // CPU Request
                4 => a.4.0.cmp(&b.4.0), // CPU Limit
                5 => a.5.0.cmp(&b.5.0), // Memory Request
                6 => a.6.0.cmp(&b.6.0), // Memory Limit
                _ => std::cmp::Ordering::Equal,
            };

            match column_index {
                0 | 1 | 2 => compare,
                _ => {
                    compare.reverse()
                }
            }
        });
    }

    // Total 행 추가
    pod_rows.push((
        "TOTAL".to_string(),
        "".to_string(),
        "".to_string(),
        total_resources.cpu_request,
        total_resources.cpu_limit,
        total_resources.memory_request,
        total_resources.memory_limit,
    ));

    // 행 데이터 생성
    let header_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let row_style = Style::default().fg(Color::Gray);
    let total_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);

    let header_contents = vec![
        "Namespace", "Pod Name", "Status", "CPU Req.", "CPU Lim.", "Mem Req.", "Mem Lim.",
    ];

    let header = Row::new(
        header_contents
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let mut cell = Cell::from(*h).style(header_style);
                if let Some(sort_config) = sort_config {
                    if sort_config.column == i {
                        cell = cell.style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    }
                }
                cell
            })
            .collect::<Vec<Cell>>(),
    )
    .style(header_style);

    let rows = pod_rows.into_iter().map(|(namespace, name, status, cpu_request, cpu_limit, memory_request, memory_limit)| {
        let style = if namespace == "TOTAL" {
            total_style
        } else {
            row_style
        };
        Row::new(vec![
            Cell::from(namespace),
            Cell::from(name),
            Cell::from(status),
            Cell::from(format_cpu(cpu_request)),
            Cell::from(format_cpu(cpu_limit)),
            Cell::from(format_memory(memory_request)),
            Cell::from(format_memory(memory_limit)),
        ])
        .style(style)
    });

    Ok(std::iter::once(header).chain(rows).collect())
}