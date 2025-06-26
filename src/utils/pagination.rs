use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn get_limit(&self, default: i64, max: i64) -> i64 {
        self.limit.unwrap_or(default).min(max).max(1)
    }

    pub fn get_offset(&self, limit: i64) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * limit
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, limit: i64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Self {
            data,
            total,
            page,
            limit,
            total_pages,
        }
    }
}
