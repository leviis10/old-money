use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
}
