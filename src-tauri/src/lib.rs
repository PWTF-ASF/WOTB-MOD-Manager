// src-tauri/src/lib.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufWriter, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{command, AppHandle, Emitter, Manager};
use zip::ZipArchive;

// ================= 常量定义 =================
const GAME_PATH_FILE: &str = "game_path.json";
const MODS_DIR: &str = "mods";
const BACKUP_DIR: &str = "mod_backups"; // 全局备份目录（纯净镜像）
const MOD_REPO_PATH_FILE: &str = "mod_repo_path.json"; // 新增
const BACKGROUND_IMAGE_FILE: &str = "background_image.json";

// ================= 数据结构 =================
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModStatus {
    name: String,
    applied: bool,          // true = 正在运行, false = 未启用
    conflicts: Vec<String>, // (可选) 未来可扩展显示与谁冲突
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModMeta {
    original_filename: String,             // 原始文件名（如 "mod.zip"）
    display_name: String,                  // 用户自定义显示名称
    install_date: Option<DateTime<Local>>, // 安装日期（部署时记录）
    category: Option<String>,
    icon_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModInfo {
    name: String,         // 原始文件名（用于操作）
    display_name: String, // 显示名称
    applied: bool,
    conflicts: Vec<String>,
    install_date: Option<DateTime<Local>>,
    category: Option<String>,
    icon_path: Option<String>,
}
// ================= 部署进度结构 =================
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeployProgress {
    current: usize,       // 当前处理的 mod 序号
    total: usize,         // 总 mod 数量
    mod_name: String,     // 当前正在部署的 mod 名称
    status: String,       // "installing" | "done" | "error"
}

// ================= 初始化进度结构 =================
#[derive(Serialize, Clone)]
struct InitProgress {
    step: u32,
    total: u32,
    message: String,
    status: String, // "running" | "done" | "error"
    error: Option<String>,
}

// ================= 辅助函数 (Helper Functions) =================

/// 获取背景图片存储目录（在应用数据目录下）
fn get_backgrounds_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let bg_dir = config_dir.join("backgrounds");
    fs::create_dir_all(&bg_dir).map_err(|e| e.to_string())?;
    Ok(bg_dir)
}

/// 读取保存的背景图片路径
fn get_background_image_path(app: &AppHandle) -> Result<Option<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let file_path = config_dir.join(BACKGROUND_IMAGE_FILE);
    if !file_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let path: String = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(path))
}

/// 保存背景图片路径（若为 None 则删除记录）
fn set_background_image_path(app: &AppHandle, path: Option<String>) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let file_path = config_dir.join(BACKGROUND_IMAGE_FILE);
    if let Some(path) = path {
        let serialized = serde_json::to_string(&path).map_err(|e| e.to_string())?;
        fs::write(&file_path, serialized).map_err(|e| e.to_string())?;
    } else {
        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// 自动识别 Mod 类别
fn infer_category(zip_path: &Path) -> Option<String> {
    let file = File::open(zip_path).ok()?;
    let mut archive = ZipArchive::new(file).ok()?;
    let mut has_3d = false;
    let mut has_voice = false;
    let mut has_ui = false;

    for i in 0..archive.len() {
        let entry = archive.by_index(i).ok()?;
        let name = entry.name().to_lowercase();
        if entry.is_dir() {
            continue;
        }

        if name.contains("3d") || name.contains("tank") || name.contains("model") {
            has_3d = true;
        }
        if name.contains("voice") || name.contains("sound") || name.contains("audio") {
            has_voice = true;
        }
        if name.contains("ui") || name.contains("gui") || name.contains("interface") {
            has_ui = true;
        }
        // 可继续扩展其他特征
    }

    if has_3d {
        Some("model".into())
    } else if has_voice {
        Some("voice".into())
    } else if has_ui {
        Some("ui".into())
    } else {
        None
    } // 未知
}

// 读取元数据文件
fn read_mods_meta(app: &AppHandle) -> Result<HashMap<String, ModMeta>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let meta_file = config_dir.join("mods_meta.json");
    if !meta_file.exists() {
        return Ok(HashMap::new());
    }
    let content = fs::read_to_string(meta_file).map_err(|e| e.to_string())?;
    let map: HashMap<String, ModMeta> =
        serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(map)
}

// 写入元数据文件
fn write_mods_meta(app: &AppHandle, map: &HashMap<String, ModMeta>) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let meta_file = config_dir.join("mods_meta.json");
    let content = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    fs::write(meta_file, content).map_err(|e| e.to_string())?;
    Ok(())
}

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
        if entry.is_dir() {
            continue;
        }

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
    let mods_dir = get_mods_dir(app)?;
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
        if entry.is_dir() {
            continue;
        }

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

        // === 2. 覆盖逻辑（使用 64KB 缓冲写入，减少磁盘 I/O 次数） ===
        if let Some(p) = target_file.parent() {
            fs::create_dir_all(p).map_err(|e| e.to_string())?;
        }
        let out = File::create(&target_file).map_err(|e| e.to_string())?;
        let mut out = BufWriter::with_capacity(64 * 1024, out);
        io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 核心逻辑：卸载单个 Mod (从备份恢复)
fn uninstall_mod_logic(app: &AppHandle, mod_name: &str) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mods_dir = get_mods_dir(app)?;
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
        if entry.is_dir() {
            continue;
        }

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
    let mods_dir = get_mods_dir(app)?;
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
        if entry.is_dir() {
            continue;
        }

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
        if checked_count >= 5 {
            break;
        } // 检查5个文件就收手，兼顾性能
    }

    Ok(true)
}

fn get_mods_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;

    // 尝试读取用户设置的 Mod 存储库路径
    if let Some(repo_path) = get_mod_repo_path(app.clone())? {
        let path = PathBuf::from(repo_path);
        // 确保目录存在，防止后续操作失败
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        return Ok(path);
    }

    // 未设置则使用默认的 <config>/mods
    Ok(config_dir.join(MODS_DIR))
}

fn get_icons_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let icons_dir = config_dir.join("icons");
    fs::create_dir_all(&icons_dir).map_err(|e| e.to_string())?;
    Ok(icons_dir)
}

// ================= Tauri 命令 (供前端调用) =================

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Rust 后端运行正常。", name)
}

#[command]
async fn rename_mod(
    app: AppHandle,
    original_filename: String,
    new_display_name: String,
) -> Result<(), String> {
    let mut meta_map = read_mods_meta(&app)?;
    if let Some(meta) = meta_map.get_mut(&original_filename) {
        meta.display_name = new_display_name;
        write_mods_meta(&app, &meta_map)?;
        Ok(())
    } else {
        Err("Mod not found".to_string())
    }
}

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
        if !exe.exists() {
            return Err("找不到 wotblitz.exe".into());
        }
        std::process::Command::new(exe)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    Err("仅支持 Windows 启动".into())
}

/// 将用户选的 ZIP 复制到 Mod 库
#[command]
fn copy_mod_file(app: AppHandle, src: String) -> Result<(), String> {
    let mods_dir = get_mods_dir(&app)?;
    fs::create_dir_all(&mods_dir).map_err(|e| e.to_string())?;
    let src_path = PathBuf::from(&src);
    let file_name = src_path.file_name().ok_or("无效文件名")?;
    let dest = mods_dir.join(file_name);
    fs::copy(&src_path, &dest).map_err(|e| e.to_string())?;

    // 添加元数据
    let mut meta_map = read_mods_meta(&app)?;
    let original_filename = file_name.to_string_lossy().to_string();
    if !meta_map.contains_key(&original_filename) {
        let display_name = Path::new(&original_filename)
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or(&original_filename)
            .to_string();
        let category = infer_category(&dest);
        let new_meta = ModMeta {
            original_filename: original_filename.clone(),
            display_name,
            install_date: None,
            category,
            icon_path: None,
        };
        meta_map.insert(original_filename, new_meta);
        write_mods_meta(&app, &meta_map)?;
    }
    Ok(())
}

#[command]
fn list_mods(app: AppHandle) -> Result<Vec<String>, String> {
    let mods_dir = get_mods_dir(&app)?;
    if !mods_dir.exists() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry
            .path()
            .extension()
            .map_or(false, |e| e.eq_ignore_ascii_case("zip"))
        {
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
    let mods_dir = get_mods_dir(&app)?; // ✅ 存储库目录

    // 1. 获取目标 Mod 的文件列表
    let target_zip = mods_dir.join(&mod_name);
    let target_files = get_zip_file_list(&target_zip, &mod_name)?;

    // 2. 遍历所有已存在的 Mod，寻找冲突
    let all_mods = list_mods(app.clone())?;
    let mut conflicts = Vec::new();

    for other_mod in all_mods {
        if other_mod == mod_name {
            continue;
        } // 跳过自己

        // 检查 Mod 是否正在运行 (Hash 检查)
        // 注意：这里可能会有点慢，如果Mod很多，可以考虑加缓存
        let is_applied = check_mod_applied(&app, &other_mod).unwrap_or(false);

        if is_applied {
            let other_zip = mods_dir.join(&other_mod);
            let other_files = get_zip_file_list(&other_zip, &other_mod)?;

            // 检查是否有交集
            if !target_files.is_disjoint(&other_files) {
                // 发现冲突！
                println!(
                    "冲突检测: {} 与 {} 冲突，准备卸载旧 Mod...",
                    mod_name, other_mod
                );
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

/// 删除 Mod 文件
#[command]
async fn delete_mod_file(app: AppHandle, mod_name: String) -> Result<(), String> {
    // 只在 Mod 确实已部署时才恢复游戏文件，避免误删游戏原厂文件
    let is_deployed = check_mod_applied(&app, &mod_name).unwrap_or(false);
    if is_deployed {
        if let Err(e) = uninstall_mod_logic(&app, &mod_name) {
            eprintln!("[delete_mod_file] 恢复原文件失败（继续删除Mod）: {}", e);
        }
    }
    let mods_dir = get_mods_dir(&app)?;
    let zip_path = mods_dir.join(&mod_name);
    if zip_path.exists() {
        fs::remove_file(zip_path).map_err(|e| e.to_string())?;
    }
    // 删除元数据
    let mut meta_map = read_mods_meta(&app)?;
    // 删除图标文件（如果有）
    if let Some(meta) = meta_map.get(&mod_name) {
        if let Some(icon_path) = &meta.icon_path {
            let icon_path_buf = Path::new(icon_path);
            if icon_path_buf.exists() {
                let _ = fs::remove_file(icon_path_buf);
            }
        }
    }
    meta_map.remove(&mod_name);
    write_mods_meta(&app, &meta_map)?;
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
    let total = mod_names.len();
    if total == 0 {
        return Ok(());
    }

    let now = Local::now();
    let mods_dir = get_mods_dir(&app)?;

    // 批量优化：一次性读取元数据，避免每部署一个 Mod 都读写磁盘
    let mut meta_map = read_mods_meta(&app)?;

    let mut success_count = 0;
    let mut error_messages: Vec<String> = Vec::new();

    for (i, name) in mod_names.iter().enumerate() {
        // 发射安装进度事件
        let _ = app.emit(
            "deploy-progress",
            DeployProgress {
                current: i + 1,
                total,
                mod_name: name.clone(),
                status: "installing".to_string(),
            },
        );

        // 安装 Mod（内部已使用缓冲写入优化磁盘 I/O）
        match install_mod_logic(&app, name) {
            Ok(()) => {
                success_count += 1;
                // 仅操作内存中的元数据
                if let Some(meta) = meta_map.get_mut(name) {
                    if meta.install_date.is_none() {
                        meta.install_date = Some(now);
                    }
                } else {
                    let display_name = Path::new(name)
                        .file_stem()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or(name)
                        .to_string();
                    let category = infer_category(&mods_dir.join(name));
                    let new_meta = ModMeta {
                        original_filename: name.clone(),
                        display_name,
                        install_date: Some(now),
                        category,
                        icon_path: None,
                    };
                    meta_map.insert(name.clone(), new_meta);
                }
                let _ = app.emit(
                    "deploy-progress",
                    DeployProgress {
                        current: i + 1,
                        total,
                        mod_name: name.clone(),
                        status: "done".to_string(),
                    },
                );
            }
            Err(e) => {
                error_messages.push(format!("{}: {}", name, e));
                let _ = app.emit(
                    "deploy-progress",
                    DeployProgress {
                        current: i + 1,
                        total,
                        mod_name: name.clone(),
                        status: "error".to_string(),
                    },
                );
            }
        }
    }

    // 批量优化：所有 Mod 部署完成后，一次性写入元数据
    write_mods_meta(&app, &meta_map)?;

    // 发射完成事件
    let _ = app.emit(
        "deploy-progress",
        DeployProgress {
            current: total,
            total,
            mod_name: String::new(),
            status: "done".to_string(),
        },
    );

    if !error_messages.is_empty() {
        return Err(format!(
            "{} 个成功, {} 个失败: {}",
            success_count,
            error_messages.len(),
            error_messages.join("; ")
        ));
    }

    Ok(())
}

#[command]
async fn get_mods_with_status(app: AppHandle) -> Result<Vec<ModInfo>, String> {
    let mods_dir = get_mods_dir(&app)?;
    if !mods_dir.exists() {
        return Ok(Vec::new());
    }
    let meta_map = read_mods_meta(&app)?;
    let mut infos = Vec::new();
    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry
            .path()
            .extension()
            .map_or(false, |e| e.eq_ignore_ascii_case("zip"))
        {
            if let Some(name) = entry.file_name().to_str() {
                let applied = check_mod_applied(&app, name).unwrap_or(false);
                let meta = meta_map.get(name);
                let display_name = meta.map(|m| m.display_name.clone()).unwrap_or_else(|| {
                    Path::new(name)
                        .file_stem()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or(name)
                        .to_string()
                });
                let install_date = meta.and_then(|m| m.install_date);
                infos.push(ModInfo {
                    name: name.to_string(),
                    display_name,
                    applied,
                    conflicts: Vec::new(),
                    install_date,
                    category: meta.and_then(|m| m.category.clone()),
                    icon_path: meta.and_then(|m| m.icon_path.clone()), // 新增
                });
            }
        }
    }
    Ok(infos)
}

/// 获取 Mod 存储库路径
#[command]
fn get_mod_repo_path(app: AppHandle) -> Result<Option<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let file_path = config_dir.join(MOD_REPO_PATH_FILE);
    if !file_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let path: String = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(Some(path))
}

/// 设置 Mod 存储库路径
#[command]
fn set_mod_repo_path(app: AppHandle, path: String) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let file_path = config_dir.join(MOD_REPO_PATH_FILE);
    let serialized = serde_json::to_string(&path).map_err(|e| e.to_string())?;
    fs::write(&file_path, serialized).map_err(|e| e.to_string())
}

#[command]
async fn migrate_mod_repo(app: AppHandle, new_path: String) -> Result<(), String> {
    let old_mods_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join(MODS_DIR);

    let new_mods_dir = PathBuf::from(&new_path);

    // 如果旧目录存在且有文件，则移动
    if old_mods_dir.exists() {
        for entry in fs::read_dir(&old_mods_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry
                .path()
                .extension()
                .map_or(false, |e| e.eq_ignore_ascii_case("zip"))
            {
                let file_name = entry.file_name();
                let dest = new_mods_dir.join(&file_name);
                // 避免覆盖已有文件
                if !dest.exists() {
                    // try atomic rename first; fall back to copy+remove for cross-volume
                    if fs::rename(entry.path(), &dest).is_err() {
                        fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
                        fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
                    }
                }
            }
        }
        // 如果旧目录变空，可以删除（可选）
        if fs::read_dir(&old_mods_dir)
            .map_err(|e| e.to_string())?
            .next()
            .is_none()
        {
            let _ = fs::remove_dir(old_mods_dir);
        }
    }
    Ok(())
}

// 更新mod类型
#[command]
async fn update_mod_category(
    app: AppHandle,
    original_filename: String,
    category: Option<String>,
) -> Result<(), String> {
    let mut meta_map = read_mods_meta(&app)?;
    if let Some(meta) = meta_map.get_mut(&original_filename) {
        meta.category = category;
        write_mods_meta(&app, &meta_map)?;
        Ok(())
    } else {
        Err("Mod not found".to_string())
    }
}

//设置mod图标
#[command]
async fn set_mod_icon(
    app: AppHandle,
    mod_name: String,
    image_path: String,
) -> Result<String, String> {
    let icons_dir = get_icons_dir(&app)?;
    let src_path = Path::new(&image_path);
    let ext = src_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let target_name = format!("{}.{}", mod_name, ext);
    let target_path = icons_dir.join(target_name);

    // 复制文件
    fs::copy(src_path, &target_path).map_err(|e| format!("复制图标文件失败: {}", e))?;
    println!("图标已复制到: {:?}", target_path);

    // 更新元数据
    let mut meta_map = read_mods_meta(&app)?;
    if let Some(meta) = meta_map.get_mut(&mod_name) {
        // 存储绝对路径，且转换为字符串时确保使用正斜杠（Windows 下也能被 convertFileSrc 处理）
        let path_str = target_path.to_string_lossy().replace('\\', "/");
        meta.icon_path = Some(path_str.clone());
        write_mods_meta(&app, &meta_map)?;
        Ok(path_str)
    } else {
        Err("Mod not found".to_string())
    }
}

//清除mod图标
#[command]
async fn clear_mod_icon(app: AppHandle, mod_name: String) -> Result<(), String> {
    let mut meta_map = read_mods_meta(&app)?;
    if let Some(meta) = meta_map.get_mut(&mod_name) {
        if let Some(icon_path) = meta.icon_path.take() {
            let path = Path::new(&icon_path);
            if path.exists() {
                fs::remove_file(path).map_err(|e| format!("删除图标文件失败: {}", e))?;
                println!("图标文件已删除: {:?}", path);
            }
        }
        write_mods_meta(&app, &meta_map)?;
        Ok(())
    } else {
        Err("Mod not found".to_string())
    }
}

/// 设置自定义背景图片
#[command]
async fn set_background_image(app: AppHandle, image_path: String) -> Result<String, String> {
    let src_path = PathBuf::from(&image_path);
    if !src_path.exists() {
        return Err("源文件不存在".to_string());
    }

    let bg_dir = get_backgrounds_dir(&app)?;

    // 生成唯一文件名（保留原扩展名）
    let ext = src_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let dest_path = bg_dir.join(file_name);

    // 复制图片
    fs::copy(&src_path, &dest_path).map_err(|e| format!("复制图片失败: {}", e))?;

    // 删除旧背景文件（如果有）
    if let Some(old_path) = get_background_image_path(&app)? {
        let old_path = PathBuf::from(&old_path);
        if old_path.exists() {
            let _ = fs::remove_file(&old_path); // 忽略删除失败
        }
    }

    // 存储新路径（确保使用正斜杠，兼容前端 convertFileSrc）
    let dest_path_str = dest_path.to_string_lossy().replace('\\', "/");
    set_background_image_path(&app, Some(dest_path_str.clone()))?;

    Ok(dest_path_str)
}

/// 获取当前背景图片路径
#[command]
async fn get_background_image(app: AppHandle) -> Result<Option<String>, String> {
    get_background_image_path(&app)
}

/// 移除背景图片（删除文件并清空配置）
#[command]
async fn remove_background_image(app: AppHandle) -> Result<(), String> {
    if let Some(path) = get_background_image_path(&app)? {
        let path = PathBuf::from(&path);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("删除背景文件失败: {}", e))?;
        }
        set_background_image_path(&app, None)?;
    }
    Ok(())
}

/// 读取图片并返回 base64 编码（解决 Linux asset 协议问题）
#[command]
async fn read_image_base64(path: String) -> Result<String, String> {
    let img_path = PathBuf::from(&path);
    if !img_path.exists() {
        return Err(format!("图片不存在: {}", path));
    }
    
    let mut file = File::open(&img_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(&buffer))
}

/// 轻量级冲突检测：仅比较已部署 Mod 的文件列表是否有交集（不做 Hash 校验）
fn detect_conflicts_lightweight(app: &AppHandle) -> Result<usize, String> {
    let mods_dir = get_mods_dir(app)?;
    if !mods_dir.exists() {
        return Ok(0);
    }

    let meta_map = read_mods_meta(app)?;

    let deployed_mods: Vec<String> = meta_map
        .iter()
        .filter(|(_, meta)| meta.install_date.is_some())
        .map(|(name, _)| name.clone())
        .collect();

    if deployed_mods.len() < 2 {
        return Ok(0);
    }

    let mut mod_file_lists: Vec<(String, HashSet<PathBuf>)> = Vec::new();
    for mod_name in &deployed_mods {
        let zip_path = mods_dir.join(mod_name);
        if !zip_path.exists() {
            continue;
        }
        if let Ok(files) = get_zip_file_list(&zip_path, mod_name) {
            mod_file_lists.push((mod_name.clone(), files));
        }
    }

    let mut conflict_count = 0usize;
    for i in 0..mod_file_lists.len() {
        for j in (i + 1)..mod_file_lists.len() {
            if !mod_file_lists[i].1.is_disjoint(&mod_file_lists[j].1) {
                conflict_count += 1;
            }
        }
    }

    Ok(conflict_count)
}

// ================= 入口函数 =================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let splash = app
                .get_webview_window("splash")
                .expect("splash window not found");
            let main_window = app
                .get_webview_window("main")
                .expect("main window not found");

            let handle = app.handle().clone();
            let total: u32 = 4;

            // Prevent user from closing splash before init completes
            let init_done = Arc::new(AtomicBool::new(false));
            let init_done_for_event = init_done.clone();
            splash.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    if !init_done_for_event.load(Ordering::Relaxed) {
                        api.prevent_close();
                    }
                }
            });

            let splash_async = splash.clone();
            let main_async = main_window.clone();

            tauri::async_runtime::spawn(async move {
                let splash = splash_async;
                let main_window = main_async;
                let mut had_error = false;

                // ── Step 1: 扫描 Mod 文件目录 ──
                {
                    let _ = handle.emit(
                        "init-progress",
                        InitProgress {
                            step: 1,
                            total,
                            message: "扫描 Mod 文件目录".into(),
                            status: "running".into(),
                            error: None,
                        },
                    );
                    match get_mods_dir(&handle) {
                        Ok(dir) => {
                            let count = if dir.exists() {
                                fs::read_dir(&dir)
                                    .map(|rd| {
                                        rd.filter_map(|e| e.ok())
                                            .filter(|e| {
                                                e.path()
                                                    .extension()
                                                    .map_or(false, |ext| {
                                                        ext.eq_ignore_ascii_case("zip")
                                                    })
                                            })
                                            .count()
                                    })
                                    .unwrap_or(0)
                            } else {
                                0
                            };
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 1,
                                    total,
                                    message: format!("扫描 Mod 文件目录 (找到 {} 个 Mod)", count),
                                    status: "done".into(),
                                    error: None,
                                },
                            );
                        }
                        Err(e) => {
                            had_error = true;
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 1,
                                    total,
                                    message: "扫描 Mod 文件目录 失败".into(),
                                    status: "error".into(),
                                    error: Some(e),
                                },
                            );
                        }
                    }
                }

                // ── Step 2: 读取已安装的 Mod 列表 ──
                {
                    let _ = handle.emit(
                        "init-progress",
                        InitProgress {
                            step: 2,
                            total,
                            message: "读取已安装的 Mod 列表".into(),
                            status: "running".into(),
                            error: None,
                        },
                    );
                    match read_mods_meta(&handle) {
                        Ok(meta_map) => {
                            let deployed_count = meta_map
                                .values()
                                .filter(|m| m.install_date.is_some())
                                .count();
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 2,
                                    total,
                                    message: format!(
                                        "读取已安装的 Mod 列表 ({} 个已部署，{} 个总计)",
                                        deployed_count,
                                        meta_map.len()
                                    ),
                                    status: "done".into(),
                                    error: None,
                                },
                            );
                        }
                        Err(e) => {
                            had_error = true;
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 2,
                                    total,
                                    message: "读取已安装的 Mod 列表 失败".into(),
                                    status: "error".into(),
                                    error: Some(e),
                                },
                            );
                        }
                    }
                }

                // ── Step 3: 检测 Mod 冲突 ──
                {
                    let _ = handle.emit(
                        "init-progress",
                        InitProgress {
                            step: 3,
                            total,
                            message: "检测 Mod 冲突".into(),
                            status: "running".into(),
                            error: None,
                        },
                    );
                    match detect_conflicts_lightweight(&handle) {
                        Ok(conflict_count) => {
                            let msg = if conflict_count == 0 {
                                "检测 Mod 冲突 (未发现冲突)".into()
                            } else {
                                format!("检测 Mod 冲突 (发现 {} 组冲突)", conflict_count)
                            };
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 3,
                                    total,
                                    message: msg,
                                    status: "done".into(),
                                    error: None,
                                },
                            );
                        }
                        Err(e) => {
                            had_error = true;
                            let _ = handle.emit(
                                "init-progress",
                                InitProgress {
                                    step: 3,
                                    total,
                                    message: "检测 Mod 冲突 失败".into(),
                                    status: "error".into(),
                                    error: Some(e),
                                },
                            );
                        }
                    }
                }

                // ── Step 4: 加载用户设置 ──
                {
                    let _ = handle.emit(
                        "init-progress",
                        InitProgress {
                            step: 4,
                            total,
                            message: "加载用户设置".into(),
                            status: "running".into(),
                            error: None,
                        },
                    );
                    let game_path = get_game_path(handle.clone()).ok().flatten();
                    let repo_path = get_mod_repo_path(handle.clone()).ok().flatten();
                    if game_path.is_some() || repo_path.is_some() {
                        let _ = handle.emit(
                            "init-progress",
                            InitProgress {
                                step: 4,
                                total,
                                message: "加载用户设置 完成".into(),
                                status: "done".into(),
                                error: None,
                            },
                        );
                    } else {
                        let _ = handle.emit(
                            "init-progress",
                            InitProgress {
                                step: 4,
                                total,
                                message: "加载用户设置 完成 (首次运行，请配置游戏路径)".into(),
                                status: "done".into(),
                                error: None,
                            },
                        );
                    }
                }

                // ── Final: close splash, show main ──
                // Mark init as done so the close-prevention allows the splash to close
                init_done.store(true, Ordering::Relaxed);

                // Brief pause so user can see all "done" states
                std::thread::sleep(std::time::Duration::from_millis(500));

                // Show main first (renders behind alwaysOnTop splash), then close splash
                // so there's no gap where no window is visible
                let _ = main_window.show();
                std::thread::sleep(std::time::Duration::from_millis(80));
                let _ = splash.close();
                let _ = main_window.set_focus();

                // Log any non-fatal errors
                if had_error {
                    eprintln!("[init] 初始化过程中出现错误，但应用已启动。");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game_path,
            set_game_path,
            launch_game,
            copy_mod_file,
            list_mods,
            apply_mod_exclusive,
            restore_mod,
            delete_mod_file,
            get_mod_status,
            deploy_mods,
            get_mods_with_status,
            get_mod_repo_path,
            set_mod_repo_path,
            migrate_mod_repo,
            rename_mod,
            update_mod_category,
            set_mod_icon,
            clear_mod_icon,
            set_background_image,
            get_background_image,
            remove_background_image,
            read_image_base64,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_mod_path_data_prefix() {
        let path = Path::new("Data/3d/Tanks/some_file.dds");
        let result = normalize_mod_path("test_mod", path);
        assert_eq!(result, PathBuf::from("Data/3d/Tanks/some_file.dds"));
    }

    #[test]
    fn test_normalize_mod_path_adds_data_prefix() {
        let path = Path::new("3d/Tanks/some_file.dds");
        let result = normalize_mod_path("test_mod", path);
        assert_eq!(result, PathBuf::from("Data/3d/Tanks/some_file.dds"));
    }

    #[test]
    fn test_normalize_mod_path_gfx_adds_data_prefix() {
        let path = Path::new("Gfx/UI/icon.dds");
        let result = normalize_mod_path("test_mod", path);
        assert_eq!(result, PathBuf::from("Data/Gfx/UI/icon.dds"));
    }

    #[test]
    fn test_normalize_mod_path_unknown_adds_data_prefix() {
        let path = Path::new("some_random_folder/file.txt");
        let result = normalize_mod_path("test_mod", path);
        assert_eq!(result, PathBuf::from("Data/some_random_folder/file.txt"));
    }

    #[test]
    fn test_normalize_mod_path_already_has_data_prefix_case_insensitive() {
        // Only exact "Data" prefix matches; mixed case gets prefix added
        let path = Path::new("data/3d/Tanks/some_file.dds");
        let result = normalize_mod_path("test_mod", path);
        assert_eq!(result, PathBuf::from("Data/data/3d/Tanks/some_file.dds"));
    }

    #[test]
    fn test_init_progress_serialization() {
        let progress = InitProgress {
            step: 1,
            total: 4,
            message: "test 消息".into(),
            status: "running".into(),
            error: None,
        };
        let json = serde_json::to_string(&progress).expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");
        assert_eq!(parsed["step"], 1);
        assert_eq!(parsed["total"], 4);
        assert_eq!(parsed["message"], "test 消息");
        assert_eq!(parsed["status"], "running");
        assert_eq!(parsed["error"], serde_json::Value::Null);
    }
}
