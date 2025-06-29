use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
}
