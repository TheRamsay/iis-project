use serde::Deserialize;

const DEFAULT_OFFSET: i64 = 0;
const DEFAULT_LIMIT: i64 = 10;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}

impl From<PaginationParams> for Pagination {
    fn from(params: PaginationParams) -> Self {
        Self {
            offset: params.offset.map_or(DEFAULT_OFFSET, |v| v),
            limit: params.limit.map_or(DEFAULT_LIMIT, |v| v),
        }
    }
}
