use axum::extract::Path;

pub async fn get_all() -> String {
    String::from("Hello from GET /budgets")
}

pub async fn get_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from GET /budgets/{id}")
}

pub async fn create() -> String {
    String::from("Hello from POST /budgets")
}

pub async fn update_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from PUT /budgets/{id}")
}

pub async fn delete_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from DELETE /budgets/{id}")
}

pub async fn not_found() -> String {
    String::from("NOT FOUND from /budgets")
}
