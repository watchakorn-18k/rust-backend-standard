use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl<T> PaginationResult<T> {
    pub fn new(data: Vec<T>, page: u64, limit: u64, total: u64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as u64;
        Self {
            data,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
