// src-tauri/src/lib.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use serde_json;
use std::{fs, fs::File, io, path::PathBuf};
use tauri::{command, AppHandle, Manager};
use zip::ZipArchive;

const GAME_PATH_FILE: &str = "game_path.json";
const MODS_DIR: &str = "mods";
const BACKUP_DIR: &str = "mod_backups";

#[derive(serde::Serialize)]
struct ModStatus {
    name: String,
    type_: String,
    author: String,
    version: String,
    applied: bool,
}

fn normalize_mod_path(_mod_name: &str, original_path: &PathBuf) -> PathBuf {
    // 如果路径已经以 "Data" 开头，直接返回
    if original_path.starts_with("Data") {
        return original_path.clone();
    }

    // 如果路径以 "3d" 开头，说明是模型类文件
    if original_path.starts_with("3d") {
        return PathBuf::from("Data").join(original_path);
    }

    // 默认加上 "Data"
    PathBuf::from("Data").join(original_path)
}

/// 简单示例命令
#[command]
fn greet(name: &str) -> String {
    println!("Backend was called with an argument: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 读取游戏路径（存储在 app_config_dir()/game_path.json）
#[command]
fn get_game_path(app: AppHandle) -> Result<Option<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let file_path = config_dir.join(GAME_PATH_FILE);

    if !file_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let path: String = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(path))
}

/// 保存游戏路径到 app_config_dir()/game_path.json
#[command]
fn set_game_path(app: AppHandle, path: String) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let file_path = config_dir.join(GAME_PATH_FILE);
    let serialized = serde_json::to_string(&path).map_err(|e| e.to_string())?;
    fs::write(&file_path, serialized).map_err(|e| e.to_string())
}

/// 启动游戏（仅 Windows）
#[command]
fn launch_game(app: AppHandle) -> Result<(), String> {
    let _game_path = get_game_path(app.clone())?.ok_or_else(|| "游戏路径未设置".to_string())?;
    #[cfg(target_os = "windows")]
    {
        let exe = PathBuf::from(&game_path).join("geek.exe");
        if !exe.exists() {
            return Err("游戏路径无效".into());
        }
        std::process::Command::new(exe)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    Err("仅支持 Windows 平台".into())
}

/// 复制用户选择的 ZIP 到本地 mods 目录
#[command]
fn copy_mod_file(app: AppHandle, src: String) -> Result<(), String> {
    let data_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = data_dir.join(MODS_DIR);
    fs::create_dir_all(&mods_dir).map_err(|e| e.to_string())?;

    let src_path = PathBuf::from(&src);
    let file_name = src_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "无效的文件名".to_string())?;
    let dest = mods_dir.join(file_name);

    if dest.exists() {
        return Err("该 Mod 已存在，请勿重复添加".into());
    }

    fs::copy(&src_path, &dest).map_err(|e| e.to_string())?;
    Ok(())
}

/// 列出本地 mods 目录下的 ZIP 文件名
#[command]
fn list_mods(app: AppHandle) -> Result<Vec<String>, String> {
    let data_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = data_dir.join(MODS_DIR);

    if !mods_dir.exists() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext.eq_ignore_ascii_case("zip"))
        {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                names.push(name.to_string());
            }
        }
    }
    Ok(names)
}

/// 应用选中的 MOD：解压、备份原文件、写入游戏目录
#[command]
fn apply_mods(app: AppHandle, mods: Vec<String>) -> Result<(), String> {
    // 1. 读取游戏路径
    let game_path = get_game_path(app.clone())?.ok_or_else(|| "游戏路径未设置".to_string())?;
    let game_dir = PathBuf::from(game_path);

    // 2. 获取本地数据目录
    // let data_dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    let data_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = data_dir.join(MODS_DIR);
    let backup_root = data_dir.join(BACKUP_DIR);

    // 3. 创建时间戳子目录
    let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
    let backup_dir = backup_root.join(ts);
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;

    // 4. 解压、备份、替换
    for mod_name in mods {
        let zip_path = mods_dir.join(&mod_name);
        let file = File::open(&zip_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let raw_path = entry
                .mangled_name()
                .components()
                .skip_while(|c| matches!(c, std::path::Component::CurDir))
                .collect::<PathBuf>();

            let rel_path = normalize_mod_path(&mod_name, &raw_path);
            let target = game_dir.join(&rel_path);

            println!("目标路径: {:?}", target);

            if entry.is_dir() {
                fs::create_dir_all(&target).map_err(|e| e.to_string())?;
                continue;
            }

            // 备份原文件
            if target.exists() {
                let backup_file = backup_dir.join(&rel_path);
                if let Some(p) = backup_file.parent() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
                fs::copy(&target, &backup_file).map_err(|e| e.to_string())?;
            }

            // 写入替换文件
            if let Some(p) = target.parent() {
                fs::create_dir_all(p).map_err(|e| e.to_string())?;
            }
            let mut out = File::create(&target).map_err(|e| e.to_string())?;
            io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        }
        println!("Mod ZIP 路径: {:?}", zip_path);
    }

    Ok(())
}

#[command]
fn restore_and_delete_mods(app: AppHandle, mods: Vec<String>) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    let backup_dir = config_dir.join(BACKUP_DIR);
    let game_path = get_game_path(app.clone())?.ok_or("游戏路径未设置")?;
    let game_dir = PathBuf::from(game_path);

    let latest_backup = fs::read_dir(&backup_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .max_by_key(|entry| entry.file_name().to_string_lossy().to_string())
        .map(|entry| entry.path())
        .ok_or("未找到备份目录")?;

    for mod_name in mods {
        let zip_path = mods_dir.join(&mod_name);
        let file = File::open(&zip_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

        for i in 0..archive.len() {
            let entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let raw_path = entry
                .mangled_name()
                .components()
                .skip_while(|c| matches!(c, std::path::Component::CurDir))
                .collect::<PathBuf>();

            let rel_path = normalize_mod_path(&mod_name, &raw_path);

            let backup_file = latest_backup.join(&rel_path);
            let game_file = game_dir.join(&rel_path);

            println!("尝试恢复文件: {:?}", game_file);
            if backup_file.is_file() {
                if let Some(p) = game_file.parent() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
                fs::copy(&backup_file, &game_file).map_err(|e| format!("恢复文件失败: {}", e))?;
            } else {
                println!("跳过目录: {:?}", backup_file);
            }
        }

        // 删除 mods 中的 ZIP 文件
        let mod_path = mods_dir.join(&mod_name);
        if mod_path.exists() {
            fs::remove_file(&mod_path).map_err(|e| format!("删除 MOD 文件失败: {}", e))?;
        }
    }

    Ok(())
}

/// 获取当前应用的 MOD 状态：检查每个 ZIP 是否已应用到游戏目录
#[command]
fn get_mod_status(app: AppHandle) -> Result<Vec<ModStatus>, String> {
    let game_path = get_game_path(app.clone())?.ok_or("游戏路径未设置")?;
    let game_dir = PathBuf::from(game_path);
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);

    let mut result = Vec::new();

    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext.eq_ignore_ascii_case("zip"))
        {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "未知MOD".to_string());
            let mut applied = true;

            let file = File::open(&path).map_err(|e| e.to_string())?;
            let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

            for i in 0..archive.len() {
                let entry = archive.by_index(i).map_err(|e| e.to_string())?;
                let raw_path = entry
                    .mangled_name()
                    .components()
                    .skip_while(|c| matches!(c, std::path::Component::CurDir))
                    .collect::<PathBuf>();
                let rel_path = normalize_mod_path(&name, &raw_path);
                let target = game_dir.join(&rel_path);

                if !target.exists() {
                    applied = false;
                    break;
                }
            }

            result.push(ModStatus {
                name,
                applied,
                author: String::new(),     // 或者你可以解析 ZIP 中的 metadata
                type_: "其他".to_string(), // 可根据文件名或路径推断类型
                version: String::new(),    // 暂时为空
            });
        }
    }

    Ok(result)
}

//搜索mod
#[command]
fn search_mods(app: AppHandle, query: String) -> Result<Vec<String>, String> {
    let all_mods = list_mods(app)?;
    let lower_query = query.to_lowercase();
    let filtered: Vec<String> = all_mods
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&lower_query))
        .collect();
    Ok(filtered)
}


/// 程序入口：注册命令并运行 Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                // 打开 DevTools
                let window = tauri::Manager::get_webview_window(app, "main").unwrap();
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
            list_mods,
            apply_mods,
            restore_and_delete_mods,
            get_mod_status,
            search_mods
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
