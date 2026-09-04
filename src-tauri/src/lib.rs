// src-tauri/src/lib.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
mod deployment;

use deployment::DeploymentState;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{command, AppHandle, Emitter, Manager};
use zip::ZipArchive;

// ================= 常量定义 =================
const GAME_PATH_FILE: &str = "game_path.json";
const MODS_DIR: &str = "mods";
const BACKUP_DIR: &str = "mod_backups"; // 全局备份目录（纯净镜像）
const MOD_REPO_PATH_FILE: &str = "mod_repo_path.json"; // 新增
const BACKGROUND_IMAGE_FILE: &str = "background_image.json";
const DEPLOYMENT_STATE_FILE: &str = "deployment_state.json";
static DEPLOYMENT_LOCK: Mutex<()> = Mutex::new(());
static MODS_META_LOCK: Mutex<()> = Mutex::new(());

// ================= 数据结构 =================
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
    current: usize,   // 当前处理的 mod 序号
    total: usize,     // 总 mod 数量
    mod_name: String, // 当前正在部署的 mod 名称
    status: String,   // "installing" | "done" | "error"
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
        let serialized = serde_json::to_vec(&path).map_err(|e| e.to_string())?;
        write_atomically(&file_path, &serialized)?;
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

fn write_atomically(path: &Path, content: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("配置文件路径无效")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let token = uuid::Uuid::new_v4();
    let temporary = parent.join(format!(".wotb-write-{token}.tmp"));
    let previous = parent.join(format!(".wotb-write-{token}.bak"));
    fs::write(&temporary, content).map_err(|error| error.to_string())?;

    let had_previous = path.exists();
    if had_previous {
        fs::rename(path, &previous).map_err(|error| error.to_string())?;
    }
    if let Err(error) = fs::rename(&temporary, path) {
        if had_previous {
            let _ = fs::rename(&previous, path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(error.to_string());
    }
    if had_previous {
        let _ = fs::remove_file(previous);
    }
    Ok(())
}

// 写入元数据文件
fn write_mods_meta(app: &AppHandle, map: &HashMap<String, ModMeta>) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let meta_file = config_dir.join("mods_meta.json");
    let content = serde_json::to_vec_pretty(map).map_err(|e| e.to_string())?;
    write_atomically(&meta_file, &content)
}

fn update_mods_meta<T, F>(app: &AppHandle, update: F) -> Result<T, String>
where
    F: FnOnce(&mut HashMap<String, ModMeta>) -> Result<T, String>,
{
    let _guard = MODS_META_LOCK
        .lock()
        .map_err(|_| "Mod 元数据锁已损坏，请重启应用后重试".to_string())?;
    let mut map = read_mods_meta(app)?;
    let result = update(&mut map)?;
    write_mods_meta(app, &map)?;
    Ok(result)
}

fn deployment_state_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?
        .join(DEPLOYMENT_STATE_FILE))
}

fn validate_game_directory(game_dir: &Path) -> Result<(), String> {
    if !game_dir.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir.display()));
    }
    if !game_dir.join("Data").is_dir() {
        return Err("所选目录不是有效的 WOTB 游戏目录：缺少 Data 文件夹".into());
    }
    #[cfg(target_os = "windows")]
    if !game_dir.join("wotblitz.exe").is_file() {
        return Err("所选目录不是有效的 WOTB 游戏目录：缺少 wotblitz.exe".into());
    }
    Ok(())
}

fn legacy_deployment_state(app: &AppHandle) -> Result<DeploymentState, String> {
    let mods_dir = get_mods_dir(app)?;
    let meta = read_mods_meta(app)?;
    let game_dir = get_game_path(app.clone())?.map(PathBuf::from);
    let mut enabled_mods = Vec::new();
    let mut managed_files = HashSet::new();

    for (name, _) in meta.iter().filter(|(_, item)| item.install_date.is_some()) {
        if deployment::validate_mod_name(name).is_err() {
            continue;
        }
        let zip_path = mods_dir.join(name);
        if !zip_path.is_file() {
            continue;
        }
        if let Ok(manifest) = deployment::inspect_archive(&zip_path, name) {
            managed_files.extend(manifest.managed_paths());
        }
        if let Some(game_dir) = &game_dir {
            if deployment::archive_matches_game(game_dir, &zip_path, name).unwrap_or(false) {
                enabled_mods.push(name.clone());
            }
        }
    }

    let mut managed_files = managed_files.into_iter().collect::<Vec<_>>();
    managed_files.sort();
    enabled_mods.sort();
    let has_managed_files = !managed_files.is_empty();
    Ok(DeploymentState {
        game_path: game_dir.map(|path| {
            fs::canonicalize(&path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string()
        }),
        backup_id: has_managed_files.then(|| "legacy".to_string()),
        enabled_mods,
        managed_files,
        managed_hashes: Default::default(),
    })
}

fn read_current_deployment_state(app: &AppHandle) -> Result<DeploymentState, String> {
    match deployment::read_deployment_state(&deployment_state_path(app)?)? {
        Some(state) => Ok(state),
        None => legacy_deployment_state(app),
    }
}

fn deploy_mods_logic_unlocked(
    app: &AppHandle,
    mod_names: &[String],
) -> Result<DeploymentState, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    let mods_dir = get_mods_dir(app)?;
    let game_path = get_game_path(app.clone())?.ok_or("请先设置游戏安装路径")?;
    let game_dir = PathBuf::from(game_path);
    validate_game_directory(&game_dir)?;
    let previous_state = read_current_deployment_state(app)?;
    if !previous_state.managed_files.is_empty() {
        if let Some(previous_game_path) = &previous_state.game_path {
            let previous = fs::canonicalize(previous_game_path)
                .unwrap_or_else(|_| PathBuf::from(previous_game_path));
            let current = fs::canonicalize(&game_dir).unwrap_or_else(|_| game_dir.clone());
            if previous.to_string_lossy().to_lowercase() != current.to_string_lossy().to_lowercase()
            {
                return Err(format!(
                    "部署状态属于另一个游戏目录（{}）。请先切回原目录并停用全部 Mod",
                    previous.display()
                ));
            }
        }
    }
    let backup_id = if previous_state.managed_files.is_empty() {
        deployment::game_backup_id(&game_dir)
    } else {
        previous_state
            .backup_id
            .clone()
            .unwrap_or_else(|| "legacy".into())
    };
    let backup_root = if backup_id == "legacy" {
        config_dir.join(BACKUP_DIR)
    } else {
        config_dir.join(BACKUP_DIR).join(&backup_id)
    };
    let total = mod_names.len();
    let transaction_root = config_dir
        .join("deployment_transactions")
        .join(uuid::Uuid::new_v4().to_string());
    let state_file = deployment_state_path(app)?;

    deployment::reconcile_deployment(
        deployment::DeploymentRequest {
            game_dir: &game_dir,
            mods_dir: &mods_dir,
            backup_root: &backup_root,
            transaction_root: &transaction_root,
            state_file: &state_file,
            backup_id: &backup_id,
            previous_state: &previous_state,
            desired_mods: mod_names,
        },
        |index, name, status| {
            let _ = app.emit(
                "deploy-progress",
                DeployProgress {
                    current: index + 1,
                    total,
                    mod_name: name.to_string(),
                    status: status.to_string(),
                },
            );
        },
    )
}

fn deploy_mods_logic(app: &AppHandle, mod_names: &[String]) -> Result<DeploymentState, String> {
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "部署锁已损坏，请重启应用后重试".to_string())?;
    deploy_mods_logic_unlocked(app, mod_names)
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

fn validate_image_file(path: &Path, allow_gif: bool) -> Result<String, String> {
    if !path.is_file() {
        return Err("所选图片不存在".into());
    }
    let metadata = fs::metadata(path).map_err(|error| error.to_string())?;
    if metadata.len() > 25 * 1024 * 1024 {
        return Err("图片不能超过 25 MB".into());
    }
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_lowercase)
        .ok_or("图片缺少文件扩展名")?;
    let allowed = matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "webp")
        || (allow_gif && extension == "gif");
    if !allowed {
        return Err(if allow_gif {
            "仅支持 PNG、JPG、GIF、WEBP 图片".into()
        } else {
            "仅支持 PNG、JPG、WEBP 图片".into()
        });
    }

    let mut signature = [0u8; 12];
    let read = File::open(path)
        .and_then(|mut file| file.read(&mut signature))
        .map_err(|error| error.to_string())?;
    let valid_signature = match extension.as_str() {
        "png" => read >= 8 && signature[..8] == [137, 80, 78, 71, 13, 10, 26, 10],
        "jpg" | "jpeg" => read >= 3 && signature[..3] == [0xff, 0xd8, 0xff],
        "gif" => read >= 6 && (&signature[..6] == b"GIF87a" || &signature[..6] == b"GIF89a"),
        "webp" => read >= 12 && &signature[..4] == b"RIFF" && &signature[8..12] == b"WEBP",
        _ => false,
    };
    if !valid_signature {
        return Err("图片内容与文件格式不匹配".into());
    }
    Ok(extension)
}

fn remove_managed_image(root: &Path, path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    let root = fs::canonicalize(root).map_err(|error| error.to_string())?;
    let path = fs::canonicalize(path).map_err(|error| error.to_string())?;
    if !path.is_file() || !path.starts_with(&root) {
        return Err("拒绝删除应用图片目录之外的文件".into());
    }
    fs::remove_file(path).map_err(|error| error.to_string())
}

fn is_managed_image(app: &AppHandle, path: &Path) -> Result<bool, String> {
    let path = fs::canonicalize(path).map_err(|error| error.to_string())?;
    for root in [get_icons_dir(app)?, get_backgrounds_dir(app)?] {
        let root = fs::canonicalize(root).map_err(|error| error.to_string())?;
        if path.starts_with(root) {
            return Ok(true);
        }
    }
    Ok(false)
}

// ================= Tauri 命令 (供前端调用) =================

#[command]
async fn rename_mod(
    app: AppHandle,
    original_filename: String,
    new_display_name: String,
) -> Result<(), String> {
    deployment::validate_mod_name(&original_filename)?;
    let display_name = new_display_name.trim().to_string();
    if display_name.is_empty() {
        return Err("Mod 显示名称不能为空".into());
    }
    update_mods_meta(&app, |meta_map| {
        let meta = meta_map
            .get_mut(&original_filename)
            .ok_or_else(|| "找不到该 Mod".to_string())?;
        meta.display_name = display_name;
        Ok(())
    })
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
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
    let trimmed = path.trim();
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let file_path = config_dir.join(GAME_PATH_FILE);
    if trimmed.is_empty() {
        if !read_current_deployment_state(&app)?
            .managed_files
            .is_empty()
        {
            return Err("当前游戏目录仍有 Mod 文件受管理，请先停用全部 Mod，再清除游戏路径".into());
        }
        if file_path.exists() {
            fs::remove_file(file_path).map_err(|error| error.to_string())?;
        }
        return Ok(());
    }
    let game_dir = PathBuf::from(trimmed);
    validate_game_directory(&game_dir)?;
    if let Some(old_path) = get_game_path(app.clone())? {
        let old_dir = fs::canonicalize(&old_path).unwrap_or_else(|_| PathBuf::from(&old_path));
        let new_dir = fs::canonicalize(&game_dir).unwrap_or_else(|_| game_dir.clone());
        if old_dir.to_string_lossy().to_lowercase() != new_dir.to_string_lossy().to_lowercase()
            && !read_current_deployment_state(&app)?
                .managed_files
                .is_empty()
        {
            return Err("当前游戏目录仍有 Mod 文件受管理，请先停用全部 Mod，再更换游戏路径".into());
        }
    }
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let serialized =
        serde_json::to_vec(&game_dir.to_string_lossy().to_string()).map_err(|e| e.to_string())?;
    write_atomically(&file_path, &serialized)
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
fn copy_mod_file_logic(app: &AppHandle, src: &str) -> Result<(), String> {
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
    let mods_dir = get_mods_dir(app)?;
    fs::create_dir_all(&mods_dir).map_err(|e| e.to_string())?;
    let src_path = PathBuf::from(src);
    if !src_path.is_file() {
        return Err("所选 Mod 文件不存在".into());
    }
    let file_name = src_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or("Mod 文件名不是有效文本")?
        .to_string();
    deployment::validate_mod_name(&file_name)?;
    deployment::inspect_archive(&src_path, &file_name)?;
    let dest = mods_dir.join(&file_name);
    let same_file =
        dest.exists() && fs::canonicalize(&src_path).ok() == fs::canonicalize(&dest).ok();
    if dest.exists() && !same_file {
        return Err(format!("同名 Mod「{file_name}」已存在，请先删除或重命名"));
    }
    if !same_file {
        let temporary = mods_dir.join(format!(".import-{}.tmp", uuid::Uuid::new_v4()));
        fs::copy(&src_path, &temporary).map_err(|e| format!("复制 Mod 文件失败: {e}"))?;
        if let Err(error) = deployment::inspect_archive(&temporary, &file_name) {
            let _ = fs::remove_file(&temporary);
            return Err(error);
        }
        if let Err(error) = fs::rename(&temporary, &dest) {
            let _ = fs::remove_file(&temporary);
            return Err(format!("保存 Mod 文件失败: {error}"));
        }
    }

    // 添加元数据
    let original_filename = file_name;
    update_mods_meta(app, |meta_map| {
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
            meta_map.insert(original_filename.clone(), new_meta);
        }
        Ok(())
    })
}

/// 删除 Mod 文件
fn delete_mod_file_logic(app: &AppHandle, mod_name: &str) -> Result<(), String> {
    deployment::validate_mod_name(mod_name)?;
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "部署锁已损坏，请重启应用后重试".to_string())?;
    let mods_dir = get_mods_dir(app)?;
    let zip_path = mods_dir.join(mod_name);
    let state_file_exists = deployment_state_path(app)?.exists();
    let state = read_current_deployment_state(app)?;
    let meta_map = read_mods_meta(app)?;
    let was_legacy_deployed = !state_file_exists
        && meta_map
            .get(mod_name)
            .is_some_and(|meta| meta.install_date.is_some());
    if state.enabled_mods.iter().any(|name| name == mod_name) || was_legacy_deployed {
        let desired = state
            .enabled_mods
            .iter()
            .filter(|name| name.as_str() != mod_name)
            .cloned()
            .collect::<Vec<_>>();
        deploy_mods_logic_unlocked(app, &desired)?;
    }
    if zip_path.exists() {
        fs::remove_file(&zip_path).map_err(|error| format!("删除 Mod 文件失败: {error}"))?;
    }
    let icon_path = update_mods_meta(app, |meta_map| {
        Ok(meta_map.remove(mod_name).and_then(|meta| meta.icon_path))
    })?;
    if let Some(icon_path) = icon_path {
        if let Err(error) = remove_managed_image(&get_icons_dir(app)?, Path::new(&icon_path)) {
            eprintln!("[delete_mod_file] 清理图标失败: {error}");
        }
    }
    Ok(())
}

#[command]
async fn delete_mod_file(app: AppHandle, mod_name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || delete_mod_file_logic(&app, &mod_name))
        .await
        .map_err(|error| format!("删除 Mod 任务异常结束: {error}"))?
}

#[command]
async fn deploy_mods(app: AppHandle, mod_names: Vec<String>) -> Result<(), String> {
    let total = mod_names.len();
    let progress_names = mod_names.clone();
    let worker_app = app.clone();
    let deployment_result = tauri::async_runtime::spawn_blocking(move || {
        let state = deploy_mods_logic(&worker_app, &mod_names)?;
        let enabled = state.enabled_mods.iter().collect::<HashSet<_>>();
        let now = Local::now();
        if let Err(error) = update_mods_meta(&worker_app, |meta_map| {
            for (name, meta) in meta_map {
                if enabled.contains(name) {
                    if meta.install_date.is_none() {
                        meta.install_date = Some(now);
                    }
                } else {
                    meta.install_date = None;
                }
            }
            Ok(())
        }) {
            eprintln!("[deploy_mods] 部署成功，但更新展示元数据失败: {error}");
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|error| format!("部署任务异常结束: {error}"))?;

    if let Err(error) = deployment_result {
        for (index, name) in progress_names.iter().enumerate() {
            let _ = app.emit(
                "deploy-progress",
                DeployProgress {
                    current: index + 1,
                    total,
                    mod_name: name.clone(),
                    status: "error".into(),
                },
            );
        }
        return Err(error);
    }

    let _ = app.emit(
        "deploy-progress",
        DeployProgress {
            current: total,
            total,
            mod_name: String::new(),
            status: "done".to_string(),
        },
    );
    Ok(())
}

#[command]
async fn analyze_mod_conflicts(
    app: AppHandle,
    mod_names: Vec<String>,
) -> Result<Vec<deployment::ModConflict>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = DEPLOYMENT_LOCK
            .lock()
            .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
        let mods_dir = get_mods_dir(&app)?;
        let mut manifests = Vec::new();
        let mut seen = HashSet::new();
        for name in mod_names {
            deployment::validate_mod_name(&name)?;
            if !seen.insert(name.to_lowercase()) {
                continue;
            }
            manifests.push(deployment::inspect_archive(&mods_dir.join(&name), &name)?);
        }
        Ok(deployment::find_conflicts(&manifests))
    })
    .await
    .map_err(|error| format!("冲突分析任务异常结束: {error}"))?
}

fn get_mods_with_status_logic(app: &AppHandle) -> Result<Vec<ModInfo>, String> {
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
    let mods_dir = get_mods_dir(app)?;
    if !mods_dir.exists() {
        return Ok(Vec::new());
    }
    let meta_map = read_mods_meta(app)?;
    let state = read_current_deployment_state(app)?;
    let enabled = state.enabled_mods.into_iter().collect::<HashSet<_>>();
    let mut infos = Vec::new();
    for entry in fs::read_dir(&mods_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry
            .path()
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("zip"))
        {
            if let Some(name) = entry.file_name().to_str() {
                let applied = enabled.contains(name);
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
    infos.sort_by(|first, second| {
        first
            .display_name
            .to_lowercase()
            .cmp(&second.display_name.to_lowercase())
    });
    Ok(infos)
}

#[command]
async fn get_mods_with_status(app: AppHandle) -> Result<Vec<ModInfo>, String> {
    tauri::async_runtime::spawn_blocking(move || get_mods_with_status_logic(&app))
        .await
        .map_err(|error| format!("读取 Mod 列表任务异常结束: {error}"))?
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

/// 使用系统文件管理器打开已配置的 Mod 仓库。
/// 路径只从应用配置读取，前端无法借此打开任意本地路径。
#[command]
fn open_mod_repo(app: AppHandle) -> Result<(), String> {
    let path = get_mod_repo_path(app)?
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "请先配置 Mod 仓库路径".to_string())?;
    let repo_path = PathBuf::from(path);

    if !repo_path.is_dir() {
        return Err("Mod 仓库目录不存在，请重新配置路径".to_string());
    }

    #[cfg(target_os = "windows")]
    let mut command = std::process::Command::new("explorer.exe");
    #[cfg(target_os = "macos")]
    let mut command = std::process::Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = std::process::Command::new("xdg-open");

    command
        .arg(&repo_path)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("无法打开 Mod 仓库: {error}"))
}

/// 设置 Mod 存储库路径
#[command]
fn set_mod_repo_path(app: AppHandle, path: String) -> Result<(), String> {
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let file_path = config_dir.join(MOD_REPO_PATH_FILE);
    let trimmed = path.trim();
    if trimmed.is_empty() {
        if !read_current_deployment_state(&app)?
            .managed_files
            .is_empty()
        {
            return Err("仍有 Mod 文件受管理，请先停用全部 Mod，再清除仓库路径".into());
        }
        if file_path.exists() {
            fs::remove_file(file_path).map_err(|error| error.to_string())?;
        }
        return Ok(());
    }
    let repo_path = PathBuf::from(trimmed);
    fs::create_dir_all(&repo_path).map_err(|error| format!("无法创建 Mod 仓库目录: {error}"))?;
    let state = read_current_deployment_state(&app)?;
    if !state.enabled_mods.is_empty()
        && state
            .enabled_mods
            .iter()
            .any(|name| !repo_path.join(name).is_file())
    {
        return Err("新仓库缺少当前启用的 Mod，请先迁移仓库或停用全部 Mod".into());
    }
    let serialized =
        serde_json::to_vec(&repo_path.to_string_lossy().to_string()).map_err(|e| e.to_string())?;
    write_atomically(&file_path, &serialized)
}

fn migrate_mod_repo_logic(app: &AppHandle, new_path: &str) -> Result<(), String> {
    let _guard = DEPLOYMENT_LOCK
        .lock()
        .map_err(|_| "操作锁已损坏，请重启应用后重试".to_string())?;
    let trimmed = new_path.trim();
    if trimmed.is_empty() {
        return Err("新 Mod 仓库路径不能为空".into());
    }
    let old_mods_dir = get_mods_dir(app)?;
    let new_mods_dir = PathBuf::from(trimmed);
    fs::create_dir_all(&old_mods_dir).map_err(|error| format!("无法读取当前 Mod 仓库: {error}"))?;
    fs::create_dir_all(&new_mods_dir).map_err(|error| format!("无法创建新 Mod 仓库: {error}"))?;
    let old_canonical = fs::canonicalize(&old_mods_dir).map_err(|error| error.to_string())?;
    let new_canonical = fs::canonicalize(&new_mods_dir).map_err(|error| error.to_string())?;
    if old_canonical == new_canonical {
        return Ok(());
    }

    let sources = fs::read_dir(&old_mods_dir)
        .map_err(|error| error.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("zip"))
        })
        .collect::<Vec<_>>();

    for source in &sources {
        let file_name = source.file_name().ok_or("Mod 文件名无效")?;
        let destination = new_mods_dir.join(file_name);
        if destination.exists()
            && deployment::file_hash(source)? != deployment::file_hash(&destination)?
        {
            return Err(format!(
                "新仓库中存在内容不同的同名文件: {}",
                file_name.to_string_lossy()
            ));
        }
    }

    let mut completed: Vec<(PathBuf, PathBuf, bool)> = Vec::new();
    for source in sources {
        let destination = new_mods_dir.join(source.file_name().ok_or("Mod 文件名无效")?);
        let destination_preexisted = destination.exists();
        let move_result = if destination_preexisted {
            fs::remove_file(&source).map_err(|error| error.to_string())
        } else if fs::rename(&source, &destination).is_ok() {
            Ok(())
        } else {
            fs::copy(&source, &destination)
                .map_err(|error| error.to_string())
                .and_then(|_| {
                    if deployment::file_hash(&source)? != deployment::file_hash(&destination)? {
                        return Err("迁移后的文件校验失败".into());
                    }
                    fs::remove_file(&source).map_err(|error| error.to_string())
                })
        };

        if let Err(error) = move_result {
            if !destination_preexisted && destination.exists() && source.exists() {
                let _ = fs::remove_file(&destination);
            }
            for (old_source, new_destination, existed) in completed.into_iter().rev() {
                if existed {
                    let _ = fs::copy(&new_destination, &old_source);
                } else if fs::rename(&new_destination, &old_source).is_err()
                    && fs::copy(&new_destination, &old_source).is_ok()
                {
                    let _ = fs::remove_file(&new_destination);
                }
            }
            return Err(format!("迁移 Mod 仓库失败，已尝试回滚: {error}"));
        }
        completed.push((source, destination, destination_preexisted));
    }

    Ok(())
}

#[command]
async fn migrate_mod_repo(app: AppHandle, new_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || migrate_mod_repo_logic(&app, &new_path))
        .await
        .map_err(|error| format!("迁移 Mod 仓库任务异常结束: {error}"))?
}

// 更新mod类型
#[command]
async fn update_mod_category(
    app: AppHandle,
    original_filename: String,
    category: Option<String>,
) -> Result<(), String> {
    deployment::validate_mod_name(&original_filename)?;
    update_mods_meta(&app, |meta_map| {
        let meta = meta_map
            .get_mut(&original_filename)
            .ok_or_else(|| "找不到该 Mod".to_string())?;
        meta.category = category;
        Ok(())
    })
}

#[command]
async fn copy_mod_file(app: AppHandle, src: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || copy_mod_file_logic(&app, &src))
        .await
        .map_err(|error| format!("导入 Mod 任务异常结束: {error}"))?
}

//设置mod图标
#[command]
async fn set_mod_icon(
    app: AppHandle,
    mod_name: String,
    image_path: String,
) -> Result<String, String> {
    deployment::validate_mod_name(&mod_name)?;
    let icons_dir = get_icons_dir(&app)?;
    let src_path = Path::new(&image_path);
    let ext = validate_image_file(src_path, true)?;
    if !read_mods_meta(&app)?.contains_key(&mod_name) {
        return Err("找不到该 Mod".into());
    }
    let target_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let target_path = icons_dir.join(target_name);

    fs::copy(src_path, &target_path).map_err(|e| format!("复制图标文件失败: {}", e))?;
    let path_str = target_path.to_string_lossy().replace('\\', "/");
    let update_result = update_mods_meta(&app, |meta_map| {
        let meta = meta_map
            .get_mut(&mod_name)
            .ok_or_else(|| "找不到该 Mod".to_string())?;
        let old_path = meta.icon_path.replace(path_str.clone());
        Ok(old_path)
    });
    let old_path = match update_result {
        Ok(path) => path,
        Err(error) => {
            let _ = fs::remove_file(&target_path);
            return Err(error);
        }
    };
    if let Some(old_path) = old_path {
        if let Err(error) = remove_managed_image(&icons_dir, Path::new(&old_path)) {
            eprintln!("[set_mod_icon] 清理旧图标失败: {error}");
        }
    }
    Ok(path_str)
}

//清除mod图标
#[command]
async fn clear_mod_icon(app: AppHandle, mod_name: String) -> Result<(), String> {
    deployment::validate_mod_name(&mod_name)?;
    let icon_path = update_mods_meta(&app, |meta_map| {
        let meta = meta_map
            .get_mut(&mod_name)
            .ok_or_else(|| "找不到该 Mod".to_string())?;
        Ok(meta.icon_path.take())
    })?;
    if let Some(icon_path) = icon_path {
        if let Err(error) = remove_managed_image(&get_icons_dir(&app)?, Path::new(&icon_path)) {
            eprintln!("[clear_mod_icon] 清理图标失败: {error}");
        }
    }
    Ok(())
}

/// 设置自定义背景图片
#[command]
async fn set_background_image(app: AppHandle, image_path: String) -> Result<String, String> {
    let src_path = PathBuf::from(&image_path);
    let ext = validate_image_file(&src_path, false)?;

    let bg_dir = get_backgrounds_dir(&app)?;

    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let dest_path = bg_dir.join(file_name);

    // 复制图片
    fs::copy(&src_path, &dest_path).map_err(|e| format!("复制图片失败: {}", e))?;

    let old_path = get_background_image_path(&app)?;
    let dest_path_str = dest_path.to_string_lossy().replace('\\', "/");
    if let Err(error) = set_background_image_path(&app, Some(dest_path_str.clone())) {
        let _ = fs::remove_file(&dest_path);
        return Err(error);
    }
    if let Some(old_path) = old_path {
        if let Err(error) = remove_managed_image(&bg_dir, Path::new(&old_path)) {
            eprintln!("[set_background_image] 清理旧背景失败: {error}");
        }
    }

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
        set_background_image_path(&app, None)?;
        if let Err(error) = remove_managed_image(&get_backgrounds_dir(&app)?, &path) {
            eprintln!("[remove_background_image] 清理背景文件失败: {error}");
        }
    }
    Ok(())
}

/// 读取图片并返回 base64 编码（解决 Linux asset 协议问题）
#[command]
async fn read_image_base64(app: AppHandle, path: String) -> Result<String, String> {
    let img_path = PathBuf::from(&path);
    validate_image_file(&img_path, true)?;
    if !is_managed_image(&app, &img_path)? {
        return Err("拒绝读取应用图片目录之外的文件".into());
    }
    let mut file = File::open(&img_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(&buffer))
}

/// 比较已启用 Mod 的最终目标路径与内容哈希。
fn detect_deployed_conflicts(app: &AppHandle) -> Result<usize, String> {
    let mods_dir = get_mods_dir(app)?;
    if !mods_dir.exists() {
        return Ok(0);
    }
    let state = read_current_deployment_state(app)?;
    if state.enabled_mods.len() < 2 {
        return Ok(0);
    }
    let mut manifests = Vec::new();
    for mod_name in &state.enabled_mods {
        let zip_path = mods_dir.join(mod_name);
        if !zip_path.exists() {
            continue;
        }
        manifests.push(deployment::inspect_archive(&zip_path, mod_name)?);
    }
    Ok(deployment::find_conflicts(&manifests).len())
}

// ================= 入口函数 =================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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

            tauri::async_runtime::spawn_blocking(move || {
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
                                                e.path().extension().is_some_and(|ext| {
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
                    match detect_deployed_conflicts(&handle) {
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

                // 主窗口显示失败时保留启动页，避免用户只看到黑屏。
                match main_window.show() {
                    Ok(()) => {
                        std::thread::sleep(std::time::Duration::from_millis(80));
                        if let Err(error) = splash.close() {
                            eprintln!("[init] 关闭启动页失败: {error}");
                        }
                        if let Err(error) = main_window.set_focus() {
                            eprintln!("[init] 聚焦主窗口失败: {error}");
                        }
                    }
                    Err(error) => {
                        had_error = true;
                        eprintln!("[init] 显示主窗口失败: {error}");
                        let _ = handle.emit(
                            "init-progress",
                            InitProgress {
                                step: total,
                                total,
                                message: "打开主窗口失败".into(),
                                status: "error".into(),
                                error: Some(error.to_string()),
                            },
                        );
                    }
                }

                // Log any non-fatal errors
                if had_error {
                    eprintln!("[init] 初始化过程中出现错误，但应用已启动。");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_game_path,
            set_game_path,
            launch_game,
            copy_mod_file,
            delete_mod_file,
            deploy_mods,
            analyze_mod_conflicts,
            get_mods_with_status,
            get_mod_repo_path,
            open_mod_repo,
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
