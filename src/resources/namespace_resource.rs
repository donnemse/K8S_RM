use crate::util::common::{extract_quantity, parse_cpu, parse_memory};

use super::resource_value::ResourceValue;

#[derive(Default, Clone)]
pub struct NamespaceResources {
    pub cpu_request: ResourceValue,
    pub cpu_limit: ResourceValue,
    pub memory_request: ResourceValue,
    pub memory_limit: ResourceValue,
}

impl NamespaceResources {
    pub fn new() -> Self {
        Self {
            cpu_request: ResourceValue::new(0),
            cpu_limit: ResourceValue::new(0),
            memory_request: ResourceValue::new(0),
            memory_limit: ResourceValue::new(0),
        }
    }

    pub fn add_container_resources(&mut self, resources: &k8s_openapi::api::core::v1::ResourceRequirements) {
        let cpu_req = extract_quantity(&resources.requests, "cpu", parse_cpu);
        let cpu_lim = extract_quantity(&resources.limits, "cpu", parse_cpu);
        let mem_req = extract_quantity(&resources.requests, "memory", parse_memory);
        let mem_lim = extract_quantity(&resources.limits, "memory", parse_memory);

        self.cpu_request = ResourceValue::new(self.cpu_request.as_millicores() + cpu_req.as_millicores());
        self.cpu_limit = ResourceValue::new(self.cpu_limit.as_millicores() + cpu_lim.as_millicores());
        self.memory_request = ResourceValue::new(self.memory_request.as_bytes() + mem_req.as_bytes());
        self.memory_limit = ResourceValue::new(self.memory_limit.as_bytes() + mem_lim.as_bytes());
    }


    pub fn add(&mut self, other: &NamespaceResources) {
        self.cpu_request = ResourceValue::new(self.cpu_request.as_millicores() + other.cpu_request.as_millicores());
        self.cpu_limit = ResourceValue::new(self.cpu_limit.as_millicores() + other.cpu_limit.as_millicores());
        self.memory_request = ResourceValue::new(self.memory_request.as_bytes() + other.memory_request.as_bytes());
        self.memory_limit = ResourceValue::new(self.memory_limit.as_bytes() + other.memory_limit.as_bytes());
    }
}