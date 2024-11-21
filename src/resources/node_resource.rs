use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

use crate::util::common::{extract_quantity, parse_cpu, parse_memory};
use super::resource_value::ResourceValue;

#[derive(Default, Clone)]
pub struct NodeResources {
    pub allocatable_cpu: ResourceValue,
    pub allocatable_memory: ResourceValue,
    pub cpu_request: ResourceValue,
    pub cpu_limit: ResourceValue,
    pub memory_request: ResourceValue,
    pub memory_limit: ResourceValue,
}

impl NodeResources {
    pub fn new() -> Self {
        Self {
            allocatable_cpu: ResourceValue(0),
            allocatable_memory: ResourceValue(0),
            cpu_request: ResourceValue(0),
            cpu_limit: ResourceValue(0),
            memory_request: ResourceValue(0),
            memory_limit: ResourceValue(0),
        }
    }

    pub fn add_container_resources(&mut self, resources: &k8s_openapi::api::core::v1::ResourceRequirements) {
        let cpu_req = extract_quantity(&resources.requests, "cpu", parse_cpu);
        let mem_req = extract_quantity(&resources.requests, "memory", parse_memory);
        let cpu_lim = extract_quantity(&resources.limits, "cpu", parse_cpu);
        let mem_lim = extract_quantity(&resources.limits, "memory", parse_memory);

        self.cpu_request = ResourceValue(self.cpu_request.0 + cpu_req.0);
        self.memory_request = ResourceValue(self.memory_request.0 + mem_req.0);
        self.cpu_limit = ResourceValue(self.cpu_limit.0 + cpu_lim.0);
        self.memory_limit = ResourceValue(self.memory_limit.0 + mem_lim.0);
    }

    pub fn add_allocatable(&mut self, allocatable: &BTreeMap<String, Quantity>) {
        let allocatable_ref = Some(allocatable.clone());
        self.allocatable_cpu = extract_quantity(&allocatable_ref, "cpu", parse_cpu);
        self.allocatable_memory = extract_quantity(&allocatable_ref, "memory", parse_memory);
    }
}