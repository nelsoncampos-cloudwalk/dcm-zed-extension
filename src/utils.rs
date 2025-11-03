use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use shellexpand::full_with_context_no_errors;
use zed_extension_api::EnvVars;

/// Resolves a potentially relative or shell-expanded path into an absolute [`PathBuf`].
pub fn resolve_path(raw: &str, env: &EnvVars, worktree_root: &Path) -> Result<PathBuf, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("path is empty".to_string());
    }

    let env_map: HashMap<String, String> = env.iter().cloned().collect();
    let expanded = full_with_context_no_errors(
        trimmed,
        || env_map.get("HOME").cloned(),
        |var| env_map.get(var).cloned(),
    );

    let candidate = PathBuf::from(expanded.as_ref());
    let resolved = if candidate.is_absolute() {
        candidate
    } else {
        worktree_root.join(candidate)
    };

    Ok(resolved)
}

/// Converts a [`Path`] into a UTF-8 [`String`], returning an error if conversion fails.
pub fn path_to_string(path: &Path) -> Result<String, String> {
    Ok(path
        .to_str()
        .map(|s| s.to_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned()))
}

/// Attempts to canonicalize a path, returning the original path if canonicalization fails.
pub fn canonicalize_if_possible(path: PathBuf) -> PathBuf {
    fs::canonicalize(&path).unwrap_or(path)
}
