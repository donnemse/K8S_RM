// node.rs
use kube::{Api, Client};
use k8s_openapi::api::core::v1::{Node, Pod};
use kube::api::ListParams;
use std::collections::HashMap;
use tui::{
    style::{Color, Modifier, Style}, widgets::{Cell, Row}
};

use crate::util::common::{format_cpu, format_memory};
use crate::models::error::AppError;
use crate::models::sort::SortConfig;
use crate::resources::node_resource::NodeResources;

pub async fn handle_node_command(sort_config: Option<SortConfig>) -> Result<Vec<Row<'static>>, AppError> {
    let client = Client::try_default().await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let nodes: Api<Node> = Api::all(client.clone());
    let pods: Api<Pod> = Api::all(client);

    let node_list = nodes.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let pod_list = pods.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;

    let mut node_data = Vec::new();

    let mut total_resources = NodeResources::new(); // Total 계산을 위한 변수

    // Pod 데이터를 노드별로 그룹화
    let pod_by_node: HashMap<String, Vec<Pod>> = pod_list.items.into_iter()
        .filter_map(|pod| {
            pod.spec.as_ref()
                .and_then(|spec| spec.node_name.clone())
                .map(|node_name| (node_name, pod))
        })
        .fold(HashMap::new(), |mut acc, (node_name, pod)| {
            acc.entry(node_name).or_default().push(pod);
            acc
        });

    // 노드별 데이터 처리
    for node in node_list {
        let name = node.metadata.name.clone().unwrap_or_default();
        let mut node_resources = NodeResources::new();

        if let Some(status) = &node.status {
            if let Some(allocatable) = &status.allocatable {
                node_resources.add_allocatable(allocatable);
            }
        }

        if let Some(pods) = pod_by_node.get(&name) {
            for pod in pods {
                if let Some(spec) = &pod.spec {
                    for container in &spec.containers {
                        if let Some(container_resources) = &container.resources {
                            node_resources.add_container_resources(container_resources);
                        }
                    }
                }
            }
        }

        // Total 업데이트
        total_resources.allocatable_cpu.0 += node_resources.allocatable_cpu.0;
        total_resources.allocatable_memory.0 += node_resources.allocatable_memory.0;
        total_resources.cpu_request.0 += node_resources.cpu_request.0;
        total_resources.cpu_limit.0 += node_resources.cpu_limit.0;
        total_resources.memory_request.0 += node_resources.memory_request.0;
        total_resources.memory_limit.0 += node_resources.memory_limit.0;

        // 데이터 저장
        node_data.push((
            name.clone(),
            node_resources.allocatable_cpu,
            node_resources.allocatable_memory,
            node_resources.cpu_request,
            node_resources.cpu_limit,
            node_resources.memory_request,
            node_resources.memory_limit,
        ));
    }

    // 정렬
    if let Some(sort_config) = sort_config {
        node_data.sort_by(|a, b| {
            let column_index = sort_config.column;
            let compare = match column_index {
                0 => a.0.cmp(&b.0), // Node Name
                1 => a.1.0.cmp(&b.1.0), // CPU Alloc
                2 => a.2.0.cmp(&b.2.0), // Memory Alloc
                3 => a.3.0.cmp(&b.3.0), // CPU Request
                4 => a.4.0.cmp(&b.4.0), // CPU Limit
                5 => a.5.0.cmp(&b.5.0), // Memory Request
                6 => a.6.0.cmp(&b.6.0), // Memory Limit
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
    node_data.push((
        "TOTAL".to_string(),
        total_resources.allocatable_cpu,
        total_resources.allocatable_memory,
        total_resources.cpu_request,
        total_resources.cpu_limit,
        total_resources.memory_request,
        total_resources.memory_limit,
    ));

    // 테이블 데이터 생성
    let header_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let row_style = Style::default().fg(Color::Gray);
    let total_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD); // Total 행 스타일 수정

    let header_contents = vec![
        "Node Name", "CPU Alloc.", "Memory Alloc.", "CPU Req.", "CPU Lim.", "Mem Req.", "Mem Lim.",
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

    let rows = node_data.into_iter().map(|(name, cpu_alloc, mem_alloc, cpu_req, cpu_lim, mem_req, mem_lim)| {
        let style = if name == "TOTAL" { // Total 행인지 확인
            total_style
        } else {
            row_style
        };
        Row::new(vec![
            Cell::from(name),
            Cell::from(format_cpu(cpu_alloc)),
            Cell::from(format_memory(mem_alloc)),
            Cell::from(format_cpu(cpu_req)),
            Cell::from(format_cpu(cpu_lim)),
            Cell::from(format_memory(mem_req)),
            Cell::from(format_memory(mem_lim)),
        ])
        .style(style)
    });

    Ok(std::iter::once(header).chain(rows).collect())
}