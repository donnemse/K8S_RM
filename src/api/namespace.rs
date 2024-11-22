use kube::{Api, Client};
use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::api::ListParams;
use std::collections::HashMap;
use crate::resources::resource::Resources;
use crate::util::common::{format_cpu, format_memory};
use crate::AppError;
use crate::models::sort::SortConfig;

async fn collect_namespace_resources(
    pod_list: Vec<Pod>
) -> HashMap<String, Resources> {
    let mut namespace_resources: HashMap<String, Resources> = HashMap::new();

    for pod in pod_list {
        if let Some(namespace) = &pod.metadata.namespace {
            let resources = namespace_resources
                .entry(namespace.clone())
                .or_insert_with(Resources::new);

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

pub async fn handle_namespace_command(sort_config: Option<SortConfig>) -> Result<Vec<Vec<String>>, AppError> {
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
    let mut total_resources = Resources::new();

    for ns in namespace_list {
        let namespace_name = ns.metadata.name.unwrap_or_default();
        let resources = namespace_resources
            .get(&namespace_name)
            .cloned()
            .unwrap_or_else(Resources::new);

        total_resources.add(&resources);
        table_rows.push((namespace_name, resources));
    }

    // 정렬
    if let Some(sort_config) = sort_config {
        table_rows.sort_by(|a, b| {
            let column_index = sort_config.column;
            let compare = match column_index {
                0 => a.0.cmp(&b.0),
                1 => a.1.cpu_request.0.cmp(&b.1.cpu_request.0),
                2 => a.1.cpu_limit.0.cmp(&b.1.cpu_limit.0),
                3 => a.1.memory_request.0.cmp(&b.1.memory_request.0),
                4 => a.1.memory_limit.0.cmp(&b.1.memory_limit.0),
                _ => std::cmp::Ordering::Equal,
            };

            match column_index {
                0 => compare,
                _ => compare.reverse()
            }
        });
    }

    // Total 행 추가
    table_rows.push((
        "TOTAL".to_string(),
        total_resources,
    ));

    // 결과 데이터 생성
    let mut result = Vec::new();
    
    // 헤더 추가
    result.push(vec![
        "Namespace".to_string(),
        "CPU Req.".to_string(),
        "CPU Lim.".to_string(),
        "Mem Req.".to_string(),
        "Mem Lim.".to_string(),
    ]);

    // 데이터 행 추가
    for (namespace_name, resources) in table_rows {
        result.push(vec![
            namespace_name,
            format_cpu(resources.cpu_request),
            format_cpu(resources.cpu_limit),
            format_memory(resources.memory_request),
            format_memory(resources.memory_limit),
        ]);
    }

    Ok(result)
}