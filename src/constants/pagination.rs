// Pagination constants
pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MAX_PAGE_SIZE: i64 = 100;
pub const DEFAULT_PAGE: i64 = 1;

/// Normalize pagination parameters
pub fn normalize_pagination(page: Option<i64>, per_page: Option<i64>) -> (i64, i64) {
    let page = page.unwrap_or(DEFAULT_PAGE).max(1);
    let per_page = per_page.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE).max(1);
    (page, per_page)
}

/// Calculate offset for database queries
pub fn calculate_offset(page: i64, per_page: i64) -> i64 {
    (page - 1) * per_page
}
