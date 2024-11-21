use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use crate::{resources::resource_value::ResourceValue, AppError};

pub fn parse_cpu(quantity: &str) -> Result<ResourceValue, AppError> {
    if quantity.is_empty() {
        return Ok(ResourceValue(0));
    }

    if quantity.ends_with('m') {
        quantity
            .trim_end_matches('m')
            .parse::<i64>()
            .map(ResourceValue)
            .map_err(|e| AppError::ParseError(format!("Failed to parse CPU millicores: {}", e)))
    } else {
        quantity
            .parse::<i64>()
            .map(|v| ResourceValue(v * 1000))
            .map_err(|e| AppError::ParseError(format!("Failed to parse CPU cores: {}", e)))
    }
}

pub fn parse_memory(quantity: &str) -> Result<ResourceValue, AppError> {
    if quantity.is_empty() {
        return Ok(ResourceValue(0));
    }

    let parse_value = |s: &str| {
        s.parse::<i64>()
            .map_err(|e| AppError::ParseError(format!("Failed to parse memory value: {}", e)))
    };

    let result = if quantity.ends_with("Mi") {
        parse_value(quantity.trim_end_matches("Mi")).map(|v| v * 1024 * 1024)
    } else if quantity.ends_with("Gi") {
        parse_value(quantity.trim_end_matches("Gi")).map(|v| v * 1024 * 1024 * 1024)
    } else if quantity.ends_with("Ki") {
        parse_value(quantity.trim_end_matches("Ki")).map(|v| v * 1024)
    } else {
        parse_value(quantity)
    };

    result.map(ResourceValue)
}

pub fn format_cpu(cpu: ResourceValue) -> String {
    let value = cpu.as_millicores();
    if value == 0 {
        String::new()
    } else if value >= 1000 {
        format!("{}", value / 1000)
    } else {
        format!("{}m", value)
    }
}

pub fn format_memory(memory: ResourceValue) -> String {
    let bytes = memory.as_bytes();
    if bytes == 0 {
        String::new()
    } else if bytes >= 1024 * 1024 * 1024 {
        format!("{}Gi", bytes / (1024 * 1024 * 1024))
    } else if bytes >= 1024 * 1024 {
        format!("{}Mi", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{}Ki", bytes / 1024)
    } else {
        format!("{}B", bytes)
    }
}

const BYTE_UNITS: &[(&str, i64)] = &[
    ("Ei", 1024i64.pow(6)),
    ("Pi", 1024i64.pow(5)),
    ("Ti", 1024i64.pow(4)),
    ("Gi", 1024i64.pow(3)),
    ("Mi", 1024i64.pow(2)),
    ("Ki", 1024)
];

pub fn extract_quantity<F>(
    resources: &Option<BTreeMap<String, Quantity>>,
    key: &str,
    parser: F,
) -> ResourceValue 
where
    F: Fn(&str) -> Result<ResourceValue, AppError>,
{
    resources
        .as_ref()
        .and_then(|map| map.get(key))
        .map(|quantity| parser(&quantity.0).unwrap_or(ResourceValue(0)))
        .unwrap_or(ResourceValue(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu() {
        assert_eq!(parse_cpu("1").unwrap().as_millicores(), 1000);
        assert_eq!(parse_cpu("500m").unwrap().as_millicores(), 500);
        assert_eq!(parse_cpu("").unwrap().as_millicores(), 0);
        assert!(parse_cpu("invalid").is_err());
    }

    #[test]
    fn test_parse_memory() {
        assert_eq!(parse_memory("1Gi").unwrap().as_bytes(), 1024 * 1024 * 1024);
        assert_eq!(parse_memory("1Mi").unwrap().as_bytes(), 1024 * 1024);
        assert_eq!(parse_memory("1Ki").unwrap().as_bytes(), 1024);
        assert_eq!(parse_memory("").unwrap().as_bytes(), 0);
        assert!(parse_memory("invalid").is_err());
    }

    #[test]
    fn test_format_cpu() {
        assert_eq!(format_cpu(ResourceValue(1000)), "1");
        assert_eq!(format_cpu(ResourceValue(500)), "500m");
        assert_eq!(format_cpu(ResourceValue(0)), "");
    }

    #[test]
    fn test_format_memory() {
        assert_eq!(format_memory(ResourceValue(1024 * 1024 * 1024)), "1Gi");
        assert_eq!(format_memory(ResourceValue(1024 * 1024)), "1Mi");
        assert_eq!(format_memory(ResourceValue(1024)), "1Ki");
        assert_eq!(format_memory(ResourceValue(0)), "");
    }
}