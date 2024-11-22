use std::cmp::Ordering;

use kube::{Api, Client};
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use crate::models::resource::Resources;
use crate::util::common::{format_cpu, format_memory};
use crate::AppError;
use crate::models::config::SortConfig;
use crate::models::config::SearchConfig;

pub async fn handle_pod_command(search_config: Option<SearchConfig>, sort_config: Option<SortConfig>) -> Result<Vec<Vec<String>>, AppError> {
    let client = Client::try_default().await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let pods: Api<Pod> = Api::all(client);

    let pod_list = pods.list(&ListParams::default()).await.map_err(|e| AppError::KubeError(e.to_string()))?;
    let mut pod_rows = Vec::new();

    let mut total_resources = Resources::new();

    // Pod 데이터 수집
    for pod in pod_list {
        let namespace = pod.metadata.namespace.unwrap_or_default();
        let name = pod.metadata.name.unwrap_or_default();
        let status = pod
            .status
            .as_ref()
            .and_then(|s| s.phase.clone())
            .unwrap_or_default();
        let node = pod
            .spec
            .as_ref()
            .and_then(|spec| spec.node_name.clone())
            .unwrap_or_default();

        // 정렬
        if let Some(search_config) = search_config.clone() {
            let column_index = search_config.column;
            let word = search_config.get_word();
            let matched = match column_index {
                0 => namespace == word,
                1 => name == word,
                2 => status == word,
                3 => node == word,
                _ => true,
            };

            if !matched {
                continue;
            }
        }

        let mut pod_resources = Resources::new();

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
            node,
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
                0 => a.0.cmp(&b.0),
                1 => a.1.cmp(&b.1),
                2 => a.2.cmp(&b.2),
                3 => a.3.cmp(&b.3),
                4 => a.4.0.cmp(&b.4.0),
                5 => a.5.0.cmp(&b.5.0),
                6 => a.6.0.cmp(&b.6.0),
                7 => a.7.0.cmp(&b.7.0),
                _ => Ordering::Equal,
            };

            match column_index {
                0 | 1 | 2 | 3 => compare,
                _ => compare.reverse()
            }
        });
    }

    // Total 행 추가
    pod_rows.push((
        "TOTAL".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        total_resources.cpu_request,
        total_resources.cpu_limit,
        total_resources.memory_request,
        total_resources.memory_limit,
    ));

    // 헤더 추가 및 데이터 변환
    let mut result = Vec::new();
    
    // 헤더 추가
    result.push(vec![
        "Namespace".to_string(),
        "Pod Name".to_string(),
        "Status".to_string(),
        "Node".to_string(),
        "CPU Req.".to_string(),
        "CPU Lim.".to_string(),
        "Mem Req.".to_string(),
        "Mem Lim.".to_string(),
    ]);

    // 데이터 행 추가
    for (namespace, name, status, node, cpu_request, cpu_limit, memory_request, memory_limit) in pod_rows {
        result.push(vec![
            namespace,
            name,
            status,
            node,
            format_cpu(cpu_request),
            format_cpu(cpu_limit),
            format_memory(memory_request),
            format_memory(memory_limit),
        ]);
    }

    Ok(result)
}