use tokio::fs;

#[tauri::command]
pub async fn create_directory(path: String) -> (bool, Option<String>) {
    match fs::create_dir_all(&path).await {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn write_file(path: String, data: String) -> (bool, Option<String>) {
    match fs::write(&path, &data).await {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn write_binary_file(path: String, data: Vec<u8>) -> (bool, Option<String>) {
    match fs::write(&path, &data).await {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn delete_directory(path: String) -> (bool, Option<String>) {
    match fs::remove_dir_all(&path).await {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn delete_file(path: String) -> (bool, Option<String>) {
    match fs::remove_file(&path).await {
        Ok(_) => (true, None),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn read_file(path: String) -> (bool, Option<String>) {
    match fs::read_to_string(&path).await {
        Ok(data) => (true, Some(data)),
        Err(err) => (false, Some(err.to_string())),
    }
}

#[tauri::command]
pub async fn exists(path: String) -> bool {
    fs::metadata(&path).await.is_ok()
}