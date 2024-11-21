use kube::{Api, Client};
use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::api::ListParams;
use std::collections::HashMap;
use tui::{
    widgets::{Cell, Row},
    style::{Color, Modifier, Style},
};
use crate::resources::namespace_resource::NamespaceResources;
use crate::util::common::{format_cpu, format_memory};
use crate::AppError;
use crate::models::sort::SortConfig;

async fn collect_namespace_resources(
    pod_list: Vec<Pod>
) -> HashMap<String, NamespaceResources> {
    let mut namespace_resources: HashMap<String, NamespaceResources> = HashMap::new();

    for pod in pod_list {
        if let Some(namespace) = &pod.metadata.namespace {
            let resources = namespace_resources
                .entry(namespace.clone())
                .or_insert_with(NamespaceResources::new);

            if let Some(spec) = &pod.spec {
                for container in &spec.containers {
                    if let Some(container_resources) = &container.resources {
                        resources.add_container_resources(container_resources);
                    }
                }
            }
        }
    }
    namespace_resources
}

pub async fn handle_namespace_command(sort_config: Option<SortConfig>) -> Result<Vec<Row<'static>>, AppError> {
    let client = Client::try_default()
        .await
        .map_err(|e| AppError::KubeError(e.to_string()))?;

    let namespaces: Api<Namespace> = Api::all(client.clone());
    let pods: Api<Pod> = Api::all(client);

    let namespace_list = namespaces.list(&ListParams::default())
        .await
        .map_err(|e| AppError::KubeError(e.to_string()))?;
    
    let pod_list = pods.list(&ListParams::default())
        .await
        .map_err(|e| AppError::KubeError(e.to_string()))?;

    let pod_items = pod_list.items;
    let namespace_resources = collect_namespace_resources(pod_items).await;

    let mut table_rows = Vec::new();
    let mut total_resources = NamespaceResources::new(); // Total 행을 위한 변수

    for ns in namespace_list {
        let namespace_name = ns.metadata.name.unwrap_or_default();
        let resources = namespace_resources
            .get(&namespace_name)
            .cloned()
            .unwrap_or_else(NamespaceResources::new);

        total_resources.add(&resources);
        table_rows.push((namespace_name, resources));
    }

    // 정렬
    if let Some(sort_config) = sort_config {
        table_rows.sort_by(|a, b| {
            let column_index = sort_config.column;
            let compare = match column_index {
                0 => a.0.cmp(&b.0), // Namespace Name
                1 => a.1.cpu_request.0.cmp(&b.1.cpu_request.0), // CPU Request
                2 => a.1.cpu_limit.0.cmp(&b.1.cpu_limit.0), // CPU Limit
                3 => a.1.memory_request.0.cmp(&b.1.memory_request.0), // Memory Request
                4 => a.1.memory_limit.0.cmp(&b.1.memory_limit.0), // Memory Limit
                _ => std::cmp::Ordering::Equal,
            };

            match column_index {
                0 => compare,
                _ => {
                    compare.reverse()
                }
            }
        });
    }

    // Total 행 추가
    table_rows.push((
        "TOTAL".to_string(),
        total_resources,
    ));

    // 테이블 생성
    let header_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let row_style = Style::default().fg(Color::Gray);
    let total_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);

    let header_contents = vec![
        "Namespace", "CPU Req.", "CPU Lim.", "Mem Req.", "Mem Lim.",
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

    let rows = table_rows.into_iter().map(|(namespace_name, resources)| {
        let style = if namespace_name == "TOTAL" {
            total_style
        } else {
            row_style
        };
        Row::new(vec![
            Cell::from(namespace_name),
            Cell::from(format_cpu(resources.cpu_request)),
            Cell::from(format_cpu(resources.cpu_limit)),
            Cell::from(format_memory(resources.memory_request)),
            Cell::from(format_memory(resources.memory_limit)),
        ])
        .style(style)
    });

    Ok(std::iter::once(header).chain(rows).collect())
    // Ok(table_rows)
}