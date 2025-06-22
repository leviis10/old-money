use axum::extract::Path;

pub async fn get_all() -> String {
    String::from("Hello from GET /users")
}

pub async fn get_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from GET /users/{id}")
}

pub async fn create() -> String {
    String::from("Hello from POST /users")
}

pub async fn update_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from PUT /users/{id}")
}

pub async fn delete_by_id(Path(id): Path<u32>) -> String {
    format!("Hello from DELETE /users/{id}")
}

pub async fn not_found() -> String {
    String::from("NOT FOUND from /users")
}
