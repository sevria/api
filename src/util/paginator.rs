use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
}
