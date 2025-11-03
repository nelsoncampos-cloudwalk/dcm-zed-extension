use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json;
use zed_extension_api::{EnvVars, Worktree};

use crate::utils::{canonicalize_if_possible, path_to_string, resolve_path};

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct UserSettings {
    pub executable_path: Option<String>,
    pub dart_sdk_path: Option<String>,
    pub show_new_version: bool,
    pub show_unused_code: bool,
    pub show_unused_files: bool,
    pub disable_baseline: bool,
    pub enable_old_formatter: bool,
    pub analyze_only_opened: bool,
    pub excluded_folders: Vec<String>,
    pub log_file_path: Option<String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            executable_path: None,
            dart_sdk_path: None,
            show_new_version: true,
            show_unused_code: false,
            show_unused_files: false,
            disable_baseline: false,
            enable_old_formatter: false,
            analyze_only_opened: false,
            excluded_folders: Vec::new(),
            log_file_path: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedSettings {
    pub user: UserSettings,
    pub executable_path: PathBuf,
    pub sdk_path: Option<PathBuf>,
    pub root_path: PathBuf,
    pub excluded_folders: Vec<PathBuf>,
    pub log_file_path: Option<PathBuf>,
    pub env: EnvVars,
}

impl ResolvedSettings {
    pub fn from_worktree(worktree: &Worktree) -> Result<Self, String> {
        let env = worktree.shell_env();
        let root_path = PathBuf::from(worktree.root_path());
        let user = load_user_settings(worktree)?;

        let executable_path = resolve_executable(worktree, &user, &env, &root_path)?;
        let sdk_path = resolve_optional_path(user.dart_sdk_path.as_deref(), &env, &root_path)?;
        let log_file_path = resolve_optional_path(user.log_file_path.as_deref(), &env, &root_path)?;
        let excluded_folders = resolve_excluded_folders(&user, &env, &root_path)?;

        Ok(Self {
            user,
            executable_path,
            sdk_path,
            root_path,
            excluded_folders,
            log_file_path,
            env,
        })
    }
}

fn load_user_settings(worktree: &Worktree) -> Result<UserSettings, String> {
    let contents = match worktree.read_text_file(".zed/settings.json") {
        Ok(contents) => contents,
        Err(_) => return Ok(UserSettings::default()),
    };

    if contents.trim().is_empty() {
        return Ok(UserSettings::default());
    }

    let root: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|err| format!("Failed to parse .zed/settings.json: {err}"))?;

    match root.get("dcm") {
        Some(value) => serde_json::from_value(value.clone())
            .map_err(|err| format!("Invalid `dcm` settings block: {err}")),
        None => Ok(UserSettings::default()),
    }
}

fn resolve_executable(
    worktree: &Worktree,
    user: &UserSettings,
    env: &EnvVars,
    root_path: &Path,
) -> Result<PathBuf, String> {
    if let Some(raw_path) = user.executable_path.as_deref() {
        let path = canonicalize_if_possible(resolve_path(raw_path, env, root_path)?);
        if path.exists() {
            return Ok(path);
        }
        return Err(format!(
            "Configured DCM executable path does not exist: {}",
            path_to_string(&path)?
        ));
    }

    match worktree.which("dcm") {
        Some(path) => {
            Ok(PathBuf::from(path))
        }
        None => Err("Unable to locate `dcm` executable. Set `dcm.executable_path` in settings or ensure it is available on PATH.".to_string()),
    }
}

fn resolve_optional_path(
    raw: Option<&str>,
    env: &EnvVars,
    root_path: &Path,
) -> Result<Option<PathBuf>, String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let path = canonicalize_if_possible(resolve_path(value, env, root_path)?);
            Ok(Some(path))
        }
        _ => Ok(None),
    }
}

fn resolve_excluded_folders(
    user: &UserSettings,
    env: &EnvVars,
    root_path: &Path,
) -> Result<Vec<PathBuf>, String> {
    let mut folders = Vec::new();
    for folder in &user.excluded_folders {
        if folder.trim().is_empty() {
            continue;
        }
        let resolved = canonicalize_if_possible(resolve_path(folder, env, root_path)?);
        folders.push(resolved);
    }
    Ok(folders)
}
