use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}
