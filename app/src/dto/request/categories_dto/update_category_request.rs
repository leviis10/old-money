use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
}
