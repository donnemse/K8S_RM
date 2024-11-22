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
use crate::resources::resource::NodeResources;

pub async fn handle_node_command(sort_config: Option<SortConfig>) -> Result<Vec<Vec<String>>, AppError> {
    let client = Client::try_default().await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let nodes: Api<Node> = Api::all(client.clone());
    let pods: Api<Pod> = Api::all(client);

    let node_list = nodes.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let pod_list = pods.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;

    let mut node_data = Vec::new();
    let mut total_resources = NodeResources::new();

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
        total_resources.base.cpu_request.0 += node_resources.base.cpu_request.0;
        total_resources.base.cpu_limit.0 += node_resources.base.cpu_limit.0;
        total_resources.base.memory_request.0 += node_resources.base.memory_request.0;
        total_resources.base.memory_limit.0 += node_resources.base.memory_limit.0;

        node_data.push((
            name.clone(),
            node_resources.allocatable_cpu,
            node_resources.allocatable_memory,
            node_resources.base.cpu_request,
            node_resources.base.cpu_limit,
            node_resources.base.memory_request,
            node_resources.base.memory_limit,
        ));
    }

    // 정렬
    if let Some(sort_config) = sort_config {
        node_data.sort_by(|a, b| {
            let column_index = sort_config.column;
            let compare = match column_index {
                0 => a.0.cmp(&b.0),
                1 => a.1.0.cmp(&b.1.0),
                2 => a.2.0.cmp(&b.2.0),
                3 => a.3.0.cmp(&b.3.0),
                4 => a.4.0.cmp(&b.4.0),
                5 => a.5.0.cmp(&b.5.0),
                6 => a.6.0.cmp(&b.6.0),
                _ => std::cmp::Ordering::Equal,
            };

            match column_index {
                0 => compare,
                _ => compare.reverse()
            }
        });
    }

    // Total 행 추가
    node_data.push((
        "TOTAL".to_string(),
        total_resources.allocatable_cpu,
        total_resources.allocatable_memory,
        total_resources.base.cpu_request,
        total_resources.base.cpu_limit,
        total_resources.base.memory_request,
        total_resources.base.memory_limit,
    ));

    // 결과 데이터 생성
    let mut result = Vec::new();
    
    // 헤더 추가
    result.push(vec![
        "Node Name".to_string(),
        "CPU Alloc.".to_string(),
        "Memory Alloc.".to_string(),
        "CPU Req.".to_string(),
        "CPU Lim.".to_string(),
        "Mem Req.".to_string(),
        "Mem Lim.".to_string(),
    ]);

    // 데이터 행 추가
    for (name, cpu_alloc, mem_alloc, cpu_req, cpu_lim, mem_req, mem_lim) in node_data {
        result.push(vec![
            name,
            format_cpu(cpu_alloc),
            format_memory(mem_alloc),
            format_cpu(cpu_req),
            format_cpu(cpu_lim),
            format_memory(mem_req),
            format_memory(mem_lim),
        ]);
    }

    Ok(result)
}