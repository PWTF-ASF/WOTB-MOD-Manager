use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;
use zip::ZipArchive;

const MAX_ARCHIVE_FILES: usize = 20_000;
const MAX_SINGLE_FILE_SIZE: u64 = 2 * 1024 * 1024 * 1024;
const MAX_TOTAL_UNCOMPRESSED_SIZE: u64 = 8 * 1024 * 1024 * 1024;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DeploymentState {
    #[serde(default)]
    pub game_path: Option<String>,
    #[serde(default)]
    pub backup_id: Option<String>,
    #[serde(default)]
    pub enabled_mods: Vec<String>,
    #[serde(default)]
    pub managed_files: Vec<String>,
    #[serde(default)]
    pub managed_hashes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct ModConflict {
    pub first_mod: String,
    pub second_mod: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct ArchiveEntry {
    archive_index: usize,
    relative_path: PathBuf,
    key: String,
    hash: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ArchiveManifest {
    name: String,
    entries: Vec<ArchiveEntry>,
}

impl ArchiveManifest {
    pub(crate) fn managed_paths(&self) -> impl Iterator<Item = String> + '_ {
        self.entries
            .iter()
            .map(|entry| serialized_path(&entry.relative_path))
    }
}

#[derive(Debug)]
struct RollbackSnapshot {
    root: PathBuf,
    existing_files: BTreeSet<String>,
}

pub(crate) fn validate_mod_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Mod 文件名不能为空".into());
    }

    let path = Path::new(trimmed);
    let mut components = path.components();
    let is_single_filename =
        matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none();
    if !is_single_filename || path.file_name().and_then(|value| value.to_str()) != Some(trimmed) {
        return Err(format!("无效的 Mod 文件名: {name}"));
    }
    if !path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("zip"))
    {
        return Err(format!("Mod 必须是 ZIP 文件: {name}"));
    }
    Ok(())
}

pub(crate) fn normalize_mod_path(original_path: &Path) -> Result<PathBuf, String> {
    let mut parts = Vec::new();
    for component in original_path.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_string_lossy().to_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!("ZIP 中包含不安全路径: {}", original_path.display()));
            }
        }
    }

    if parts
        .first()
        .is_some_and(|part| part.eq_ignore_ascii_case("wotb"))
    {
        parts.remove(0);
    }
    if parts
        .first()
        .is_some_and(|part| part.eq_ignore_ascii_case("data"))
    {
        parts.remove(0);
    }
    if parts.is_empty() {
        return Err(format!(
            "ZIP 中包含无效文件路径: {}",
            original_path.display()
        ));
    }

    let mut result = PathBuf::from("Data");
    for part in parts {
        result.push(part);
    }
    Ok(result)
}

fn path_key(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/").to_lowercase()
}

fn serialized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn hash_reader<R: Read>(reader: &mut R) -> Result<String, String> {
    let mut hasher = Sha256::new();
    io::copy(reader, &mut hasher).map_err(|error| error.to_string())?;
    Ok(hex::encode(hasher.finalize()))
}

pub(crate) fn file_hash(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|error| error.to_string())?;
    hash_reader(&mut file)
}

pub(crate) fn inspect_archive(zip_path: &Path, mod_name: &str) -> Result<ArchiveManifest, String> {
    validate_mod_name(mod_name)?;
    let file =
        File::open(zip_path).map_err(|error| format!("无法打开 Mod「{mod_name}」: {error}"))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| format!("Mod「{mod_name}」不是有效的 ZIP: {error}"))?;
    if archive.len() > MAX_ARCHIVE_FILES {
        return Err(format!(
            "Mod「{mod_name}」包含过多文件（{}，上限 {MAX_ARCHIVE_FILES}）",
            archive.len()
        ));
    }

    let mut entries_by_key: BTreeMap<String, ArchiveEntry> = BTreeMap::new();
    let mut total_size = 0u64;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("读取 Mod「{mod_name}」失败: {error}"))?;
        if entry.is_dir() {
            continue;
        }
        if entry.size() > MAX_SINGLE_FILE_SIZE {
            return Err(format!("Mod「{mod_name}」中的文件过大: {}", entry.name()));
        }
        total_size = total_size
            .checked_add(entry.size())
            .ok_or_else(|| format!("Mod「{mod_name}」解压大小溢出"))?;
        if total_size > MAX_TOTAL_UNCOMPRESSED_SIZE {
            return Err(format!("Mod「{mod_name}」解压后体积超过安全上限"));
        }

        let relative_path = normalize_mod_path(&entry.mangled_name())?;
        let key = path_key(&relative_path);
        let hash = hash_reader(&mut entry)?;
        let archive_entry = ArchiveEntry {
            archive_index: index,
            relative_path,
            key: key.clone(),
            hash,
        };
        if let Some(previous) = entries_by_key.get(&key) {
            if previous.hash != archive_entry.hash {
                return Err(format!(
                    "Mod「{mod_name}」包含映射到同一路径的不同文件: {}",
                    archive_entry.relative_path.display()
                ));
            }
            continue;
        }
        entries_by_key.insert(key, archive_entry);
    }

    if entries_by_key.is_empty() {
        return Err(format!("Mod「{mod_name}」不包含可部署文件"));
    }

    Ok(ArchiveManifest {
        name: mod_name.to_string(),
        entries: entries_by_key.into_values().collect(),
    })
}

pub(crate) fn find_conflicts(manifests: &[ArchiveManifest]) -> Vec<ModConflict> {
    let maps: Vec<HashMap<&str, &str>> = manifests
        .iter()
        .map(|manifest| {
            manifest
                .entries
                .iter()
                .map(|entry| (entry.key.as_str(), entry.hash.as_str()))
                .collect()
        })
        .collect();
    let mut conflicts = Vec::new();

    for first_index in 0..manifests.len() {
        for second_index in (first_index + 1)..manifests.len() {
            let mut paths = Vec::new();
            for entry in &manifests[first_index].entries {
                if maps[second_index]
                    .get(entry.key.as_str())
                    .is_some_and(|hash| *hash != entry.hash)
                {
                    paths.push(serialized_path(&entry.relative_path));
                }
            }
            if !paths.is_empty() {
                conflicts.push(ModConflict {
                    first_mod: manifests[first_index].name.clone(),
                    second_mod: manifests[second_index].name.clone(),
                    paths,
                });
            }
        }
    }
    conflicts
}

pub(crate) fn format_conflicts(conflicts: &[ModConflict]) -> String {
    let summary = conflicts
        .iter()
        .map(|conflict| {
            format!(
                "「{}」与「{}」冲突 {} 个文件",
                conflict.first_mod,
                conflict.second_mod,
                conflict.paths.len()
            )
        })
        .collect::<Vec<_>>()
        .join("；");
    format!("无法同时部署存在内容冲突的 Mod：{summary}")
}

pub(crate) fn game_backup_id(game_dir: &Path) -> String {
    let stable_path = fs::canonicalize(game_dir).unwrap_or_else(|_| game_dir.to_path_buf());
    let normalized = stable_path
        .to_string_lossy()
        .replace('\\', "/")
        .to_lowercase();
    let digest = Sha256::digest(normalized.as_bytes());
    hex::encode(digest)[..16].to_string()
}

pub(crate) fn read_deployment_state(path: &Path) -> Result<Option<DeploymentState>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path).map_err(|error| format!("无法读取部署状态: {error}"))?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(|error| format!("部署状态文件已损坏: {error}"))
}

fn write_deployment_state(path: &Path, state: &DeploymentState) -> Result<(), String> {
    let parent = path.parent().ok_or("部署状态路径无效")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let token = Uuid::new_v4();
    let temporary = parent.join(format!(".deployment-state-{token}.tmp"));
    let previous = parent.join(format!(".deployment-state-{token}.bak"));
    let content = serde_json::to_vec_pretty(state).map_err(|error| error.to_string())?;
    fs::write(&temporary, content).map_err(|error| format!("无法写入部署状态: {error}"))?;
    let had_previous = path.exists();
    if had_previous {
        fs::rename(path, &previous).map_err(|error| format!("无法暂存旧部署状态: {error}"))?;
    }
    if let Err(error) = fs::rename(&temporary, path) {
        if had_previous {
            let _ = fs::rename(&previous, path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(format!("无法提交部署状态: {error}"));
    }
    if had_previous {
        let _ = fs::remove_file(previous);
    }
    Ok(())
}

fn create_snapshot(
    game_dir: &Path,
    touched: &BTreeMap<String, PathBuf>,
    root: &Path,
) -> Result<RollbackSnapshot, String> {
    fs::create_dir_all(root).map_err(|error| format!("无法创建部署回滚目录: {error}"))?;
    let mut existing_files = BTreeSet::new();
    for (key, relative_path) in touched {
        let source = game_dir.join(relative_path);
        if !source.exists() {
            continue;
        }
        if !source.is_file() {
            return Err(format!("游戏目标路径不是文件: {}", source.display()));
        }
        let destination = root.join(relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(&source, &destination)
            .map_err(|error| format!("创建回滚副本失败（{}）: {error}", source.display()))?;
        existing_files.insert(key.clone());
    }
    Ok(RollbackSnapshot {
        root: root.to_path_buf(),
        existing_files,
    })
}

fn restore_snapshot(
    game_dir: &Path,
    touched: &BTreeMap<String, PathBuf>,
    snapshot: &RollbackSnapshot,
) -> Result<(), String> {
    let mut errors = Vec::new();
    for (key, relative_path) in touched {
        let target = game_dir.join(relative_path);
        if snapshot.existing_files.contains(key) {
            let source = snapshot.root.join(relative_path);
            if let Some(parent) = target.parent() {
                if let Err(error) = fs::create_dir_all(parent) {
                    errors.push(error.to_string());
                    continue;
                }
            }
            if let Err(error) = fs::copy(&source, &target) {
                errors.push(format!("{}: {error}", target.display()));
            }
        } else if target.exists() {
            if let Err(error) = fs::remove_file(&target) {
                errors.push(format!("{}: {error}", target.display()));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("回滚未完全成功: {}", errors.join("；")))
    }
}

fn restore_managed_files(
    game_dir: &Path,
    backup_root: &Path,
    managed_files: &[String],
) -> Result<(), String> {
    for serialized in managed_files {
        let relative_path = normalize_mod_path(Path::new(serialized))?;
        let target = game_dir.join(&relative_path);
        let backup = backup_root.join(&relative_path);
        if backup.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            fs::copy(&backup, &target)
                .map_err(|error| format!("恢复游戏原文件失败（{}）: {error}", target.display()))?;
        } else if target.exists() {
            fs::remove_file(&target).map_err(|error| {
                format!("移除 Mod 新增文件失败（{}）: {error}", target.display())
            })?;
        }
    }
    Ok(())
}

fn backup_new_targets(
    game_dir: &Path,
    backup_root: &Path,
    old_managed: &BTreeSet<String>,
    desired_files: &BTreeMap<String, PathBuf>,
) -> Result<(), String> {
    for (key, relative_path) in desired_files {
        if old_managed.contains(key) {
            continue;
        }
        let source = game_dir.join(relative_path);
        let backup = backup_root.join(relative_path);
        if source.is_file() {
            if let Some(parent) = backup.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            let temporary = backup.with_extension(format!("{}.backup.tmp", Uuid::new_v4()));
            fs::copy(&source, &temporary)
                .map_err(|error| format!("备份游戏原文件失败（{}）: {error}", source.display()))?;
            if backup.exists() {
                fs::remove_file(&backup).map_err(|error| error.to_string())?;
            }
            fs::rename(&temporary, &backup).map_err(|error| error.to_string())?;
        } else if backup.exists() {
            fs::remove_file(&backup).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn refresh_externally_changed_files(
    game_dir: &Path,
    backup_root: &Path,
    previous_state: &DeploymentState,
) -> Result<(), String> {
    for (serialized, deployed_hash) in &previous_state.managed_hashes {
        let relative_path = normalize_mod_path(Path::new(serialized))?;
        let target = game_dir.join(&relative_path);
        let backup = backup_root.join(&relative_path);
        if target.is_file() {
            if file_hash(&target)? == *deployed_hash {
                continue;
            }
            if let Some(parent) = backup.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            let temporary = backup.with_extension(format!("{}.refresh.tmp", Uuid::new_v4()));
            fs::copy(&target, &temporary).map_err(|error| {
                format!("更新游戏原文件备份失败（{}）: {error}", target.display())
            })?;
            if backup.exists() {
                fs::remove_file(&backup).map_err(|error| error.to_string())?;
            }
            fs::rename(&temporary, &backup).map_err(|error| error.to_string())?;
        } else if backup.exists() {
            fs::remove_file(&backup).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn install_manifest(
    game_dir: &Path,
    zip_path: &Path,
    manifest: &ArchiveManifest,
) -> Result<(), String> {
    let file = File::open(zip_path)
        .map_err(|error| format!("无法打开 Mod「{}」: {error}", manifest.name))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| format!("Mod「{}」不是有效的 ZIP: {error}", manifest.name))?;

    for manifest_entry in &manifest.entries {
        let mut entry = archive
            .by_index(manifest_entry.archive_index)
            .map_err(|error| format!("读取 Mod「{}」文件失败: {error}", manifest.name))?;
        let target = game_dir.join(&manifest_entry.relative_path);
        let parent = target
            .parent()
            .ok_or_else(|| format!("无效部署路径: {}", target.display()))?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let temporary = parent.join(format!(".wotb-mod-{}.tmp", Uuid::new_v4()));
        let result = (|| -> Result<(), String> {
            let output = File::create(&temporary).map_err(|error| error.to_string())?;
            let mut output = BufWriter::with_capacity(64 * 1024, output);
            io::copy(&mut entry, &mut output).map_err(|error| error.to_string())?;
            output.flush().map_err(|error| error.to_string())?;
            drop(output);
            if target.exists() {
                fs::remove_file(&target).map_err(|error| error.to_string())?;
            }
            fs::rename(&temporary, &target).map_err(|error| error.to_string())?;
            Ok(())
        })();
        if result.is_err() && temporary.exists() {
            let _ = fs::remove_file(&temporary);
        }
        result.map_err(|error| {
            format!(
                "部署 Mod「{}」中的 {} 失败: {error}",
                manifest.name,
                manifest_entry.relative_path.display()
            )
        })?;
    }
    Ok(())
}

pub(crate) struct DeploymentRequest<'a> {
    pub(crate) game_dir: &'a Path,
    pub(crate) mods_dir: &'a Path,
    pub(crate) backup_root: &'a Path,
    pub(crate) transaction_root: &'a Path,
    pub(crate) state_file: &'a Path,
    pub(crate) backup_id: &'a str,
    pub(crate) previous_state: &'a DeploymentState,
    pub(crate) desired_mods: &'a [String],
}

pub(crate) fn reconcile_deployment<F>(
    request: DeploymentRequest<'_>,
    mut on_mod: F,
) -> Result<DeploymentState, String>
where
    F: FnMut(usize, &str, &str),
{
    let DeploymentRequest {
        game_dir,
        mods_dir,
        backup_root,
        transaction_root,
        state_file,
        backup_id,
        previous_state,
        desired_mods,
    } = request;
    if !game_dir.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir.display()));
    }
    fs::create_dir_all(backup_root).map_err(|error| error.to_string())?;

    let mut seen_names = BTreeSet::new();
    let mut manifests = Vec::new();
    for name in desired_mods {
        validate_mod_name(name)?;
        let name_key = name.to_lowercase();
        if !seen_names.insert(name_key) {
            return Err(format!("部署列表中存在重复 Mod: {name}"));
        }
        manifests.push(inspect_archive(&mods_dir.join(name), name)?);
    }

    let conflicts = find_conflicts(&manifests);
    if !conflicts.is_empty() {
        return Err(format_conflicts(&conflicts));
    }

    let old_managed: BTreeMap<String, PathBuf> = previous_state
        .managed_files
        .iter()
        .map(|value| {
            let path = normalize_mod_path(Path::new(value))?;
            Ok((path_key(&path), path))
        })
        .collect::<Result<_, String>>()?;
    let desired_files: BTreeMap<String, PathBuf> = manifests
        .iter()
        .flat_map(|manifest| manifest.entries.iter())
        .map(|entry| (entry.key.clone(), entry.relative_path.clone()))
        .collect();
    let mut touched = old_managed.clone();
    touched.extend(desired_files.clone());

    let next_state = DeploymentState {
        game_path: Some(
            fs::canonicalize(game_dir)
                .unwrap_or_else(|_| game_dir.to_path_buf())
                .to_string_lossy()
                .to_string(),
        ),
        backup_id: Some(backup_id.to_string()),
        enabled_mods: desired_mods.to_vec(),
        managed_files: desired_files
            .values()
            .map(|path| serialized_path(path))
            .collect(),
        managed_hashes: manifests
            .iter()
            .flat_map(|manifest| manifest.entries.iter())
            .map(|entry| (serialized_path(&entry.relative_path), entry.hash.clone()))
            .collect(),
    };
    let snapshot = create_snapshot(game_dir, &touched, transaction_root)?;
    let operation = (|| -> Result<(), String> {
        let old_keys = old_managed.keys().cloned().collect::<BTreeSet<_>>();
        refresh_externally_changed_files(game_dir, backup_root, previous_state)?;
        backup_new_targets(game_dir, backup_root, &old_keys, &desired_files)?;
        restore_managed_files(game_dir, backup_root, &previous_state.managed_files)?;
        for (index, manifest) in manifests.iter().enumerate() {
            on_mod(index, &manifest.name, "installing");
            if let Err(error) = install_manifest(game_dir, &mods_dir.join(&manifest.name), manifest)
            {
                on_mod(index, &manifest.name, "error");
                return Err(error);
            }
            on_mod(index, &manifest.name, "done");
        }
        write_deployment_state(state_file, &next_state)?;
        Ok(())
    })();

    if let Err(error) = operation {
        let rollback_error = restore_snapshot(game_dir, &touched, &snapshot).err();
        let _ = fs::remove_dir_all(transaction_root);
        return match rollback_error {
            Some(rollback_error) => Err(format!("{error}；{rollback_error}")),
            None => Err(format!("{error}；部署已回滚")),
        };
    }
    let _ = fs::remove_dir_all(transaction_root);

    Ok(next_state)
}

pub(crate) fn archive_matches_game(
    game_dir: &Path,
    zip_path: &Path,
    mod_name: &str,
) -> Result<bool, String> {
    let manifest = inspect_archive(zip_path, mod_name)?;
    for entry_info in manifest.entries {
        let target = game_dir.join(&entry_info.relative_path);
        if !target.is_file() {
            return Ok(false);
        }
        let mut target_file = File::open(&target).map_err(|error| error.to_string())?;
        if hash_reader(&mut target_file)? != entry_info.hash {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("wotb-manager-{label}-{}", Uuid::new_v4()));
            fs::create_dir_all(&path).expect("create temp dir");
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn create_zip(path: &Path, files: &[(&str, &[u8])]) {
        let file = File::create(path).expect("create zip");
        let mut writer = zip::ZipWriter::new(file);
        for (name, content) in files {
            writer
                .start_file(*name, SimpleFileOptions::default())
                .expect("start zip entry");
            writer.write_all(content).expect("write zip entry");
        }
        writer.finish().expect("finish zip");
    }

    #[test]
    fn normalizes_data_prefix_case_insensitively() {
        assert_eq!(
            normalize_mod_path(Path::new("data/3d/Tanks/file.dvpl")).unwrap(),
            PathBuf::from("Data/3d/Tanks/file.dvpl")
        );
        assert_eq!(
            normalize_mod_path(Path::new("WOTB/Data/Gfx/icon.dvpl")).unwrap(),
            PathBuf::from("Data/Gfx/icon.dvpl")
        );
    }

    #[test]
    fn rejects_unsafe_paths_and_names() {
        assert!(normalize_mod_path(Path::new("../outside.txt")).is_err());
        assert!(validate_mod_name("../mod.zip").is_err());
        assert!(validate_mod_name("mod.rar").is_err());
    }

    #[test]
    fn only_different_content_on_same_path_is_a_conflict() {
        let temp = TempDir::new("conflicts");
        create_zip(&temp.0.join("a.zip"), &[("Data/a.txt", b"one")]);
        create_zip(&temp.0.join("b.zip"), &[("data/a.txt", b"one")]);
        create_zip(&temp.0.join("c.zip"), &[("a.txt", b"two")]);
        let manifests = vec![
            inspect_archive(&temp.0.join("a.zip"), "a.zip").unwrap(),
            inspect_archive(&temp.0.join("b.zip"), "b.zip").unwrap(),
            inspect_archive(&temp.0.join("c.zip"), "c.zip").unwrap(),
        ];
        let conflicts = find_conflicts(&manifests);
        assert_eq!(conflicts.len(), 2);
        assert!(conflicts.iter().all(|item| item.second_mod == "c.zip"));
    }

    #[test]
    fn reconcile_restores_vanilla_when_disabling_all() {
        let temp = TempDir::new("disable-all");
        let game = temp.0.join("game");
        let mods = temp.0.join("mods");
        let backup = temp.0.join("backup");
        fs::create_dir_all(game.join("Data")).unwrap();
        fs::create_dir_all(&mods).unwrap();
        fs::write(game.join("Data/a.txt"), b"vanilla").unwrap();
        create_zip(&mods.join("a.zip"), &[("a.txt", b"modded")]);

        let state = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx1"),
                state_file: &temp.0.join("state.json"),
                backup_id: "test",
                previous_state: &DeploymentState::default(),
                desired_mods: &["a.zip".into()],
            },
            |_, _, _| {},
        )
        .unwrap();
        assert_eq!(fs::read(game.join("Data/a.txt")).unwrap(), b"modded");

        let state = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx2"),
                state_file: &temp.0.join("state.json"),
                backup_id: "test",
                previous_state: &state,
                desired_mods: &[],
            },
            |_, _, _| {},
        )
        .unwrap();
        assert!(state.enabled_mods.is_empty());
        assert_eq!(fs::read(game.join("Data/a.txt")).unwrap(), b"vanilla");
    }

    #[test]
    fn reconcile_rejects_conflicts_before_writing_game_files() {
        let temp = TempDir::new("reject-conflict");
        let game = temp.0.join("game");
        let mods = temp.0.join("mods");
        fs::create_dir_all(game.join("Data")).unwrap();
        fs::create_dir_all(&mods).unwrap();
        fs::write(game.join("Data/a.txt"), b"vanilla").unwrap();
        create_zip(&mods.join("a.zip"), &[("a.txt", b"first")]);
        create_zip(&mods.join("b.zip"), &[("Data/a.txt", b"second")]);

        let result = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &temp.0.join("backup"),
                transaction_root: &temp.0.join("tx"),
                state_file: &temp.0.join("state.json"),
                backup_id: "test",
                previous_state: &DeploymentState::default(),
                desired_mods: &["a.zip".into(), "b.zip".into()],
            },
            |_, _, _| {},
        );
        assert!(result.unwrap_err().contains("冲突"));
        assert_eq!(fs::read(game.join("Data/a.txt")).unwrap(), b"vanilla");
    }

    #[test]
    fn switching_conflicting_mods_restores_then_applies_selected_mod() {
        let temp = TempDir::new("switch-mod");
        let game = temp.0.join("game");
        let mods = temp.0.join("mods");
        let backup = temp.0.join("backup");
        let state_file = temp.0.join("state.json");
        fs::create_dir_all(game.join("Data")).unwrap();
        fs::create_dir_all(&mods).unwrap();
        fs::write(game.join("Data/tank.dvpl"), b"vanilla").unwrap();
        create_zip(&mods.join("first.zip"), &[("tank.dvpl", b"first")]);
        create_zip(&mods.join("second.zip"), &[("Data/tank.dvpl", b"second")]);

        let first_state = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx1"),
                state_file: &state_file,
                backup_id: "test",
                previous_state: &DeploymentState::default(),
                desired_mods: &["first.zip".into()],
            },
            |_, _, _| {},
        )
        .unwrap();
        let second_state = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx2"),
                state_file: &state_file,
                backup_id: "test",
                previous_state: &first_state,
                desired_mods: &["second.zip".into()],
            },
            |_, _, _| {},
        )
        .unwrap();

        assert_eq!(second_state.enabled_mods, vec!["second.zip"]);
        assert_eq!(fs::read(game.join("Data/tank.dvpl")).unwrap(), b"second");
        assert_eq!(fs::read(backup.join("Data/tank.dvpl")).unwrap(), b"vanilla");
    }

    #[test]
    fn failed_install_rolls_back_game_files_and_state() {
        let temp = TempDir::new("rollback");
        let game = temp.0.join("game");
        let mods = temp.0.join("mods");
        let backup = temp.0.join("backup");
        let state_file = temp.0.join("state.json");
        fs::create_dir_all(game.join("Data")).unwrap();
        fs::create_dir_all(&mods).unwrap();
        fs::write(game.join("Data/a.txt"), b"vanilla-a").unwrap();
        fs::write(game.join("Data/block"), b"not-a-directory").unwrap();
        create_zip(
            &mods.join("broken.zip"),
            &[
                ("a.txt", b"changed-a"),
                ("block/child.txt", b"cannot-write"),
            ],
        );
        let old_state = DeploymentState::default();

        let result = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx"),
                state_file: &state_file,
                backup_id: "test",
                previous_state: &old_state,
                desired_mods: &["broken.zip".into()],
            },
            |_, _, _| {},
        );

        assert!(result.is_err());
        assert_eq!(fs::read(game.join("Data/a.txt")).unwrap(), b"vanilla-a");
        assert_eq!(
            fs::read(game.join("Data/block")).unwrap(),
            b"not-a-directory"
        );
        assert!(!state_file.exists());
    }

    #[test]
    fn external_game_update_refreshes_the_restore_baseline() {
        let temp = TempDir::new("game-update");
        let game = temp.0.join("game");
        let mods = temp.0.join("mods");
        let backup = temp.0.join("backup");
        let state_file = temp.0.join("state.json");
        fs::create_dir_all(game.join("Data")).unwrap();
        fs::create_dir_all(&mods).unwrap();
        fs::write(game.join("Data/a.txt"), b"old-vanilla").unwrap();
        create_zip(&mods.join("a.zip"), &[("a.txt", b"modded")]);
        let state = reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx1"),
                state_file: &state_file,
                backup_id: "test",
                previous_state: &DeploymentState::default(),
                desired_mods: &["a.zip".into()],
            },
            |_, _, _| {},
        )
        .unwrap();

        fs::write(game.join("Data/a.txt"), b"new-vanilla-from-game-update").unwrap();
        reconcile_deployment(
            DeploymentRequest {
                game_dir: &game,
                mods_dir: &mods,
                backup_root: &backup,
                transaction_root: &temp.0.join("tx2"),
                state_file: &state_file,
                backup_id: "test",
                previous_state: &state,
                desired_mods: &[],
            },
            |_, _, _| {},
        )
        .unwrap();

        assert_eq!(
            fs::read(game.join("Data/a.txt")).unwrap(),
            b"new-vanilla-from-game-update"
        );
    }
}
