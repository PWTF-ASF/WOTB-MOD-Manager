// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Backend was called with an argument: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = tauri::Manager::get_webview_window(_app, "main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game_path,
            set_game_path,
            launch_game,
            copy_mod_file,
            list_mods
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use std::{fs, path::PathBuf};
use tauri::{command, Manager, AppHandle};

const GAME_PATH_FILE: &str = "game_path.json";
const MODS_DIR: &str = "mods";

#[command]
fn get_game_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let path = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join(GAME_PATH_FILE);

    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[command]
fn set_game_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let file_path = dir.join(GAME_PATH_FILE);
    std::fs::write(file_path, serde_json::to_string(&path).unwrap()).map_err(|e| e.to_string())
}

#[command]
fn launch_game(app: tauri::AppHandle) -> Result<(), String> {
    let Some(path) = get_game_path(app.clone())? else {
        return Err("游戏路径未设置".into());
    };

    #[cfg(target_os = "windows")]
    {
        let exe = PathBuf::from(path).join("wotblitz.exe");
        if !exe.exists() {
            return Err("游戏路径无效".into());
        }
        std::process::Command::new(exe)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(not(target_os = "windows"))]
    return Err("仅支持 Windows".into());

    Ok(())
}

#[command]
fn copy_mod_file(app: AppHandle, src: String) -> Result<(), String> {
    // 1. 计算 mods 目录
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join(MODS_DIR);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    // 2. 拷贝 ZIP 文件
    let src_path = PathBuf::from(&src);
    let file_name = src_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    let dest = dir.join(file_name);
    fs::copy(&src_path, &dest).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
fn list_mods(app: AppHandle) -> Result<Vec<String>, String> {
    // 1. mods 目录路径
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join(MODS_DIR);

    // 2. 不存在就返回空列表
    if !dir.exists() {
        return Ok(vec![]);
    }

    // 3. 遍历 .zip 文件
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    let mut mods = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|e| e.to_str())
            .map_or(false, |e| e.eq_ignore_ascii_case("zip"))
        {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                mods.push(name.to_string());
            }
        }
    }
    Ok(mods)
}