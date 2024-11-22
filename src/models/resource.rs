use std::collections::BTreeMap;

use crate::util::common::{extract_quantity, parse_cpu, parse_memory};
use k8s_openapi::api::core::v1::ResourceRequirements;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

#[derive(Debug, Clone, Copy, Default)]  // Default trait 추가
pub struct ResourceValue(pub i64);      // field를 pub으로 변경

impl ResourceValue {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn as_millicores(&self) -> i64 {
        self.0
    }

    pub fn as_bytes(&self) -> i64 {
        self.0
    }
}

#[derive(Default, Clone)]
pub struct Resources {
    pub cpu_request: ResourceValue,
    pub cpu_limit: ResourceValue,
    pub memory_request: ResourceValue,
    pub memory_limit: ResourceValue,
}

#[derive(Default, Clone)]
pub struct NodeResources {
    pub base: Resources,                // 기본 리소스 정보
    pub allocatable_cpu: ResourceValue, // 노드에서 사용 가능한 CPU
    pub allocatable_memory: ResourceValue, // 노드에서 사용 가능한 메모리
}

impl Resources {
    pub fn new() -> Self {
        Self {
            cpu_request: ResourceValue::new(0),
            cpu_limit: ResourceValue::new(0),
            memory_request: ResourceValue::new(0),
            memory_limit: ResourceValue::new(0),
        }
    }

    pub fn add_container_resources(&mut self, resources: &ResourceRequirements) {
        let cpu_req = extract_quantity(&resources.requests, "cpu", parse_cpu);
        let cpu_lim = extract_quantity(&resources.limits, "cpu", parse_cpu);
        let mem_req = extract_quantity(&resources.requests, "memory", parse_memory);
        let mem_lim = extract_quantity(&resources.limits, "memory", parse_memory);

        self.cpu_request = ResourceValue::new(self.cpu_request.as_millicores() + cpu_req.as_millicores());
        self.cpu_limit = ResourceValue::new(self.cpu_limit.as_millicores() + cpu_lim.as_millicores());
        self.memory_request = ResourceValue::new(self.memory_request.as_bytes() + mem_req.as_bytes());
        self.memory_limit = ResourceValue::new(self.memory_limit.as_bytes() + mem_lim.as_bytes());
    }

    pub fn add(&mut self, other: &Resources) {
        self.cpu_request = ResourceValue::new(self.cpu_request.as_millicores() + other.cpu_request.as_millicores());
        self.cpu_limit = ResourceValue::new(self.cpu_limit.as_millicores() + other.cpu_limit.as_millicores());
        self.memory_request = ResourceValue::new(self.memory_request.as_bytes() + other.memory_request.as_bytes());
        self.memory_limit = ResourceValue::new(self.memory_limit.as_bytes() + other.memory_limit.as_bytes());
    }
}

impl NodeResources {
    pub fn new() -> Self {
        Self {
            base: Resources::new(),
            allocatable_cpu: ResourceValue::new(0),
            allocatable_memory: ResourceValue::new(0),
        }
    }

    pub fn add_container_resources(&mut self, resources: &ResourceRequirements) {
        self.base.add_container_resources(resources);
    }

    pub fn add_allocatable(&mut self, allocatable: &BTreeMap<String, Quantity>) {
        let allocatable_ref = Some(allocatable.clone());
        self.allocatable_cpu = extract_quantity(&allocatable_ref, "cpu", parse_cpu);
        self.allocatable_memory = extract_quantity(&allocatable_ref, "memory", parse_memory);
    }
}