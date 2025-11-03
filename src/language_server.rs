use std::path::PathBuf;

use serde_json::json;
use zed_extension_api::{Command, EnvVars};

use crate::config::ResolvedSettings;
use crate::utils::path_to_string;

pub fn build_command(settings: &ResolvedSettings) -> Result<Command, String> {
    let executable = path_to_string(&settings.executable_path)?;
    let mut args = Vec::new();
    args.push("start-server".to_string());

    if let Some(sdk_path) = &settings.sdk_path {
        args.push(format!("--sdk-path={}", path_to_string(sdk_path)?));
    }

    args.push(format!(
        "--root-folder={}",
        path_to_string(&settings.root_path)?
    ));

    if !settings.user.show_new_version {
        args.push("--no-show-new-version-update".to_string());
    }

    if settings.user.show_unused_code {
        args.push("--show-unused-code".to_string());
    }

    if settings.user.analyze_only_opened {
        args.push("--only-opened".to_string());
    }

    if !settings.excluded_folders.is_empty() {
        let joined = join_paths(&settings.excluded_folders)?;
        args.push(format!("--excluded-folders={joined}"));
    }

    if settings.user.show_unused_files {
        args.push("--show-unused-files".to_string());
    }

    if settings.user.disable_baseline {
        args.push("--disable-baseline".to_string());
    }

    if let Some(log_path) = &settings.log_file_path {
        args.push(format!("--logs={}", path_to_string(log_path)?));
    }

    if settings.user.enable_old_formatter {
        args.push("--old-formatter".to_string());
    }

    args.push("--client=zed".to_string());

    let env = build_environment(settings)?;
    let mut command = Command::new(executable);
    command = command.args(args);
    command = command.envs(env);
    Ok(command)
}

pub fn initialization_options(settings: &ResolvedSettings) -> serde_json::Value {
    json!({
        "show_unused_code": settings.user.show_unused_code,
        "show_unused_files": settings.user.show_unused_files,
        "disable_baseline": settings.user.disable_baseline,
        "enable_old_formatter": settings.user.enable_old_formatter,
        "analyze_only_opened": settings.user.analyze_only_opened,
        "log_file_path": settings
            .log_file_path
            .as_ref()
            .and_then(|path| path_to_string(path).ok()),
    })
}

pub fn workspace_configuration(settings: &ResolvedSettings) -> Result<serde_json::Value, String> {
    let mut excluded = Vec::with_capacity(settings.excluded_folders.len());
    for path in &settings.excluded_folders {
        excluded.push(path_to_string(path)?);
    }

    let config = json!({
        "dcm": {
            "showUnusedCode": settings.user.show_unused_code,
            "showUnusedFiles": settings.user.show_unused_files,
            "disableBaseline": settings.user.disable_baseline,
            "enableOldFormatter": settings.user.enable_old_formatter,
            "analyzeOnlyOpened": settings.user.analyze_only_opened,
            "logFilePath": settings
                .log_file_path
                .as_ref()
                .and_then(|path| path_to_string(path).ok()),
            "dartSdkPath": settings
                .sdk_path
                .as_ref()
                .and_then(|path| path_to_string(path).ok()),
            "excludedFolders": excluded,
        }
    });

    Ok(config)
}

fn build_environment(settings: &ResolvedSettings) -> Result<EnvVars, String> {
    let mut env = settings.env.clone();
    upsert_env(&mut env, "PWD", path_to_string(&settings.root_path)?);
    upsert_env(
        &mut env,
        "ZED_WORKTREE_ROOT",
        path_to_string(&settings.root_path)?,
    );

    if let Some(sdk_path) = &settings.sdk_path {
        upsert_env(&mut env, "DART_SDK", path_to_string(sdk_path)?);
    }

    Ok(env)
}

fn upsert_env(env: &mut EnvVars, key: &str, value: String) {
    if let Some(existing) = env.iter_mut().find(|(name, _)| name == key) {
        existing.1 = value;
    } else {
        env.push((key.to_string(), value));
    }
}

fn join_paths(paths: &[PathBuf]) -> Result<String, String> {
    let mut values = Vec::with_capacity(paths.len());
    for path in paths {
        values.push(path_to_string(path)?);
    }
    Ok(values.join(","))
}
