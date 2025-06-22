use axum::extract::Path;

pub async fn get_all() -> String {
    String::from("Hello from GET /transactions")
}

pub async fn get_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from GET /transactions/{id}")
}

pub async fn create() -> String {
    String::from("Hello from POST /transactions")
}

pub async fn update_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from PUT /transactions/{id}")
}

pub async fn delete_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from DELETE /transactions/{id}")
}

pub async fn not_found() -> String {
    String::from("NOT FOUND from /transactions")
}
