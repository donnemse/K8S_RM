#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SortConfig {
    pub column: usize,
}

impl SortConfig {
    pub fn new(column: usize) -> Self {
        Self {
            column,
        }
    }
}