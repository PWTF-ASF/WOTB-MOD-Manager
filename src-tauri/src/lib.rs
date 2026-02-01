// src-tauri/src/lib.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tauri::{command, AppHandle, Manager};
use zip::ZipArchive;

// ================= 常量定义 =================
const GAME_PATH_FILE: &str = "game_path.json";
const MODS_DIR: &str = "mods";
const BACKUP_DIR: &str = "mod_backups"; // 全局备份目录（纯净镜像）

// ================= 数据结构 =================
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModStatus {
    name: String,
    applied: bool, // true = 正在运行, false = 未启用
    conflicts: Vec<String>, // (可选) 未来可扩展显示与谁冲突
}

// ================= 辅助函数 (Helper Functions) =================

/// 规范化路径：确保所有 Mod 文件都指向 Data 目录
/// 例如： "3d/Tanks/..." -> "Data/3d/Tanks/..."
fn normalize_mod_path(_mod_name: &str, original_path: &Path) -> PathBuf {
    // 如果已经是 Data 开头，保持不变
    if original_path.starts_with("Data") {
        return original_path.to_path_buf();
    }
    // 如果是 3d, Gfx 等常见资源目录，加上 Data 前缀
    if original_path.starts_with("3d") || original_path.starts_with("Gfx") {
        return PathBuf::from("Data").join(original_path);
    }
    // 默认加上 Data，防止解压到根目录弄乱游戏
    PathBuf::from("Data").join(original_path)
}

/// 计算文件的 SHA256 哈希值
fn calculate_hash<R: Read>(reader: &mut R) -> Result<String, String> {
    let mut hasher = Sha256::new();
    io::copy(reader, &mut hasher).map_err(|e| e.to_string())?;
    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

/// 获取 ZIP 包内所有文件的相对路径列表 (用于冲突检测)
fn get_zip_file_list(zip_path: &Path, mod_name: &str) -> Result<HashSet<PathBuf>, String> {
    let file = File::open(zip_path).map_err(|e| format!("无法打开ZIP: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("ZIP解析失败: {}", e))?;
    let mut files = HashSet::new();

    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() { continue; }
        
        // 获取并清洗路径
        let raw_path = entry.mangled_name(); 
        let rel_path = normalize_mod_path(mod_name, &raw_path);
        files.insert(rel_path);
    }
    Ok(files)
}

/// 核心逻辑：安装单个 Mod (带备份功能)
fn install_mod_logic(app: &AppHandle, mod_name: &str) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    let backup_root = config_dir.join(BACKUP_DIR);
    
    let game_path_str = get_game_path(app.clone())?.ok_or("游戏路径未设置")?;
    let game_dir = PathBuf::from(game_path_str);

    // 确保备份目录存在
    if !backup_root.exists() {
        fs::create_dir_all(&backup_root).map_err(|e| e.to_string())?;
    }

    let zip_path = mods_dir.join(mod_name);
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() { continue; }

        let raw_path = entry.mangled_name();
        let rel_path = normalize_mod_path(mod_name, &raw_path);
        let target_file = game_dir.join(&rel_path);

        // === 1. 备份逻辑 (Vanilla Mirror) ===
        // 如果游戏目录里有这个文件，且备份目录里没有，说明这是原厂文件，备份它！
        if target_file.exists() {
            let backup_file = backup_root.join(&rel_path);
            if !backup_file.exists() {
                if let Some(p) = backup_file.parent() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
                println!("[备份] {:?} -> {:?}", target_file, backup_file);
                fs::copy(&target_file, &backup_file).map_err(|e| e.to_string())?;
            }
        }

        // === 2. 覆盖逻辑 ===
        if let Some(p) = target_file.parent() {
            fs::create_dir_all(p).map_err(|e| e.to_string())?;
        }
        let mut out = File::create(&target_file).map_err(|e| e.to_string())?;
        io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 核心逻辑：卸载单个 Mod (从备份恢复)
fn uninstall_mod_logic(app: &AppHandle, mod_name: &str) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    let backup_root = config_dir.join(BACKUP_DIR);
    
    let game_path_str = get_game_path(app.clone())?.ok_or("游戏路径未设置")?;
    let game_dir = PathBuf::from(game_path_str);

    let zip_path = mods_dir.join(mod_name);
    // 即使zip被删了，如果只是为了恢复文件，其实只需要知道它改了哪些文件
    // 但这里为了简单，我们还是假设 zip 存在用于读取文件列表
    if !zip_path.exists() {
        return Ok(()); // 文件都不在了，忽略
    }

    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() { continue; }

        let rel_path = normalize_mod_path(mod_name, &entry.mangled_name());
        let game_file = game_dir.join(&rel_path);
        let backup_file = backup_root.join(&rel_path);

        if backup_file.exists() {
            // 恢复原厂文件
            if let Some(p) = game_file.parent() {
                fs::create_dir_all(p).map_err(|e| e.to_string())?;
            }
            // println!("[恢复] {:?} -> {:?}", backup_file, game_file);
            fs::copy(&backup_file, &game_file).map_err(|e| e.to_string())?;
        } else {
            // 如果没有备份，说明这文件是 Mod 纯新增的，直接删除游戏里的文件
            if game_file.exists() {
                // println!("[删除] 新增文件 {:?}", game_file);
                let _ = fs::remove_file(game_file);
            }
        }
    }
    Ok(())
}

/// 核心逻辑：检查 Mod 是否已应用 (通过 Hash 对比)
fn check_mod_applied(app: &AppHandle, mod_name: &str) -> Result<bool, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    let game_path_str = get_game_path(app.clone())?.ok_or("NoPath")?; // 没路径直接返回错
    let game_dir = PathBuf::from(game_path_str);

    let zip_path = mods_dir.join(mod_name);
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    // 为了性能，我们只检查前 3 个非文件夹文件。
    // 如果前 3 个文件的 Hash 都匹配，我们就认为已安装。
    // (如果想要绝对准确，可以去掉 .take(3)，检查所有文件)
    let mut checked_count = 0;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() { continue; }

        let rel_path = normalize_mod_path(mod_name, &entry.mangled_name());
        let game_file = game_dir.join(&rel_path);

        if !game_file.exists() {
            return Ok(false); // 文件缺失，肯定没装
        }

        // 1. 算 Zip 里文件的 Hash
        let mod_hash = calculate_hash(&mut entry)?;
        // 2. 算 游戏目录文件的 Hash
        let mut game_f = File::open(&game_file).map_err(|e| e.to_string())?;
        let game_hash = calculate_hash(&mut game_f)?;

        if mod_hash != game_hash {
            return Ok(false); // Hash 不匹配，没装
        }

        checked_count += 1;
        if checked_count >= 5 { break; } // 检查5个文件就收手，兼顾性能
    }

    Ok(true)
}


// ================= Tauri 命令 (供前端调用) =================

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Rust 后端运行正常。", name)
}

#[command]
fn get_game_path(app: AppHandle) -> Result<Option<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let file_path = config_dir.join(GAME_PATH_FILE);
    if !file_path.exists() { return Ok(None); }
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let path: String = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(path))
}

#[command]
fn set_game_path(app: AppHandle, path: String) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let file_path = config_dir.join(GAME_PATH_FILE);
    let serialized = serde_json::to_string(&path).map_err(|e| e.to_string())?;
    fs::write(&file_path, serialized).map_err(|e| e.to_string())
}

#[command]
fn launch_game(app: AppHandle) -> Result<(), String> {
    let game_path = get_game_path(app.clone())?.ok_or_else(|| "游戏路径未设置".to_string())?;
    #[cfg(target_os = "windows")]
    {
        let exe = PathBuf::from(&game_path).join("wotblitz.exe"); // 确保这是正确的可执行文件名
        if !exe.exists() { return Err("找不到 wotblitz.exe".into()); }
        std::process::Command::new(exe).spawn().map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    Err("仅支持 Windows 启动".into())
}

/// 将用户选的 ZIP 复制到 Mod 库
#[command]
fn copy_mod_file(app: AppHandle, src: String) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    fs::create_dir_all(&mods_dir).map_err(|e| e.to_string())?;

    let src_path = PathBuf::from(&src);
    let file_name = src_path.file_name().ok_or("无效文件名")?;
    let dest = mods_dir.join(file_name);
    
    fs::copy(&src_path, &dest).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
fn list_mods(app: AppHandle) -> Result<Vec<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    if !mods_dir.exists() { return Ok(Vec::new()); }

    let mut names = Vec::new();
    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().extension().map_or(false, |e| e.eq_ignore_ascii_case("zip")) {
            if let Some(n) = entry.file_name().to_str() {
                names.push(n.to_string());
            }
        }
    }
    Ok(names)
}

/// 智能应用 Mod (Smart Apply)
/// 自动检测冲突，卸载冲突的旧 Mod，安装新 Mod
#[command]
async fn apply_mod_exclusive(app: AppHandle, mod_name: String) -> Result<Vec<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = config_dir.join(MODS_DIR);
    
    // 1. 获取目标 Mod 的文件列表
    let target_zip = mods_dir.join(&mod_name);
    let target_files = get_zip_file_list(&target_zip, &mod_name)?;

    // 2. 遍历所有已存在的 Mod，寻找冲突
    let all_mods = list_mods(app.clone())?;
    let mut conflicts = Vec::new();

    for other_mod in all_mods {
        if other_mod == mod_name { continue; } // 跳过自己

        // 检查 Mod 是否正在运行 (Hash 检查)
        // 注意：这里可能会有点慢，如果Mod很多，可以考虑加缓存
        let is_applied = check_mod_applied(&app, &other_mod).unwrap_or(false);
        
        if is_applied {
            let other_zip = mods_dir.join(&other_mod);
            let other_files = get_zip_file_list(&other_zip, &other_mod)?;

            // 检查是否有交集
            if !target_files.is_disjoint(&other_files) {
                // 发现冲突！
                println!("冲突检测: {} 与 {} 冲突，准备卸载旧 Mod...", mod_name, other_mod);
                conflicts.push(other_mod);
            }
        }
    }

    // 3. 卸载冲突的 Mod
    for conflict_mod in &conflicts {
        uninstall_mod_logic(&app, conflict_mod)?;
    }

    // 4. 安装新 Mod
    install_mod_logic(&app, &mod_name)?;

    // 返回被卸载的 Mod 列表，方便前端更新开关状态
    Ok(conflicts)
}

/// 卸载/恢复 Mod
#[command]
async fn restore_mod(app: AppHandle, mod_name: String) -> Result<(), String> {
    uninstall_mod_logic(&app, &mod_name)
}

/// 删除 Mod 文件 (物理删除 ZIP)
#[command]
async fn delete_mod_file(app: AppHandle, mod_name: String) -> Result<(), String> {
    // 先尝试恢复原文件
    let _ = uninstall_mod_logic(&app, &mod_name); 

    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let zip_path = config_dir.join(MODS_DIR).join(mod_name);
    if zip_path.exists() {
        fs::remove_file(zip_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 获取所有 Mod 的状态 (是否应用)
#[command]
async fn get_mod_status(app: AppHandle) -> Result<Vec<ModStatus>, String> {
    let mod_names = list_mods(app.clone())?;
    let mut statuses = Vec::new();

    for name in mod_names {
        // 使用 Hash 检查
        let applied = check_mod_applied(&app, &name).unwrap_or(false);
        statuses.push(ModStatus {
            name,
            applied,
            conflicts: Vec::new(),
        });
    }

    Ok(statuses)
}

#[command]
async fn deploy_mods(app: AppHandle, mod_names: Vec<String>) -> Result<(), String> {
    // 1. 获取游戏路径
    let game_path_str = get_game_path(app.clone())?.ok_or("请先在‘启动游戏’处设置游戏路径")?;
    
    // 2. (可选) 这里的策略是：在部署新Mod前，可以先根据需要考虑是否恢复纯净备份
    // 为了简单，我们直接执行安装逻辑，install_mod_logic 内部自带备份功能
    
    for name in mod_names {
        println!("正在部署 Mod: {}", name);
        // 调用你现有的 install_mod_logic 辅助函数
        // 该函数会自动：备份原文件 -> 解压覆盖新文件
        install_mod_logic(&app, &name)?;
    }

    Ok(())
}

// ================= 入口函数 =================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_prevent_default::init()) // 根据你的配置可选
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game_path,
            set_game_path,
            launch_game,
            copy_mod_file,
            list_mods,
            apply_mod_exclusive, // 智能安装
            restore_mod,         // 卸载
            delete_mod_file,     // 删除
            get_mod_status,       // 状态检查
            deploy_mods
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}