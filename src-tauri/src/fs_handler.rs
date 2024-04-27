use tokio::fs;

type Result<T> = std::result::Result<T, String>;

macro_rules! ConvertResult {
    ($result: ident) => {
        $result.or_else(|v| return Err(v.to_string()))
    };
}

macro_rules! ConvertFunction {
    ($fn_name:tt, $origin:tt, $r:ty, $($a:ident),*) => {
        #[tauri::command]
        pub async fn $fn_name($($a: String),*) -> Result<$r> {
            let value = fs::$origin($($a),*).await;
            return ConvertResult!(value)
        }
    };
}

ConvertFunction!(create_directory, create_dir_all, (), path);
ConvertFunction!(write_file, write, (), path, data);
ConvertFunction!(delete_directory, remove_dir_all, (), path);
ConvertFunction!(delete_file, remove_file, (), path);
ConvertFunction!(read_file, read_to_string, String, path);
ConvertFunction!(exists, try_exists, bool, path);

/* Not using macro due to different type */
#[tauri::command]
pub async fn write_binary_file(path: String, data: Vec<u8>) -> Result<()> {
    let value = fs::write(&path, &data).await;
    return ConvertResult!(value);
}
