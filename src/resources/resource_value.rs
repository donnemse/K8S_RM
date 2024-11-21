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