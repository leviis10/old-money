use axum::extract::Path;

pub async fn get_all() -> String {
    String::from("Hello from GET /budget-config")
}

pub async fn get_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from GET /budget-config/{id}")
}

pub async fn create() -> String {
    String::from("Hello from POST /budget-config")
}

pub async fn update_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from PUT /budget-config/{id}")
}

pub async fn delete_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from DELETE /budget-config/{id}")
}

pub async fn not_found() -> String {
    String::from("NOT FOUND from /budget-config")
}
