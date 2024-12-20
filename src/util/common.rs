use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use crate::{models::resource::ResourceValue, AppError};

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
        let float_value = value as f64 / 1000.0;
        if float_value.fract() == 0.0 {
            format!("{}", float_value as i64) // 소수점이 필요 없으면 정수로
        } else {
            format!("{:.1}", float_value) // 소수점이 필요하면 표현
        }
    } else {
        format!("{}m", value)
    }
}

pub fn format_memory(memory: ResourceValue) -> String {
    let bytes = memory.as_bytes();
    if bytes == 0 {
        return String::new();
    }

    let (value, unit) = if bytes >= 1024 * 1024 * 1024 {
        (bytes as f64 / (1024.0 * 1024.0 * 1024.0), "Gi")
    } else if bytes >= 1024 * 1024 {
        (bytes as f64 / (1024.0 * 1024.0), "Mi")
    } else if bytes >= 1024 {
        (bytes as f64 / 1024.0, "Ki")
    } else {
        (bytes as f64, "B")
    };

    if value.fract() == 0.0 {
        format!("{}{}", value as u64, unit) // 정수로 출력
    } else {
        format!("{:.1}{}", value, unit) // 소수점 둘째 자리까지 출력
    }
}

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